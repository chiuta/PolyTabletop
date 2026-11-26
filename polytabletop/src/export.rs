//! `polytabletop export FILE --format pgn|text|json` and `polytabletop import-pgn FILE`.

use crate::record::{outcome_str, Record};
use crate::replay::run_with;
use polytabletop_core::{GameObj, Player};
use polytabletop_games::chess_san::{move_from_san, parse_pgn, san_of};

/// One replayed ply: (mover 1/2, canonical move text, SAN if chess).
struct Ply {
    who: u8,
    text: String,
    san: Option<String>,
}

fn plies(rec: &Record) -> Result<Vec<Ply>, String> {
    let mut out = Vec::new();
    let chess = rec.game == "chess";
    let mut prev: Option<Box<dyn GameObj>> = None;
    let mut err = None;
    run_with(rec, &mut |_, mv, g| {
        if let (Some(m), Some(before)) = (mv, prev.as_ref()) {
            out.push(Ply {
                who: if before.to_move() == Player::One {
                    1
                } else {
                    2
                },
                text: m.to_string(),
                san: if chess {
                    san_of(before.as_ref(), m)
                } else {
                    None
                },
            });
            if chess && out.last().is_some_and(|p| p.san.is_none()) {
                err = Some(format!("SAN imposibil pentru «{m}»"));
            }
        }
        prev = Some(g.clone_box());
    })?;
    match err {
        Some(e) => Err(e),
        None => Ok(out),
    }
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

pub fn to_text(rec: &Record) -> Result<String, String> {
    let ps = plies(rec)?;
    let mut s = format!("Joc: {}\nSeed: {}\n", rec.game, rec.seed);
    if let Some(l) = &rec.level {
        s.push_str(&format!("Nivel: {l}\n"));
    }
    s.push_str(&format!(
        "Rezultat: {}\nMutări: {}\n\n",
        rec.result,
        ps.len()
    ));
    for (i, p) in ps.iter().enumerate() {
        s.push_str(&format!("{:>4}. J{} {}\n", i + 1, p.who, p.text));
    }
    Ok(s)
}

pub fn to_json(rec: &Record) -> Result<String, String> {
    let ps = plies(rec)?;
    let mut s = String::from("{\n");
    s.push_str(&format!("  \"game\": {},\n", json_str(&rec.game)));
    s.push_str(&format!("  \"seed\": {},\n", rec.seed));
    match &rec.level {
        Some(l) => s.push_str(&format!("  \"level\": {},\n", json_str(l))),
        None => s.push_str("  \"level\": null,\n"),
    }
    s.push_str(&format!("  \"result\": {},\n", json_str(&rec.result)));
    s.push_str("  \"moves\": [\n");
    for (i, p) in ps.iter().enumerate() {
        s.push_str(&format!(
            "    {{\"n\": {}, \"player\": {}, \"move\": {}}}{}\n",
            i + 1,
            p.who,
            json_str(&p.text),
            if i + 1 < ps.len() { "," } else { "" }
        ));
    }
    s.push_str("  ]\n}\n");
    Ok(s)
}

fn pgn_result(r: &str) -> &'static str {
    match r {
        "win1" => "1-0",
        "win2" => "0-1",
        "draw" => "1/2-1/2",
        _ => "*",
    }
}

