use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};
use polytabletop_net::net::{serve, GameResult, ServerCfg, Spy};
use polytabletop_net::proto::*;
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

struct C {
    r: BufReader<TcpStream>,
    w: TcpStream,
}

impl C {
    fn raw(addr: &str) -> C {
        let s = TcpStream::connect(addr).unwrap();
        s.set_read_timeout(Some(Duration::from_secs(20))).unwrap();
        C {
            r: BufReader::new(s.try_clone().unwrap()),
            w: s,
        }
    }
    fn join(addr: &str) -> C {
        let mut c = C::raw(addr);
        c.say(&format!("HELLO {VERSION}"));
        assert_eq!(c.recv(), ServerMsg::Hello);
        c
    }
    fn say(&mut self, l: &str) {
        write_client(&mut self.w, l).unwrap();
    }
    fn recv(&mut self) -> ServerMsg {
        read_server(&mut self.r).unwrap().expect("eof")
    }
    fn recv_opt(&mut self) -> Option<ServerMsg> {
        read_server(&mut self.r).ok().flatten()
    }
}

type Log = Arc<Mutex<Vec<(Player, String, String)>>>;

fn start(id: &str, seed: u64, mut cfg: ServerCfg) -> (String, JoinHandle<GameResult>, Log) {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = l.local_addr().unwrap().to_string();
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let lg = log.clone();
    let spy: Spy =
        Box::new(move |p, t, o| lg.lock().unwrap().push((p, t.to_string(), o.to_string())));
    cfg.spy = Some(spy);
    let id = id.to_string();
    let h = std::thread::spawn(move || serve(l, &id, seed, cfg).unwrap());
    (addr, h, log)
}

fn lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

/// Lines of `other` that are not in `mine` must not appear anywhere in `mine`.
fn assert_no_leak(mine: &[String], other: &[String], what: &str) -> usize {
    let mut private = 0;
    for l in other {
        let t = l.trim();
        if t.len() >= 4 && !mine.iter().any(|m| m == l) {
            private += 1;
            assert!(
                !mine.iter().any(|m| m.contains(t)),
                "{what}: scurgere: «{t}»"
            );
        }
    }
    private
}

fn play_full(id: &str, seed: u64, max_plies: usize) -> (GameResult, usize, bool) {
    let (addr, h, log) = start(id, seed, ServerCfg::default());
    let mut c = [C::join(&addr), C::join(&addr)];
    for (i, x) in c.iter_mut().enumerate() {
        assert_eq!(
            x.recv(),
            ServerMsg::Start {
                id: id.into(),
                seed,
                side: i as u8 + 1
            }
        );
    }
    let mut rng = Rng::new(seed ^ 0xABCD);
    let mut states: [Vec<String>; 2] = [vec![], vec![]];
    let mut turn;
    let mut plies = 0;
    let mut private_seen = 0;
    macro_rules! read_states {
        () => {{
            let mut t = 9;
            for i in 0..2 {
                match c[i].recv() {
                    ServerMsg::State { turn, lines } => {
                        t = turn;
                        states[i] = lines;
                    }
                    m => panic!("așteptam STATE, am primit {m:?}"),
                }
            }
            for i in 0..2 {
                private_seen += assert_no_leak(&states[i], &states[1 - i], id);
            }
            t
        }};
    }
    turn = read_states!();
    while turn != 0 && plies < max_plies {
        let m = (turn - 1) as usize;
        c[1 - m].say("MOVE zzz"); // out of turn: must be refused
        assert!(matches!(c[1 - m].recv(), ServerMsg::Err(_)));
        c[m].say("LEGAL");
        let ServerMsg::Legal(v) = c[m].recv() else {
            panic!("LEGAL")
        };
        assert!(!v.is_empty());
        c[m].say("MOVE definitiv-ilegal");
        assert!(matches!(c[m].recv(), ServerMsg::Err(_)));
        c[m].say(&format!("MOVE {}", v[rng.below(v.len())]));
        assert!(matches!(c[m].recv(), ServerMsg::Ok(_)));
        plies += 1;
        turn = read_states!();
    }
    let finished = turn == 0;
    if finished {
        for x in c.iter_mut() {
            assert!(matches!(x.recv(), ServerMsg::End { .. }));
        }
    } else {
        c[0].say("QUIT");
        match c[1].recv() {
            ServerMsg::End { outcome, .. } => assert_eq!(outcome, "aborted"),
            m => panic!("{m:?}"),
        }
    }
    let res = h.join().unwrap();
    // Everything the server rendered for a viewer is what that viewer got, and only that.
    for (p, t, o) in log.lock().unwrap().iter() {
        let (mine, other): (Vec<String>, Vec<String>) = (
            lines(t).iter().map(|s| s.to_string()).collect(),
            lines(o).iter().map(|s| s.to_string()).collect(),
        );
        private_seen += assert_no_leak(&mine, &other, &format!("{id} spy {p:?}"));
    }
    (res, private_seen, finished)
}

#[test]
fn full_games_five() {
    for (id, hidden) in [
        ("chess", false),
        ("belote", true),
        ("battleship", true),
        ("backgammon", false),
        ("poker", true),
    ] {
        let (res, private, finished) = play_full(id, 7, 700);
        eprintln!("{id}: terminat={finished} rezultat={res:?} linii private verificate={private}");
        if finished {
            assert!(res.outcome.is_some(), "{id}");
        }
        if hidden {
            assert!(private > 0, "{id}: testul de scurgere ar fi gol");
        }
    }
}

