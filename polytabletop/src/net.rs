//! Two-human network play over TCP (std only). Protocol: see `proto` and docs/RETEA.md.
//!
//! The server is authoritative: it owns the only `GameObj`, validates every move
//! through `GameObj::play` and renders each STATE with `set_viewer(Some(side))`
//! so a side never receives the other side's private render.

use crate::proto::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Receives (viewer, text sent to viewer, render the *other* side would see).
/// Test hook; `None` in production.
pub type Spy = Box<dyn FnMut(Player, &str, &str) + Send>;

pub struct ServerCfg {
    /// Side of the first connection (the host). `None` = random from the seed.
    pub host_side: Option<Player>,
    pub hello_timeout: Duration,
    /// Time allowed for one move; the mover forfeits on expiry.
    pub turn_timeout: Duration,
    /// How long to wait for the second player (`None` = forever).
    pub wait_timeout: Option<Duration>,
    /// Bad lines tolerated per connection before it is dropped.
    pub max_bad: u32,
    pub spy: Option<Spy>,
}

impl Default for ServerCfg {
    fn default() -> Self {
        ServerCfg {
            host_side: Some(Player::One),
            hello_timeout: Duration::from_secs(10),
            turn_timeout: Duration::from_secs(600),
            wait_timeout: None,
            max_bad: 30,
            spy: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameResult {
    /// `None` = game aborted (disconnect, timeout before start...).
    pub outcome: Option<Outcome>,
    pub reason: String,
}

enum Ev {
    Joined(TcpStream, BufReader<TcpStream>),
    Line(usize, String),
    Bad(usize, &'static str),
    Eof,
}

struct Seat {
    w: TcpStream,
    side: Player,
    bad: u32,
}

fn num(p: Player) -> u8 {
    p.idx() as u8 + 1
}

fn send(w: &mut TcpStream, m: &ServerMsg) {
    let _ = write_server(w, m);
}

fn handshake(s: TcpStream, to: Duration, tx: &mpsc::Sender<Ev>) {
    let _ = s.set_read_timeout(Some(to));
    let _ = s.set_write_timeout(Some(Duration::from_secs(5)));
    let _ = s.set_nodelay(true);
    let Ok(rs) = s.try_clone() else { return };
    let mut r = BufReader::new(rs);
    let mut w = s;
    match read_line_capped(&mut r, MAX_LINE) {
        Ok(Line::Text(t)) => match parse_client(&t) {
            Ok(ClientMsg::Hello(v)) if v.split_whitespace().next() == Some(VERSION) => {
                send(&mut w, &ServerMsg::Hello);
                let _ = w.set_read_timeout(None);
                let _ = tx.send(Ev::Joined(w, r));
                return;
            }
            Ok(ClientMsg::Hello(_)) => send(
                &mut w,
                &ServerMsg::Err(format!("versiune neacceptată; folosiți {VERSION}")),
            ),
            _ => send(&mut w, &ServerMsg::Err("se aștepta HELLO".into())),
        },
        _ => send(&mut w, &ServerMsg::Err("HELLO invalid sau expirat".into())),
    }
    let _ = w.shutdown(std::net::Shutdown::Both);
}

fn reader_thread(i: usize, mut r: BufReader<TcpStream>, tx: mpsc::Sender<Ev>) {
    loop {
        let ev = match read_line_capped(&mut r, MAX_LINE) {
            Ok(Line::Text(t)) => Ev::Line(i, t),
            Ok(Line::TooLong) => Ev::Bad(i, "linie prea lungă"),
            Ok(Line::BadUtf8) => Ev::Bad(i, "UTF-8 invalid"),
            Ok(Line::Eof) | Err(_) => {
                let _ = tx.send(Ev::Eof);
                return;
            }
        };
        if tx.send(ev).is_err() {
            return;
        }
    }
}

fn state_text(game: &mut dyn GameObj, p: Player) -> String {
    game.set_viewer(Some(p));
    game.render()
}

fn send_states(game: &mut dyn GameObj, seats: &mut [Seat], spy: &mut Option<Spy>) {
    let turn = if game.outcome().is_some() {
        0
    } else {
        num(game.to_move())
    };
    for s in seats.iter_mut() {
        let text = state_text(game, s.side);
        if let Some(f) = spy {
            let other = state_text(game, s.side.other());
            f(s.side, &text, &other);
        }
        let lines = text
            .lines()
            .take(MAX_STATE_LINES)
            .map(String::from)
            .collect();
        send(&mut s.w, &ServerMsg::State { turn, lines });
    }
}

fn outcome_word(o: Outcome) -> &'static str {
    match o {
        Outcome::Win(Player::One) => "win1",
        Outcome::Win(Player::Two) => "win2",
        Outcome::Draw => "draw",
    }
}

fn finish(seats: &mut [Seat], outcome: Option<Outcome>, reason: &str) -> GameResult {
    let word = outcome.map(outcome_word).unwrap_or("aborted");
    for s in seats.iter_mut() {
        send(
            &mut s.w,
            &ServerMsg::End {
                outcome: word.into(),
                reason: reason.into(),
            },
        );
        let _ = s.w.shutdown(std::net::Shutdown::Both);
    }
    GameResult {
        outcome,
        reason: reason.into(),
    }
}

/// Runs one game on `listener`: accepts two players, plays, returns the result.
/// The first player to connect is the host.
pub fn serve(
    listener: TcpListener,
    id: &str,
    seed: u64,
    mut cfg: ServerCfg,
) -> io::Result<GameResult> {
    let p = polytabletop_games::find(id)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("joc necunoscut: {id}")))?;
    let mut game = (p.make)();
    let host_side = cfg
        .host_side
        .unwrap_or(if Rng::new(seed ^ 0x51DE).next_u64() & 1 == 0 {
            Player::One
        } else {
            Player::Two
        });
    let (tx, rx) = mpsc::channel::<Ev>();
    let stop = Arc::new(AtomicBool::new(false));
    listener.set_nonblocking(true)?;
    {
        let (tx, stop, to) = (tx.clone(), stop.clone(), cfg.hello_timeout);
        std::thread::spawn(move || {
            while !stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((s, _)) => {
                        let _ = s.set_nonblocking(false);
                        let tx = tx.clone();
                        std::thread::spawn(move || handshake(s, to, &tx));
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(20))
                    }
                    Err(_) => std::thread::sleep(Duration::from_millis(100)),
                }
            }
        });
    }
    let r = run(&mut *game, id, seed, host_side, &mut cfg, &rx, &tx);
    stop.store(true, Ordering::SeqCst);
    Ok(r)
}