/// PGN with the Seven Tag Roster (Event, Site, Date, Round, White, Black,
/// Result). No date is stored in transcripts, so Date is the unknown `????.??.??`.
pub fn to_pgn(rec: &Record) -> Result<String, String> {
    if rec.game != "chess" {
        return Err("formatul pgn există doar pentru șah (alege text sau json)".into());
    }
    let ps = plies(rec)?;
    let res = pgn_result(&rec.result);
    let mut s = String::new();
    for (k, v) in [
        ("Event", "PolyTabletop game"),
        ("Site", "PolyTabletop"),
        ("Date", "????.??.??"),
        ("Round", "-"),
        ("White", "Player 1"),
        ("Black", "Player 2"),
        ("Result", res),
    ] {
        s.push_str(&format!("[{k} \"{v}\"]\n"));
    }
    s.push_str(&format!("[Seed \"{}\"]\n\n", rec.seed));
    let mut line = String::new();
    let mut toks: Vec<String> = Vec::new();
    for (i, p) in ps.iter().enumerate() {
        if i % 2 == 0 {
            toks.push(format!("{}.", i / 2 + 1));
        }
        toks.push(p.san.clone().unwrap_or_default());
    }
    toks.push(res.to_string());
    for t in toks {
        if !line.is_empty() && line.len() + 1 + t.len() > 80 {
            s.push_str(&line);
            s.push('\n');
            line.clear();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(&t);
    }
    s.push_str(&line);
    s.push('\n');
    Ok(s)
}

/// Best-effort import of a standard chess PGN (first game, from the initial
/// position) into a transcript; every move is validated against PolyTabletop's chess.
pub fn import_pgn(text: &str) -> Result<(Record, Vec<String>), String> {
    let pgn = parse_pgn(text);
    let mut warnings = Vec::new();
    if pgn
        .tags
        .iter()
        .any(|(k, v)| (k == "SetUp" && v == "1") || k == "FEN")
    {
        return Err("PGN cu poziție de start (FEN) nu e suportat; doar partide complete".into());
    }
    if let Some((_, v)) = pgn
        .tags
        .iter()
        .find(|(k, v)| k == "Variant" && v != "Standard")
    {
        return Err(format!("varianta «{v}» nu e suportată"));
    }
    let seed = pgn
        .tags
        .iter()
        .find(|(k, _)| k == "Seed")
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0);
    let p = polytabletop_games::find("chess").ok_or("jocul chess lipsește")?;
    let mut g = (p.make)();
    let mut rec = Record::new("chess", seed, "one");
    for (i, san) in pgn.moves.iter().enumerate() {
        let c = move_from_san(g.as_ref(), san).ok_or_else(|| {
            format!(
                "mutarea {} «{san}» nu este legală în poziția curentă",
                i + 1
            )
        })?;
        g.play(&c)?;
        rec.moves.push(c);
    }
    rec.result = outcome_str(g.outcome()).to_string();
    let declared = pgn
        .result
        .or_else(|| {
            pgn.tags
                .iter()
                .find(|(k, _)| k == "Result")
                .map(|(_, v)| v.clone())
        })
        .unwrap_or_else(|| "*".into());
    if declared != pgn_result(&rec.result) {
        warnings.push(format!(
            "rezultatul PGN ({declared}) diferă de cel din joc ({}); se păstrează cel din joc \
             (abandon/acord nu se pot reconstrui)",
            pgn_result(&rec.result)
        ));
    }
    Ok((rec, warnings))
}

fn read(file: &str) -> String {
    std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("Nu pot citi {file}: {e}");
        std::process::exit(2)
    })
}

pub fn cmd_export(args: &[String]) {
    let Some(file) = args.first() else {
        eprintln!("Folosire: polytabletop export FILE --format pgn|text|json");
        std::process::exit(2);
    };
    let fmt = args
        .windows(2)
        .find(|w| w[0] == "--format")
        .map(|w| w[1].as_str())
        .unwrap_or("text");
    let rec = Record::parse(&read(file)).unwrap_or_else(|e| {
        eprintln!("{file}: {e}");
        std::process::exit(2)
    });
    let out = match fmt {
        "pgn" => to_pgn(&rec),
        "text" => to_text(&rec),
        "json" => to_json(&rec),
        other => Err(format!("format necunoscut: {other} (pgn|text|json)")),
    };
    match out {
        Ok(s) => print!("{s}"),
        Err(e) => {
            eprintln!("EROARE: {e}");
            std::process::exit(1);
        }
    }
}