#[test]
fn belote_and_poker_hands_differ_per_viewer() {
    for id in ["belote", "poker", "battleship"] {
        let (addr, h, _log) = start(id, 3, ServerCfg::default());
        let mut a = C::join(&addr);
        let mut b = C::join(&addr);
        a.recv();
        b.recv();
        let (ServerMsg::State { lines: la, .. }, ServerMsg::State { lines: lb, .. }) =
            (a.recv(), b.recv())
        else {
            panic!()
        };
        assert_ne!(la, lb, "{id}: ambele părți văd aceeași informație");
        a.say("QUIT");
        assert!(matches!(b.recv(), ServerMsg::End { .. }));
        h.join().unwrap();
    }
}

#[test]
fn disconnect_aborts_and_timeout_forfeits() {
    let (addr, h, _) = start("chess", 1, ServerCfg::default());
    let mut a = C::join(&addr);
    let mut b = C::join(&addr);
    for x in [&mut a, &mut b] {
        x.recv();
        x.recv();
    }
    drop(b);
    loop {
        match a.recv() {
            ServerMsg::End { outcome, .. } => {
                assert_eq!(outcome, "aborted");
                break;
            }
            _ => continue,
        }
    }
    assert_eq!(h.join().unwrap().outcome, None);

    let cfg = ServerCfg {
        turn_timeout: Duration::from_millis(300),
        ..ServerCfg::default()
    };
    let (addr, h, _) = start("chess", 1, cfg);
    let mut a = C::join(&addr);
    let mut b = C::join(&addr);
    a.recv();
    b.recv();
    let res = h.join().unwrap(); // white (host) never moves
    assert_eq!(res.outcome, Some(Outcome::Win(Player::Two)));
}

#[test]
fn random_side_and_chat() {
    let cfg = ServerCfg {
        host_side: None,
        ..ServerCfg::default()
    };
    let (addr, h, _) = start("chess", 12345, cfg);
    let mut a = C::join(&addr);
    let mut b = C::join(&addr);
    let ServerMsg::Start { side: sa, .. } = a.recv() else {
        panic!()
    };
    let ServerMsg::Start { side: sb, .. } = b.recv() else {
        panic!()
    };
    assert_eq!(sa + sb, 3);
    a.recv();
    b.recv();
    a.say("chat salut  tuturor\x07");
    assert_eq!(
        b.recv(),
        ServerMsg::Chat {
            side: sa,
            text: "salut  tuturor".into()
        }
    );
    b.say("QUIT");
    h.join().unwrap();
}

#[test]
fn malformed_input_fuzz() {
    let cfg = ServerCfg {
        max_bad: 40,
        hello_timeout: Duration::from_millis(400),
        ..ServerCfg::default()
    };
    let (addr, h, _) = start("chess", 5, cfg);
    // Junk connections before the game: silence, bad version, binary, overlong line.
    let _silent = TcpStream::connect(&addr).unwrap();
    let mut v = C::raw(&addr);
    v.say("HELLO polytabletop/99");
    assert!(matches!(v.recv(), ServerMsg::Err(_)));
    let mut g = TcpStream::connect(&addr).unwrap();
    let _ = g.write_all(&[0xff, 0xfe, 0, 1, 2, b'\n']);
    let mut g2 = TcpStream::connect(&addr).unwrap();
    let _ = g2.write_all(&vec![b'A'; 100_000]);
    let mut a = C::join(&addr);
    let mut b = C::join(&addr);
    a.recv();
    b.recv();
    a.recv();
    b.recv();
    // Fuzz from b with deterministic pseudo-random garbage.
    let mut rng = Rng::new(99);
    let words = [
        "MOVE",
        "move",
        "LEGAL",
        "STATE",
        "CHAT",
        "HELLO",
        "",
        " ",
        "END",
        "OK",
        "\t",
        "e2e4",
        "STATE 1 99999",
    ];
    let mut sent_bad = 0;
    for i in 0..120 {
        let kind = rng.below(5);
        let line: Vec<u8> = match kind {
            0 => (0..rng.below(300))
                .map(|_| rng.below(256) as u8)
                .filter(|&x| x != b'\n')
                .collect(),
            1 => format!(
                "{} {}",
                words[rng.below(words.len())],
                words[rng.below(words.len())]
            )
            .into_bytes(),
            2 => vec![b'x'; 1025 + rng.below(5000)],
            3 => "MOVE \u{202e}\u{0}é☃".as_bytes().to_vec(),
            _ => b"CHAT".to_vec(),
        };
        let mut l = line;
        l.push(b'\n');
        if b.w.write_all(&l).is_err() {
            break;
        }
        sent_bad = i;
    }
    let _ = sent_bad;
    // Server must survive: a (valid player) is told the game ended, never hangs/panics.
    let mut ended = false;
    while let Some(m) = a.recv_opt() {
        if let ServerMsg::End { outcome, .. } = m {
            assert!(outcome == "aborted" || outcome.starts_with("win"));
            ended = true;
            break;
        }
    }
    assert!(ended);
    let r = h.join().expect("serverul nu are voie să intre în panică");
    let _ = r;
}