fn run(
    game: &mut dyn GameObj,
    id: &str,
    seed: u64,
    host_side: Player,
    cfg: &mut ServerCfg,
    rx: &mpsc::Receiver<Ev>,
    tx: &mpsc::Sender<Ev>,
) -> GameResult {
    let mut seats: Vec<Seat> = Vec::new();
    let wait_until = cfg.wait_timeout.map(|d| Instant::now() + d);
    // Phase 1: wait for two players.
    while seats.len() < 2 {
        let t = match wait_until {
            Some(u) => u.saturating_duration_since(Instant::now()),
            None => Duration::from_secs(3600),
        };
        match rx.recv_timeout(t) {
            Ok(Ev::Joined(w, r)) => {
                let i = seats.len();
                let side = if i == 0 { host_side } else { host_side.other() };
                let tx = tx.clone();
                std::thread::spawn(move || reader_thread(i, r, tx));
                seats.push(Seat { w, side, bad: 0 });
            }
            Ok(Ev::Line(i, l)) => {
                let m = match parse_client(&l) {
                    Ok(ClientMsg::Quit) => {
                        return finish(&mut seats, None, "jucătorul a plecat înainte de start");
                    }
                    Ok(ClientMsg::Chat(_)) => "încă se așteaptă adversarul".to_string(),
                    _ => "jocul nu a început".to_string(),
                };
                send(&mut seats[i].w, &ServerMsg::Err(m));
            }
            Ok(Ev::Bad(i, why)) => send(&mut seats[i].w, &ServerMsg::Err(why.into())),
            Ok(Ev::Eof) => return finish(&mut seats, None, "jucătorul a plecat înainte de start"),
            Err(RecvTimeoutError::Timeout) => {
                if wait_until.is_some() {
                    return finish(&mut seats, None, "nimeni nu s-a alăturat la timp");
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                return finish(&mut seats, None, "eroare internă")
            }
        }
    }
    for s in seats.iter_mut() {
        send(
            &mut s.w,
            &ServerMsg::Start {
                id: id.into(),
                seed,
                side: num(s.side),
            },
        );
    }
    send_states(game, &mut seats, &mut cfg.spy);
    if let Some(o) = game.outcome() {
        return finish(&mut seats, Some(o), "terminat");
    }
    let mut deadline = Instant::now() + cfg.turn_timeout;
    // Phase 2: play.
    loop {
        let ev = match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok(e) => e,
            Err(RecvTimeoutError::Timeout) => {
                let loser = game.to_move();
                return finish(
                    &mut seats,
                    Some(Outcome::Win(loser.other())),
                    "timeout: mutarea nu a venit la timp",
                );
            }
            Err(RecvTimeoutError::Disconnected) => {
                return finish(&mut seats, None, "eroare internă")
            }
        };
        let (i, line) = match ev {
            Ev::Joined(mut w, _) => {
                send(&mut w, &ServerMsg::Err("jocul este plin".into()));
                let _ = w.shutdown(std::net::Shutdown::Both);
                continue;
            }
            Ev::Eof => return finish(&mut seats, None, "adversarul s-a deconectat"),
            Ev::Bad(i, why) => {
                if strike(&mut seats[i], cfg.max_bad, why) {
                    return finish(
                        &mut seats,
                        None,
                        "adversarul a fost exclus (prea multe erori)",
                    );
                }
                continue;
            }
            Ev::Line(i, l) => (i, l),
        };
        let me = seats[i].side;
        match parse_client(&line) {
            Err(e) => {
                if strike(&mut seats[i], cfg.max_bad, &e) {
                    return finish(
                        &mut seats,
                        None,
                        "adversarul a fost exclus (prea multe erori)",
                    );
                }
            }
            Ok(ClientMsg::Hello(_)) => send(
                &mut seats[i].w,
                &ServerMsg::Err("HELLO a fost deja trimis".into()),
            ),
            Ok(ClientMsg::Quit) => return finish(&mut seats, None, "adversarul a plecat"),
            Ok(ClientMsg::Chat(t)) => {
                let o = 1 - i;
                send(
                    &mut seats[o].w,
                    &ServerMsg::Chat {
                        side: num(me),
                        text: t,
                    },
                );
            }
            Ok(ClientMsg::State) => {
                let turn = num(game.to_move());
                let lines = state_text(game, me)
                    .lines()
                    .take(MAX_STATE_LINES)
                    .map(String::from)
                    .collect();
                send(&mut seats[i].w, &ServerMsg::State { turn, lines });
            }
            Ok(ClientMsg::Legal) => {
                if game.to_move() != me {
                    send(
                        &mut seats[i].w,
                        &ServerMsg::Err("nu este rândul tău".into()),
                    );
                } else {
                    game.set_viewer(Some(me));
                    let m = game.legal_moves();
                    send(&mut seats[i].w, &ServerMsg::Legal(m));
                }
            }
            Ok(ClientMsg::Move(t)) => {
                if game.to_move() != me {
                    send(
                        &mut seats[i].w,
                        &ServerMsg::Err("nu este rândul tău".into()),
                    );
                    continue;
                }
                game.set_viewer(Some(me));
                match game.play(&t) {
                    Err(e) => send(&mut seats[i].w, &ServerMsg::Err(e)),
                    Ok(c) => {
                        send(&mut seats[i].w, &ServerMsg::Ok(c));
                        send_states(game, &mut seats, &mut cfg.spy);
                        if let Some(o) = game.outcome() {
                            return finish(&mut seats, Some(o), "terminat");
                        }
                        deadline = Instant::now() + cfg.turn_timeout;
                    }
                }
            }
        }
    }
}

