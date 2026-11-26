//! Mod de captură: `polytabletop-gui --capture ID --out DIR [--seed N] [--lang xx]
//! [--frames N] [--size 960x600] [--delay-ms 200] [--max-moves 400]`.
//!
//! Calculatorul joacă ambele părți (AI contra AI) pe aceeași stare pe care o
//! desenează GUI-ul „la masă” (`set_viewer(None)`); la mutările alese se cere
//! captura REALĂ a ferestrei (`Context::request_screenshot`) și se salvează ca
//! `DIR/frame_0001.png…`, apoi `DIR/meta.json`. Aplicația se închide singură.

use crate::App;
use eframe::egui;
use polytabletop_core::{GameObj, Outcome};
use polytabletop_games::HOTSEAT_SAFE;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(90);

struct Opts {
    id: String,
    out: String,
    frames: usize,
    size: (f32, f32),
    delay: Duration,
    max_moves: usize,
    /// timp maxim pentru simularea partidei AI contra AI (apoi se captează ce există)
    sim_budget: Duration,
}

fn arg<'a>(argv: &'a [String], key: &str) -> Option<&'a str> {
    argv.iter()
        .position(|a| a == key)
        .and_then(|i| argv.get(i + 1))
        .map(String::as_str)
}

fn parse(argv: &[String]) -> Result<Option<Opts>, String> {
    let Some(id) = arg(argv, "--capture") else {
        return Ok(None);
    };
    let num = |k: &str, d: usize| -> Result<usize, String> {
        arg(argv, k).map_or(Ok(d), |v| {
            v.parse().map_err(|_| format!("{k}: număr invalid"))
        })
    };
    let size = arg(argv, "--size").unwrap_or("960x600");
    let (w, h) = size
        .split_once('x')
        .and_then(|(w, h)| Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?)))
        .ok_or("--size: format WxH")?;
    Ok(Some(Opts {
        id: id.to_string(),
        out: arg(argv, "--out").ok_or("--out DIR lipsește")?.to_string(),
        frames: num("--frames", 20)?.max(1),
        size: (w, h),
        delay: Duration::from_millis(num("--delay-ms", 200)? as u64),
        max_moves: num("--max-moves", 400)?,
        sim_budget: Duration::from_secs(num("--sim-secs", 45)? as u64),
    }))
}

/// Dacă linia de comandă conține `--capture`, rulează fereastra de captură.
pub fn maybe_run(argv: &[String]) -> Option<eframe::Result<()>> {
    if argv.iter().any(|a| a == "--capture-list") {
        // id-urile jocurilor jucabile în GUI, unul pe linie, cu „1” dacă au tablă grafică
        for p in polytabletop_games::playable() {
            println!("{} {}", p.id, u8::from(crate::kind_of(p.id).is_some()));
        }
        std::process::exit(0);
    }
    let o = match parse(argv) {
        Ok(None) => return None,
        Ok(Some(o)) => o,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    if let Err(e) = std::fs::create_dir_all(&o.out) {
        eprintln!("{}: {e}", o.out);
        std::process::exit(2);
    }
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([o.size.0, o.size.1])
            .with_resizable(false),
        ..Default::default()
    };
    Some(eframe::run_native(
        crate::APP_NAME,
        opts,
        Box::new(move |_cc| Ok(Box::new(Cap::new(o)))),
    ))
}

/// O stare de joc după `n` mutări, cu jurnalul aferent.
struct Snap {
    game: Box<dyn GameObj>,
    log: Vec<String>,
    moves: Vec<String>,
}

enum Phase {
    /// se așteaptă desenarea stabilă a cadrului `plan[k]`
    Settle {
        since: Instant,
        frames: u32,
    },
    /// captura a fost cerută
    Waiting,
    Done,
}

struct Cap {
    app: App,
    o: Opts,
    snaps: Vec<Snap>,
    plan: Vec<usize>,
    k: usize,
    phase: Phase,
    tx: Sender<Arc<egui::ColorImage>>,
    rx: Receiver<Arc<egui::ColorImage>>,
    started: Instant,
    outcome: String,
    hidden: bool,
    saved: usize,
    timed_out: bool,
    error: String,
}

impl Cap {
    fn new(o: Opts) -> Self {
        let mut app = App::new();
        let (tx, rx) = channel();
        let mut error = String::new();
        let mut snaps = Vec::new();
        let mut outcome = "unfinished".to_string();
        let mut hidden = false;
        match app.games.iter().position(|g| g.id == o.id) {
            None => error = format!("joc necunoscut: {}", o.id),
            Some(i) => {
                app.sel = i;
                app.human = None; // „la masă”: privitorul = cel la mutare
                app.game = (app.games[i].make)();
                app.apply_viewer();
                app.path = app.default_path();
                let p = &app.games[i];
                hidden = HOTSEAT_SAFE.contains(&p.id) || crate::session::hotseat_leaks(p);
                let mut log = Vec::new();
                let mut moves = Vec::new();
                snaps.push(Snap {
                    game: app.game.clone_box(),
                    log: log.clone(),
                    moves: moves.clone(),
                });
                let sim = Instant::now();
                while app.game.outcome().is_none()
                    && moves.len() < o.max_moves
                    && sim.elapsed() < o.sim_budget
                {
                    let me = app.game.to_move();
                    let Some(m) = app.game.ai_move(app.rng.next_u64()) else {
                        outcome = "stalled".into();
                        break;
                    };
                    match app.game.play(&m) {
                        Ok(t) => {
                            log.push(format!(
                                "{} ({}): {t}",
                                crate::session::who(me),
                                crate::gui_i18n::tr("cpu_tag")
                            ));
                            moves.push(t);
                        }
                        Err(_) => {
                            outcome = "stalled".into();
                            break;
                        }
                    }
                    snaps.push(Snap {
                        game: app.game.clone_box(),
                        log: log.clone(),
                        moves: moves.clone(),
                    });
                }
                match app.game.outcome() {
                    Some(Outcome::Win(p)) => {
                        outcome = if p == polytabletop_core::Player::One {
                            "win1".into()
                        } else {
                            "win2".into()
                        }
                    }
                    Some(Outcome::Draw) => outcome = "draw".into(),
                    None => {}
                }
            }
        }
        // cadre: început, mijloc, final, eșantionate uniform pe mutări
        let t = snaps.len().saturating_sub(1);
        let n = o.frames.min(t + 1).max(1);
        let mut plan: Vec<usize> = (0..n)
            .map(|i| if n == 1 { 0 } else { i * t / (n - 1) })
            .collect();
        plan.dedup();
        let mut c = Cap {
            app,
            o,
            snaps,
            plan,
            k: 0,
            phase: Phase::Settle {
                since: Instant::now(),
                frames: 0,
            },
            tx,
            rx,
            started: Instant::now(),
            outcome,
            hidden,
            saved: 0,
            timed_out: false,
            error,
        };
        c.show(0);
        c
    }

