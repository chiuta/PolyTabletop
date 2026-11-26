//! PolyTabletop command-line front-end.
//!
//!   polytabletop list                      games you can play now
//!   polytabletop catalog [--kind KIND]     all target games and their status
//!   polytabletop play ID [--you one|two|both|none] [--seed N]
//!   polytabletop langs                     available interface languages
//!   polytabletop --lang CODE <command>     interface language (en ro fr it es pt de hu)
//!
//! While playing: type a move; `?` lists legal moves, `hint` asks the AI for
//! a suggestion, `q` quits.

mod level;

mod export;
mod record;
mod replay;

mod arena;

use polytabletop_core::i18n::{self, tr, tr_fmt, Lang};
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};
use polytabletop_games::{catalog, find, playable, Kind};
use std::io::{self, BufRead, Write};

fn usage() {
    println!(
        "{}\n  \
         polytabletop list\n  \
         polytabletop catalog [--kind implemented|traditional|branded|toolkit]\n  \
         polytabletop play ID [--you one|two|both|none] [--seed N] [--level easy|normal|hard|expert]\n  \
         polytabletop arena ID [--games N] [--ms A:B[:C]] [--level easy:hard] [--threads T]\n  \
         polytabletop bench | polytabletop ladder ID\n  \
         polytabletop host ID [--port P] [--seed N] [--side 1|2|random]\n  \
         polytabletop join HOST:PORT\n{}\n",
        tr_fmt("cli.usage_head", &[("app", polytabletop_core::APP_NAME)]),
        tr("cli.usage_tail")
    );
}

/// Extracts the global `--lang CODE` flag (anywhere in the arguments) and sets the language.
fn init_lang(args: &mut Vec<String>) {
    let mut flag = None;
    if let Some(i) = args.iter().position(|a| a == "--lang") {
        match args.get(i + 1) {
            Some(c) => flag = Some(c.clone()),
            None => {
                i18n::set_lang(i18n::detect(None));
                eprintln!(
                    "{}",
                    tr_fmt("cli.lang_missing", &[("list", &i18n::codes_list())])
                );
                std::process::exit(2);
            }
        }
        args.drain(i..i + 2);
    }
    if let Some(c) = &flag {
        let Some(l) = Lang::from_code(c) else {
            i18n::set_lang(i18n::detect(None));
            eprintln!(
                "{}",
                tr_fmt(
                    "cli.unknown_lang",
                    &[("code", c), ("list", &i18n::codes_list())]
                )
            );
            std::process::exit(2);
        };
        i18n::set_lang(l);
    } else {
        i18n::set_lang(i18n::detect(None));
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    init_lang(&mut args);
    match args.first().map(String::as_str) {
        Some("list") => cmd_list(),
        Some("catalog") => cmd_catalog(&args[1..]),
        Some("play") => cmd_play(&args[1..]),
        Some("replay") => replay::cmd_replay(&args[1..]),
        Some("export") => export::cmd_export(&args[1..]),
        Some("import-pgn") => export::cmd_import_pgn(&args[1..]),
        Some(c @ ("arena" | "bench" | "ladder")) => arena::dispatch(c, &args[1..]),
        Some("langs") => cmd_langs(),
        Some("host") => std::process::exit(polytabletop_net::net::cmd_host(&args[1..])),
        Some("join") => std::process::exit(polytabletop_net::net::cmd_join(&args[1..])),
        _ => usage(),
    }
}

fn cmd_langs() {
    println!(
        "{}",
        tr_fmt("cli.langs_header", &[("cur", i18n::lang().code())])
    );
    for l in Lang::ALL {
        println!("  {}  {}", l.code(), l.name_native());
    }
}

fn kind_label(k: Kind) -> &'static str {
    tr(match k {
        Kind::Implemented => "kind.implemented",
        Kind::Traditional => "kind.traditional",
        Kind::Branded => "kind.branded",
        Kind::Toolkit => "kind.toolkit",
    })
}

fn cmd_list() {
    let l = i18n::lang();
    for p in playable() {
        let title = polytabletop_games::i18n::title(p.id, l).unwrap_or(p.title);
        let rules = polytabletop_games::i18n::rules(p.id, l).unwrap_or(p.rules);
        println!("{:<9} {title}\n          {rules}\n", p.id);
    }
}

fn cmd_catalog(args: &[String]) {
    let want = args
        .windows(2)
        .find(|w| w[0] == "--kind")
        .map(|w| w[1].as_str());
    let kind_of = |s: &str| match s {
        "implemented" => Some(Kind::Implemented),
        "traditional" => Some(Kind::Traditional),
        "branded" => Some(Kind::Branded),
        "toolkit" => Some(Kind::Toolkit),
        _ => None,
    };
    let filter = want.and_then(kind_of);
    let mut counts = [0usize; 4];
    for e in catalog() {
        let i = match e.kind {
            Kind::Implemented => 0,
            Kind::Traditional => 1,
            Kind::Branded => 2,
            Kind::Toolkit => 3,
        };
        counts[i] += 1;
        if filter.is_none() || filter == Some(e.kind) {
            println!(
                "{:<44} {:<12} {:<22} {}",
                e.name,
                e.category,
                e.origin,
                kind_label(e.kind)
            );
        }
    }
    let n = |v: usize| v.to_string();
    println!(
        "{}",
        tr_fmt(
            "cli.catalog_total",
            &[
                ("total", &n(catalog().len())),
                ("impl", &n(counts[0])),
                ("trad", &n(counts[1])),
                ("own", &n(counts[2])),
                ("toolkit", &n(counts[3])),
            ]
        )
    );
}