pub fn cmd_import_pgn(args: &[String]) {
    let Some(file) = args.first() else {
        eprintln!("Folosire: polytabletop import-pgn FILE [--out TRANSCRIERE]");
        std::process::exit(2);
    };
    match import_pgn(&read(file)) {
        Ok((rec, warns)) => {
            for w in warns {
                eprintln!("Atenție: {w}");
            }
            match args.windows(2).find(|w| w[0] == "--out") {
                Some(w) => {
                    if let Err(e) = std::fs::write(&w[1], rec.to_text()) {
                        eprintln!("Nu pot scrie {}: {e}", w[1]);
                        std::process::exit(2);
                    }
                    eprintln!("Scris {} ({} mutări).", w[1], rec.moves.len());
                }
                None => print!("{}", rec.to_text()),
            }
        }
        Err(e) => {
            eprintln!("EROARE import: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::SEED_LOCK;

    const OPERA: &str = "1. e4 e5 2. Nf3 d6 3. d4 Bg4 4. dxe5 Bxf3 5. Qxf3 dxe5 6. Bc4 Nf6 7. Qb3 Qe7 8. Nc3 c6 9. Bg5 b5 10. Nxb5 cxb5 11. Bxb5+ Nbd7 12. O-O-O Rd8 13. Rxd7 Rxd7 14. Rd1 Qe6 15. Bxd7+ Nxd7 16. Qb8+ Nxb8 17. Rd8# 1-0";

    fn opera_record() -> Record {
        let text = format!(
            "[Event \"Paris\"]\n[Site \"Paris FRA\"]\n[Date \"1858.??.??\"]\n[Round \"-\"]\n[White \"Morphy, Paul\"]\n[Black \"Duke of Brunswick and Count Isouard\"]\n[Result \"1-0\"]\n\n{OPERA}\n"
        );
        let (rec, w) = import_pgn(&text).unwrap();
        assert!(w.is_empty(), "{w:?}");
        rec
    }

    #[test]
    fn import_opera_game_and_export_pgn_round_trip() {
        let _l = SEED_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let rec = opera_record();
        assert_eq!(rec.moves.len(), 33);
        assert_eq!(rec.result, "win1");
        let pgn = to_pgn(&rec).unwrap();
        for tag in ["Event", "Site", "Date", "Round", "White", "Black", "Result"] {
            assert!(pgn.contains(&format!("[{tag} ")), "{tag}");
        }
        let (rec2, w) = import_pgn(&pgn).unwrap();
        assert!(w.is_empty());
        assert_eq!(rec2.moves, rec.moves);
        assert_eq!(rec2.result, rec.result);
        // movetext survives verbatim
        let p = parse_pgn(&pgn);
        assert_eq!(p.moves.len(), 33);
        assert!(p.moves[32] == "Rd8#");
        assert_eq!(p.result.as_deref(), Some("1-0"));
        // transcript -> replay verifies the outcome
        run_with(&rec, &mut |_, _, _| {}).unwrap();
    }

    #[test]
    fn text_and_json_export() {
        let _l = SEED_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let rec = opera_record();
        let t = to_text(&rec).unwrap();
        assert!(t.contains("   1. J1 e2e4"));
        let j = to_json(&rec).unwrap();
        assert!(j.contains("\"result\": \"win1\""));
        assert!(j.contains("\"move\": \"d1d8\""));
        // non-chess game: text and json work, pgn is refused
        let mut g = Record::new("connect4", 3, "one");
        let p = polytabletop_games::find("connect4").unwrap();
        polytabletop_core::rng::Rng::set_global_seed(Some(3));
        let mut game = (p.make)();
        let m = game.legal_moves()[0].clone();
        g.moves.push(game.play(&m).unwrap());
        assert!(to_text(&g).is_ok() && to_json(&g).is_ok());
        assert!(to_pgn(&g).is_err());
    }

    #[test]
    fn illegal_pgn_is_rejected() {
        assert!(import_pgn("1. e4 e5 2. Ke3 *").is_err());
        assert!(import_pgn("1. e5 *").is_err());
    }
}