    /// Pune pe ecran starea de după `plan[k]` mutări.
    fn show(&mut self, k: usize) {
        let Some(&i) = self.plan.get(k) else { return };
        let Some(s) = self.snaps.get(i) else { return };
        let mut g = s.game.clone_box();
        g.set_viewer(None);
        self.app.game = g;
        self.app.log = s.log.clone();
        self.app.moves = s.moves.clone();
        self.app.msg.clear();
        self.app.info.clear();
        self.app.handover = None;
        self.app.clear_selection();
    }

    fn finish(&mut self, ctx: &egui::Context) {
        let complete = self.error.is_empty()
            && !self.timed_out
            && self.saved == self.plan.len()
            && self.outcome.starts_with(['w', 'd']);
        let total = self.snaps.len().saturating_sub(1);
        let meta = format!(
            "{{\"id\":\"{}\",\"moves\":{},\"frames\":{},\"outcome\":\"{}\",\"hidden_info_shown\":{},\"complete\":{},\"timed_out\":{},\"error\":\"{}\"}}\n",
            self.o.id,
            total,
            self.saved,
            self.outcome,
            self.hidden,
            complete,
            self.timed_out,
            self.error.replace(['"', '\\'], "'")
        );
        if let Err(e) = std::fs::write(format!("{}/meta.json", self.o.out), meta) {
            eprintln!("meta.json: {e}");
        }
        self.phase = Phase::Done;
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn save(&mut self, img: &egui::ColorImage) {
        let [w, h] = img.size;
        let mut rgb = Vec::with_capacity(w * h * 3);
        for c in &img.pixels {
            rgb.extend_from_slice(&[c.r(), c.g(), c.b()]);
        }
        self.saved += 1;
        let path = format!("{}/frame_{:04}.png", self.o.out, self.saved);
        if let Err(e) = std::fs::write(&path, png(w as u32, h as u32, &rgb)) {
            self.error = format!("{path}: {e}");
        }
    }
}

impl eframe::App for Cap {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.app.ui(ui, frame);
        if matches!(self.phase, Phase::Done) {
            return;
        }
        if !self.error.is_empty() || self.plan.is_empty() {
            return self.finish(&ctx);
        }
        if self.started.elapsed() > TIMEOUT {
            self.timed_out = true;
            return self.finish(&ctx);
        }
        ctx.request_repaint();
        match &mut self.phase {
            Phase::Settle { since, frames } => {
                *frames += 1;
                if *frames >= 4 && since.elapsed() >= self.o.delay {
                    let tx = self.tx.clone();
                    ctx.request_screenshot(move |img| {
                        let _ = tx.send(img);
                    });
                    self.phase = Phase::Waiting;
                }
            }
            Phase::Waiting => {
                if let Ok(img) = self.rx.try_recv() {
                    self.save(&img);
                    self.k += 1;
                    if self.k >= self.plan.len() || !self.error.is_empty() {
                        return self.finish(&ctx);
                    }
                    self.show(self.k);
                    self.phase = Phase::Settle {
                        since: Instant::now(),
                        frames: 0,
                    };
                }
            }
            Phase::Done => {}
        }
    }
}

// --- PNG minimal (RGB, deflate „stored”; Python îl recomprimă) ---

fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= u32::from(b);
        for _ in 0..8 {
            c = if c & 1 != 0 {
                0xEDB8_8320 ^ (c >> 1)
            } else {
                c >> 1
            };
        }
    }
    !c
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut body = kind.to_vec();
    body.extend_from_slice(data);
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
}

fn png(w: u32, h: u32, rgb: &[u8]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(rgb.len() + h as usize);
    for row in rgb.chunks(w as usize * 3) {
        raw.push(0);
        raw.extend_from_slice(row);
    }
    let mut z = vec![0x78, 0x01];
    let mut chunks = raw.chunks(65535).peekable();
    while let Some(c) = chunks.next() {
        z.push(u8::from(chunks.peek().is_none()));
        z.extend_from_slice(&(c.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(c.len() as u16)).to_le_bytes());
        z.extend_from_slice(c);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &x in &raw {
        a = (a + u32::from(x)) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&w.to_be_bytes());
    ihdr.extend_from_slice(&h.to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &z);
    chunk(&mut out, b"IEND", &[]);
    out
}