fn cmd_play(args: &[String]) {
    let Some(id) = args.first() else {
        usage();
        return;
    };
    let Some(p) = find(id) else {
        eprintln!("{}", tr_fmt("cli.unknown_game", &[("id", id)]));
        std::process::exit(2);
    };
    let you = args
        .windows(2)
        .find(|w| w[0] == "--you")
        .map(|w| w[1].as_str())
        .unwrap_or("one");
    if let Err(e) = level::apply(&args[1..]) {
        eprintln!("{e}");
        std::process::exit(2);
    }
    let human = |pl: Player| match you {
        "both" => true,
        "none" => false,
        "two" => pl == Player::Two,
        _ => pl == Player::One,
    };
    // --seed N seeds the whole process (deals, dice, AI) before any game is made.
    let seed = args
        .windows(2)
        .find(|w| w[0] == "--seed")
        .map(|w| w[1].parse::<u64>());
    let flag = |name: &str| args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone());
    let rec_path = flag("--record");
    let seed = if rec_path.is_some() && seed.is_none() {
        Some(Ok(record::fresh_seed()))
    } else {
        seed
    };
    if let Some(Ok(n)) = seed {
        Rng::set_global_seed(Some(n));
    }
    let mut game = (p.make)();
    // Private hands: show the human's own hand (or hand over the screen in hot-seat).
    game.set_viewer(match you {
        "both" => None,
        "two" => Some(Player::Two),
        _ => Some(Player::One),
    });
    let mut hot_seat_leaks = false;
    if matches!(you, "both" | "two") && !polytabletop_games::HOTSEAT_SAFE.contains(&p.id) {
        // Randomised state (hidden hands, stock, dice) and no per-viewer rendering:
        // the screen would show Player 1's private information.
        let a = (p.make)().render();
        let b = (p.make)().render();
        if a != b {
            hot_seat_leaks =
                you == "both" && !matches!(p.category, "Abstract" | "Clasic" | "Zaruri");
            eprintln!("{}", tr("play.hotseat_warn"));
        }
    }
    // AI / hint move choices are seeded too.
    let mut rng = match seed {
        Some(Ok(n)) => {
            println!("(seed {n})");
            Rng::new(n)
        }
        Some(Err(_)) => {
            eprintln!("{}", tr("play.seed_bad"));
            std::process::exit(2);
        }
        None => Rng::from_clock(),
    };
    let mut recorder = rec_path.as_ref().map(|path| {
        let mut r = record::Record::new(p.id, seed_value(&seed), you);
        r.level = flag("--level");
        record::Recorder::new(path, r)
    });
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let lg = i18n::lang();
    let title = polytabletop_games::i18n::title(p.id, lg).unwrap_or(p.title);
    let rules = polytabletop_games::i18n::rules(p.id, lg).unwrap_or(p.rules);
    println!("{title} — {rules}\n");

    loop {
        println!("{}", game.render());
        if let Some(o) = game.outcome() {
            match o {
                Outcome::Win(Player::One) => println!("{}", tr("play.win1")),
                Outcome::Win(Player::Two) => println!("{}", tr("play.win2")),
                Outcome::Draw => println!("{}", tr("play.draw")),
            }
            return;
        }
        let who = game.to_move();
        let label = if who == Player::One { "1" } else { "2" };
        if human(who) {
            print!("{}", tr_fmt("play.prompt", &[("n", label)]));
            io::stdout().flush().ok();
            let Some(Ok(line)) = lines.next() else {
                println!();
                return;
            };
            let line = line.trim();
            match line {
                "q" | "quit" => return,
                "?" => println!(
                    "{}",
                    tr_fmt("play.moves", &[("list", &game.legal_moves().join(" "))])
                ),
                "hint" if hot_seat_leaks => {
                    println!("{}", tr("play.hint_unavailable"))
                }
                "hint" => match game.ai_move(rng.next_u64()) {
                    Some(m) => println!("{}", tr_fmt("play.hint", &[("m", &m)])),
                    None => println!("{}", tr("play.no_move")),
                },
                "" => {}
                mv => match game.play(mv) {
                    Ok(t) => {
                        if let Some(r) = recorder.as_mut() {
                            r.push(game.as_ref(), &t);
                        }
                    }
                    Err(e) => println!("{e}"),
                },
            }
        } else {
            match game.ai_move(rng.next_u64()) {
                Some(m) => {
                    if let Ok(t) = game.play(&m) {
                        if let Some(r) = recorder.as_mut() {
                            r.push(game.as_ref(), &t);
                        }
                    }
                    println!("{}\n", tr_fmt("play.ai_plays", &[("n", label), ("m", &m)]));
                }
                None => return,
            }
        }
    }
}

fn seed_value(seed: &Option<Result<u64, std::num::ParseIntError>>) -> u64 {
    match seed {
        Some(Ok(n)) => *n,
        _ => 0,
    }
}