/// Counts a bad line; true when the connection exceeded its allowance.
fn strike(s: &mut Seat, max: u32, why: &str) -> bool {
    s.bad += 1;
    send(&mut s.w, &ServerMsg::Err(why.to_string()));
    s.bad >= max
}

// ---------------------------------------------------------------- client

/// Plays as a terminal client. `input` provides the user's lines; output goes to `out`.
pub fn run_client<R, W>(addr: &str, input: R, out: &mut W) -> io::Result<GameResult>
where
    R: BufRead + Send + 'static,
    W: Write,
{
    let sock = addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "adresă invalidă"))?;
    let s = TcpStream::connect_timeout(&sock, Duration::from_secs(10))?;
    let _ = s.set_nodelay(true);
    let mut w = s.try_clone()?;
    let mut r = BufReader::new(s);
    write_client(&mut w, &format!("HELLO {VERSION}"))?;
    match read_server(&mut r)? {
        Some(ServerMsg::Hello) => {}
        Some(ServerMsg::Err(e)) => return Err(io::Error::other(e)),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "serverul nu răspunde cu HELLO",
            ))
        }
    }
    let mut uw = w.try_clone()?;
    std::thread::spawn(move || {
        let mut input = input;
        while let Ok(Line::Text(l)) = read_line_capped(&mut input, MAX_LINE) {
            let l = l.trim();
            let msg = match l {
                "" => continue,
                "?" => "LEGAL".to_string(),
                "q" | "quit" => "QUIT".to_string(),
                "state" => "STATE".to_string(),
                _ if l.len() > 5 && l[..5].eq_ignore_ascii_case("chat ") => l.to_string(),
                _ => format!("MOVE {l}"),
            };
            if write_client(&mut uw, &msg).is_err() {
                return;
            }
        }
    });
    let mut side = 0u8;
    loop {
        let Some(m) = read_server(&mut r)? else {
            writeln!(out, "Conexiune închisă.")?;
            return Ok(GameResult {
                outcome: None,
                reason: "conexiune închisă".into(),
            });
        };
        match m {
            ServerMsg::Hello => {}
            ServerMsg::Start { id, seed, side: sd } => {
                side = sd;
                writeln!(out, "Joc {id} (seed {seed}); ești Jucătorul {sd}.")?;
            }
            ServerMsg::State { turn, lines } => {
                for l in &lines {
                    writeln!(out, "{l}")?;
                }
                match turn {
                    0 => {}
                    t if t == side => {
                        writeln!(out, "Rândul tău > (? = mutări, chat TEXT, q = ieșire)")?
                    }
                    _ => writeln!(out, "Aștepți adversarul...")?,
                }
            }
            ServerMsg::Ok(t) => writeln!(out, "Ai jucat: {t}")?,
            ServerMsg::Err(e) => writeln!(out, "Eroare: {e}")?,
            ServerMsg::Legal(v) => writeln!(out, "Mutări: {}", v.join(" "))?,
            ServerMsg::Chat { side, text } => writeln!(out, "[Jucător {side}] {text}")?,
            ServerMsg::End { outcome, reason } => {
                let o = match outcome.as_str() {
                    "win1" => Some(Outcome::Win(Player::One)),
                    "win2" => Some(Outcome::Win(Player::Two)),
                    "draw" => Some(Outcome::Draw),
                    _ => None,
                };
                let txt = match outcome.as_str() {
                    "win1" => "Câștigă Jucătorul 1.",
                    "win2" => "Câștigă Jucătorul 2.",
                    "draw" => "Remiză.",
                    _ => "Joc întrerupt.",
                };
                writeln!(out, "{txt} ({reason})")?;
                return Ok(GameResult { outcome: o, reason });
            }
        }
        out.flush()?;
    }
}

// ---------------------------------------------------------------- CLI

fn opt<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].as_str())
}

/// `polytabletop host ID [--port P] [--seed N] [--side 1|2|random] [--timeout SECS]`
pub fn cmd_host(args: &[String]) -> i32 {
    let Some(id) = args.first() else {
        eprintln!(
            "Folosire: polytabletop host ID [--port P] [--seed N] [--side 1|2|random] [--timeout SECS]"
        );
        return 2;
    };
    if polytabletop_games::find(id).is_none() {
        eprintln!("Joc necunoscut: {id}. Încearcă `polytabletop list`.");
        return 2;
    }
    let port: u16 = match opt(args, "--port").map(str::parse) {
        None => 7878,
        Some(Ok(p)) => p,
        Some(Err(_)) => {
            eprintln!("--port așteaptă un număr 0-65535.");
            return 2;
        }
    };
    let seed = match opt(args, "--seed").map(str::parse::<u64>) {
        None => Rng::from_clock().next_u64() % 1_000_000,
        Some(Ok(n)) => n,
        Some(Err(_)) => {
            eprintln!("--seed așteaptă un număr întreg fără semn.");
            return 2;
        }
    };
    let host_side = match opt(args, "--side") {
        None | Some("1") => Some(Player::One),
        Some("2") => Some(Player::Two),
        Some("random") => None,
        Some(_) => {
            eprintln!("--side: 1, 2 sau random.");
            return 2;
        }
    };
    let timeout = opt(args, "--timeout")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(600);
    Rng::set_global_seed(Some(seed));
    let listener = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Nu pot asculta pe portul {port}: {e}");
            return 1;
        }
    };
    let local = listener.local_addr().map(|a| a.port()).unwrap_or(port);
    let cfg = ServerCfg {
        host_side,
        turn_timeout: Duration::from_secs(timeout),
        ..ServerCfg::default()
    };
    let id2 = id.clone();
    let srv = std::thread::spawn(move || serve(listener, &id2, seed, cfg));
    // The host plays through its own connection, made first so it takes the host seat.
    let addr = format!("127.0.0.1:{local}");
    println!(
        "Aștept adversarul pe portul {local}; el rulează: polytabletop join <IP-ul-tău>:{local}"
    );
    let res = run_client(&addr, BufReader::new(io::stdin()), &mut io::stdout());
    let _ = srv.join();
    match res {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Eroare de rețea: {e}");
            1
        }
    }
}

/// `polytabletop join HOST:PORT`
pub fn cmd_join(args: &[String]) -> i32 {
    let Some(addr) = args.first() else {
        eprintln!("Folosire: polytabletop join HOST:PORT");
        return 2;
    };
    match run_client(addr, BufReader::new(io::stdin()), &mut io::stdout()) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Eroare de rețea: {e}");
            1
        }
    }
}
