//! `polytabletop-gui --selftest`: verificări headless (fără fereastră).

use crate::board::{self, hit, kind_of, layout, Extra};
use crate::compose::{self, Click};
use crate::journal::Journal;
use crate::menu;
use crate::session;
use eframe::egui::{Pos2, Rect, Vec2};
use polytabletop_core::rng::Rng;
use polytabletop_games::playable;

const GRAPHIC: [&str; 36] = [
    "chess",
    "draughts",
    "reversi",
    "connect4",
    "gomoku",
    "go",
    "morris",
    "xiangqi",
    "makruk",
    "janggi",
    "dara",
    "morabaraba",
    "mancala",
    "sungka",
    "ayo",
    "pallanguzhi",
    "shogi",
    "backgammon",
    "ludo",
    "pachisi",
    "hex",
    "amazoane",
    "linii",
    "konane",
    "fanorona",
    "tigri",
    "surakarta",
    "hasami",
    "puncte",
    "strapungere",
    "izolare",
    "rotire",
    "coridor",
    "drum",
    "tripierd",
    "nim",
];

/// Joacă o mutare întreagă prin click-uri simulate pe coordonate de ecran.
fn play_by_clicks(b: &board::Board, legal: &[String], target: &str) -> Result<String, String> {
    let area = Rect::from_min_size(Pos2::new(5.0, 5.0), Vec2::new(720.0, 640.0));
    let l = layout(b, area);
    let (sqs, _) = compose::tokenize_k(Some(b.kind), target);
    let mut sel: Vec<String> = vec![];
    for (k, s) in sqs.iter().enumerate() {
        let ci = (0..b.cells.len())
            .find(|&i| b.cells[i].name == *s && b.clickable(i))
            .ok_or_else(|| format!("pătrat necunoscut sau neapăsabil {s} din {target}"))?;
        let h = hit(b, &l, l.center(&b.cells[ci])).ok_or("hit gol")?;
        let name = &b.cells[h].name;
        match compose::click_k(Some(b.kind), legal, &sel, name) {
            Click::Play(m) => {
                return if k + 1 == sqs.len() && m == target {
                    Ok(m)
                } else {
                    Err(format!("Play {m} != {target}"))
                }
            }
            Click::Choose(c) => {
                return if c.iter().any(|x| x == target) {
                    Ok(target.to_string())
                } else {
                    Err("Choose fără țintă".into())
                }
            }
            Click::Pending { sel: s2, exact } => {
                if k + 1 == sqs.len() {
                    return match exact {
                        Some(e) if e == target => Ok(e),
                        // capete ambigue (sufixe diferite): butoanele „Termină aici”
                        _ if compose::ends_here_k(Some(b.kind), legal, &s2)
                            .iter()
                            .any(|x| x == target) =>
                        {
                            Ok(target.to_string())
                        }
                        _ => Err(format!("rămas Pending pentru {target}")),
                    };
                }
                sel = s2;
            }
            Click::Reset => return Err(format!("Reset la {name} pentru {target}")),
        }
    }
    Err("fără pătrate".into())
}

type Check<'a> = dyn FnMut(&str, Result<(), String>) -> Result<(), String> + 'a;

fn hotseat_checks(games: &[polytabletop_games::Playable], check: &mut Check) -> Result<(), String> {
    use polytabletop_core::Player;
    use polytabletop_games::HOTSEAT_SAFE;
    let find = |id: &str| games.iter().find(|p| p.id == id);
    // None = mâna celui la mutare; Some(altul) arată altă mână
    let mut tested = 0;
    let mut differs = 0;
    let r: Result<(), String> = (|| {
        for id in HOTSEAT_SAFE {
            let Some(p) = find(id) else { continue };
            let mut g = (p.make)();
            for _ in 0..3 {
                let mover = g.to_move();
                let mut a = g.clone_box();
                a.set_viewer(None);
                let mut b = g.clone_box();
                b.set_viewer(Some(mover));
                let mut c = g.clone_box();
                c.set_viewer(Some(mover.other()));
                // bannerul „Predă ecranul” (linia ════) e singura diferență permisă
                let strip = |r: String| {
                    r.lines()
                        .filter(|l| !l.starts_with('═'))
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                if strip(a.render()) != strip(b.render()) {
                    return Err(format!("{id}: viewer None != viewer mutator"));
                }
                if strip(a.render()) != strip(c.render()) {
                    differs += 1;
                }
                tested += 1;
                if g.outcome().is_some() {
                    break;
                }
                let m = g.legal_moves();
                if m.is_empty() {
                    break;
                }
                g.play(&m[0])?;
            }
        }
        if differs == 0 {
            return Err("nicio mână diferită între privitori".into());
        }
        Ok(())
    })();
    check(
        &format!("la masă: viewer None = mâna mutatorului ({tested} poziții, {differs} cu mâini diferite)"),
        r,
    )?;

    let r: Result<(), String> = (|| {
        let sep = find("septica").ok_or("fără septica")?;
        let chess = find("chess").ok_or("fără chess")?;
        if !session::needs_handover(sep, None) || session::needs_handover(sep, Some(Player::One)) {
            return Err("predare greșită pentru septica".into());
        }
        if session::needs_handover(chess, None) {
            return Err("șahul nu are nevoie de predare".into());
        }
        if session::hint_allowed(sep, None, true).is_ok() {
            return Err("sugestia trebuie blocată pe ecranul de predare".into());
        }
        if session::hint_allowed(sep, None, false).is_err()
            || session::hint_allowed(chess, None, false).is_err()
        {
            return Err("sugestia ar trebui permisă".into());
        }
        let leaky: Vec<&str> = games
            .iter()
            .filter(|p| session::hotseat_leaks(p))
            .map(|p| p.id)
            .collect();
        if leaky.is_empty() {
            return Err("niciun joc marcat ca scurgere".into());
        }
        let p = find(leaky[0]).unwrap();
        if session::hint_allowed(p, None, false).is_ok() {
            return Err(format!("sugestia pentru {} trebuia refuzată", leaky[0]));
        }
        if session::hint_allowed(p, Some(Player::One), false).is_err() {
            return Err("sugestia vs calculator trebuie permisă".into());
        }
        // sugestia se calculează cu privitorul pus pe cel la mutare
        let g = (sep.make)();
        let h = session::hint_state(g.as_ref(), None);
        let mut want = g.clone_box();
        want.set_viewer(Some(g.to_move()));
        let strip = |r: String| {
            r.lines()
                .filter(|l| !l.starts_with('═'))
                .collect::<Vec<_>>()
                .join("\n")
        };
        if strip(h.render()) != strip(want.render()) {
            return Err("hint_state nu folosește mâna mutatorului".into());
        }
        Ok(())
    })();
    check("la masă: predare ecran + sugestie fără scurgeri", r)
}

/// Cele opt table noi (Puncte, Strapungere, Izolare, Rotire, Coridor, Drum, Trei-pierde, Nim):
/// structură, geometrie (hit-test), mutări prin click-uri simulate și rezultatul în tablă.
fn more_boards_checks(
    games: &[polytabletop_games::Playable],
    check: &mut Check,
) -> Result<(), String> {
    use polytabletop_core::Player;
    let make = |id: &str| -> Result<Box<dyn polytabletop_core::GameObj>, String> {
        let p = games
            .iter()
            .find(|p| p.id == id)
            .ok_or(format!("lipsește {id}"))?;
        Ok((p.make)())
    };
    let parse = |id: &str, g: &dyn polytabletop_core::GameObj| -> Result<board::Board, String> {
        board::parse(kind_of(id).ok_or("kind")?, &g.render())
            .map(|b| b.with_mover(g.to_move()))
            .ok_or(format!("parsare {id}"))
    };
    // joacă mutările text date, fiecare prin click-uri pe coordonate
    let play_all = |g: &mut Box<dyn polytabletop_core::GameObj>,
                    id: &str,
                    ms: &[&str]|
     -> Result<(), String> {
        for m in ms {
            let b = parse(id, g.as_ref())?;
            let legal = g.legal_moves();
            let got = play_by_clicks(&b, &legal, m)?;
            g.play(&got)?;
        }
        Ok(())
    };

    // hit-test: centrul fiecărei celule cade pe ea însăși, pe toate cele opt table
    let r: Result<(), String> = (|| {
        for id in [
            "puncte",
            "strapungere",
            "izolare",
            "rotire",
            "coridor",
            "drum",
            "tripierd",
            "nim",
        ] {
            let g = make(id)?;
            let b = parse(id, g.as_ref())?;
            let l = layout(
                &b,
                Rect::from_min_size(Pos2::new(8.0, 9.0), Vec2::new(700.0, 620.0)),
            );
            for (i, c) in b.cells.iter().enumerate() {
                if hit(&b, &l, l.center(c)) != Some(i) {
                    return Err(format!("{id}: hit {}", c.name));
                }
            }
        }
        Ok(())
    })();
    check("8 table noi: hit-test pe centrul fiecărei celule", r)?;

    // Puncte și căsuțe: 36 puncte, 60 laturi apăsabile, 25 căsuțe; partidă întreagă prin click
    let r: Result<(), String> = (|| {
        let mut g = make("puncte")?;
        let b = parse("puncte", g.as_ref())?;
        let dots = b.cells.iter().filter(|c| c.name.starts_with("~d")).count();
        let boxes = b.cells.iter().filter(|c| c.name.starts_with("~b")).count();
        let edges = (0..b.cells.len()).filter(|&i| b.clickable(i)).count();
        if (dots, boxes, edges, b.cells.len()) != (36, 25, 60, 121) {
            return Err(format!("{dots} puncte, {boxes} căsuțe, {edges} laturi"));
        }
        let mut rng = Rng::new(31);
        let mut n = 0;
        while g.outcome().is_none() && n < 200 {
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            let b = parse("puncte", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
            n += 1;
        }
        if n != 60 || g.outcome().is_none() {
            return Err(format!("partida a durat {n} mutări, fără final"));
        }
        let b = parse("puncte", g.as_ref())?;
        let (x, o) = (
            b.cells
                .iter()
                .filter(|c| c.name.starts_with("~b") && c.ch == 'X')
                .count(),
            b.cells
                .iter()
                .filter(|c| c.name.starts_with("~b") && c.ch == 'O')
                .count(),
        );
        if x + o != 25 || x == o {
            return Err(format!("căsuțe X {x} + O {o}"));
        }
        if (0..b.cells.len()).any(|i| b.clickable(i)) {
            return Err("după final nu mai trebuie nicio latură liberă".into());
        }
        Ok(())
    })();
    check(
        "puncte: 36 puncte + 60 laturi + 25 căsuțe, partidă de 60 mutări prin click, căsuțe pe proprietari",
        r,
    )?;

    // Strapungere: 64 celule, 16+16 piese, captură diagonală prin click
    let r: Result<(), String> = (|| {
        let mut g = make("strapungere")?;
        let b = parse("strapungere", g.as_ref())?;
        let cnt = |c| b.cells.iter().filter(|x| x.ch == c).count();
        if (b.cells.len(), cnt('X'), cnt('O')) != (64, 16, 16) {
            return Err("poziția de start".into());
        }
        let mut rng = Rng::new(8);
        let mut caps = 0;
        let mut n = 0;
        while g.outcome().is_none() && n < 300 {
            let legal = g.legal_moves();
            // preferă capturile ca să fie exersate
            let caps_l: Vec<&String> = legal.iter().filter(|m| m.contains('x')).collect();
            let m = if !caps_l.is_empty() && rng.below(2) == 0 {
                caps += 1;
                caps_l[rng.below(caps_l.len())].clone()
            } else {
                legal[rng.below(legal.len())].clone()
            };
            let b = parse("strapungere", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
            n += 1;
        }
        if caps == 0 {
            return Err("nicio captură exersată".into());
        }
        Ok(())
    })();
    check(
        "strapungere: 64 celule, 16 X + 16 O, capturi diagonale prin click",
        r,
    )?;

    // Izolare: mută apoi scoate pătratul (trei click-uri), pătratul scos apare ca '#'
    let r: Result<(), String> = (|| {
        let mut g = make("izolare")?;
        let b = parse("izolare", g.as_ref())?;
        if b.cells.len() != 49 || b.cells[b.find("d1").ok_or("d1")?].ch != 'X' {
            return Err("poziția de start".into());
        }
        play_all(&mut g, "izolare", &["d1-d3/e5", "d7-d5/e4"])?;
        let b = parse("izolare", g.as_ref())?;
        let ch = |n: &str| b.find(n).map(|i| b.cells[i].ch);
        if (ch("d3"), ch("e5"), ch("e4"), ch("d1")) != (Some('X'), Some('#'), Some('#'), Some('.'))
        {
            return Err(format!("tablă: d3 {:?} e5 {:?}", ch("d3"), ch("e5")));
        }
        // fără alegere intermediară: trei pătrate = mutare completă; a doua selecție e doar Pending
        let legal = g.legal_moves();
        match compose::click_k(Some(b.kind), &legal, &[], "d3") {
            Click::Pending { .. } => {}
            x => return Err(format!("{x:?}")),
        }
        Ok(())
    })();
    check(
        "izolare: 49 celule, mutare + scoatere pătrat prin 3 click-uri (#)",
        r,
    )?;

    // Rotire: 36 celule + 8 butoane; piesa apoi butonul cadranului
    let r: Result<(), String> = (|| {
        let mut g = make("rotire")?;
        let b = parse("rotire", g.as_ref())?;
        let buttons = b.cells.iter().filter(|c| c.name.starts_with('q')).count();
        if (b.cells.len(), buttons) != (44, 8) {
            return Err(format!("{} celule, {buttons} butoane", b.cells.len()));
        }
        // buton fără piesă aleasă: nu face nimic
        let legal = g.legal_moves();
        if compose::click_k(Some(b.kind), &legal, &[], "q2d") != Click::Reset {
            return Err("butonul fără celulă trebuia ignorat".into());
        }
        play_all(&mut g, "rotire", &["c3/2d", "b2/1s", "f6/4d"])?;
        let b = parse("rotire", g.as_ref())?;
        let marbles = b.cells.iter().filter(|c| matches!(c.ch, 'X' | 'O')).count();
        if marbles != 3 {
            return Err(format!("{marbles} bile"));
        }
        let mut rng = Rng::new(3);
        for _ in 0..40 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            let b = parse("rotire", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        Ok(())
    })();
    check(
        "rotire: 36 celule + 8 butoane de cadran, mutări 'c3/2d' prin click",
        r,
    )?;

    // Coridor: 81 pătrate + 64 sloturi; zid prin slot (+ alegere h/v), parsarea zidurilor
    let r: Result<(), String> = (|| {
        let mut g = make("coridor")?;
        let b = parse("coridor", g.as_ref())?;
        let slots = b.cells.iter().filter(|c| c.name.starts_with("w-")).count();
        if (b.cells.len(), slots) != (145, 64) {
            return Err(format!("{} celule, {slots} sloturi", b.cells.len()));
        }
        // slotul d4 oferă ambele orientări → alegere; apoi zid vertical pe alt slot, pion, grămezi de ziduri
        let legal = g.legal_moves();
        match compose::click_k(Some(b.kind), &legal, &[], "w-d4") {
            Click::Choose(c) if c.len() == 2 => {}
            x => return Err(format!("slot d4: {x:?}")),
        }
        play_all(
            &mut g,
            "coridor",
            &["d4h", "e5v", "e2", "a1v", "a3v", "a5h", "c5h"],
        )?;
        let b = parse("coridor", g.as_ref())?;
        let Extra::Walls { h, v, left } = &b.extra else {
            return Err("fără ziduri".into());
        };
        let mut hs = h.clone();
        hs.sort();
        let mut vs = v.clone();
        vs.sort();
        if hs != [(0, 4), (2, 4), (3, 3)] || vs != [(0, 0), (0, 2), (4, 4)] {
            return Err(format!("ziduri h {hs:?} v {vs:?}"));
        }
        // 4 ziduri puse de J1 (d4h, a1v?..) — numărătoarea vine din text
        if left.iter().map(|&x| x as u32).sum::<u32>() != 20 - 6 {
            return Err(format!("ziduri rămase {left:?}"));
        }
        if b.cells[b.find("e2").ok_or("e2")?].ch != 'X' {
            return Err("pionul X nu e pe e2".into());
        }
        Ok(())
    })();
    check(
        "coridor: 81 pătrate + 64 sloturi, zid h/v prin click, ziduri citite din tablă",
        r,
    )?;

    // Drum: plasare cu alegere (plat / zid / căpetenie), mutări de stive (direcție + loc de oprire)
    let r: Result<(), String> = (|| {
        let mut g = make("drum")?;
        let b = parse("drum", g.as_ref())?;
        if b.cells.len() != 25 {
            return Err("25 celule".into());
        }
        // prima mutare a fiecăruia: o singură variantă (piesă plată a adversarului) → fără alegere
        play_all(&mut g, "drum", &["c3", "a1"])?;
        let b = parse("drum", g.as_ref())?;
        let legal = g.legal_moves();
        match compose::click_k(Some(b.kind), &legal, &[], "d3") {
            Click::Choose(c) if c.len() == 3 => {}
            x => return Err(format!("d3: {x:?}")),
        }
        let mut rng = Rng::new(21);
        let (mut slides, mut multi, mut placed_other) = (0, 0, 0);
        let mut n = 0;
        while g.outcome().is_none() && n < 150 {
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            if m.contains(['<', '>', '+', '-']) {
                slides += 1;
                if m.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                    multi += 1;
                }
            } else if m.starts_with('s') || m.starts_with("cc") {
                placed_other += 1;
            }
            let b = parse("drum", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
            n += 1;
        }
        if slides == 0 || placed_other == 0 {
            return Err(format!("slides {slides}, zid/căpetenie {placed_other}"));
        }
        let b = parse("drum", g.as_ref())?;
        let tall = match &b.extra {
            Extra::Stacks(s) => s.iter().map(|x| x.chars().count()).max().unwrap_or(0),
            _ => return Err("fără stive".into()),
        };
        if tall < 1 {
            return Err("nicio stivă".into());
        }
        let _ = multi;
        Ok(())
    })();
    check(
        "drum: 25 celule, stive, plasare cu alegere, mutări de stive prin click",
        r,
    )?;

    // Trei-pierde: 61 de hexagoane (rânduri 5..9..5), mutări prin click
    let r: Result<(), String> = (|| {
        let mut g = make("tripierd")?;
        let b = parse("tripierd", g.as_ref())?;
        if b.cells.len() != 61 || b.find("a1").is_none() || b.find("i5").is_none() {
            return Err("61 celule a1..i5".into());
        }
        let rowlen = |r: usize| b.cells.iter().filter(|c| c.row == r).count();
        if (0..9).map(rowlen).collect::<Vec<_>>() != [5, 6, 7, 8, 9, 8, 7, 6, 5] {
            return Err("lungimile rândurilor".into());
        }
        play_all(&mut g, "tripierd", &["e5", "a1", "e6"])?;
        let b = parse("tripierd", g.as_ref())?;
        let ch = |n: &str| b.find(n).map(|i| b.cells[i].ch);
        if (ch("e5"), ch("a1"), ch("e6")) != (Some('X'), Some('O'), Some('X')) {
            return Err("piese".into());
        }
        let mut rng = Rng::new(4);
        while g.outcome().is_none() {
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            let b = parse("tripierd", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        Ok(())
    })();
    check(
        "tripierd: 61 hexagoane, rânduri 5-9-5, partidă prin click până la final",
        r,
    )?;

    // Nim: 18 bețe; click pe un băț ia tot de la el spre dreapta
    let r: Result<(), String> = (|| {
        let mut g = make("nim")?;
        let b = parse("nim", g.as_ref())?;
        if b.cells.len() != 18 {
            return Err(format!("{} bețe", b.cells.len()));
        }
        // al doilea băț (de la stânga) din grămada b (4 bețe) = „b3”
        let i = (0..b.cells.len())
            .find(|&i| b.cells[i].row == 1 && b.cells[i].col == 1)
            .ok_or("băț")?;
        if b.cells[i].name != "b3" || b.hover_names(i).len() != 3 {
            return Err(format!(
                "băț {} hover {:?}",
                b.cells[i].name,
                b.hover_names(i)
            ));
        }
        play_all(&mut g, "nim", &["b3"])?;
        let b = parse("nim", g.as_ref())?;
        if b.cells.len() != 15 || b.cells.iter().filter(|c| c.row == 1).count() != 1 {
            return Err("grămada b trebuia să rămână cu un băț".into());
        }
        let mut rng = Rng::new(12);
        while g.outcome().is_none() {
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            let b = parse("nim", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        if g.outcome() != Some(polytabletop_core::Outcome::Win(Player::One))
            && g.outcome() != Some(polytabletop_core::Outcome::Win(Player::Two))
        {
            return Err("fără câștigător".into());
        }
        Ok(())
    })();
    check(
        "nim: 18 bețe, click pe băț = b3, partidă până la ultimul băț",
        r,
    )?;

    // ultima mutare evidențiată pe tablele noi
    let r: Result<(), String> = (|| {
        let mut g = make("puncte")?;
        let m = g.legal_moves()[0].clone();
        let before = g.to_move();
        g.play(&m)?;
        let b = parse("puncte", g.as_ref())?;
        let v = board::last_cells(&b, &m, before, g.to_move());
        if v.len() != 1 || b.cells[v[0]].name != m {
            return Err("puncte: latura".into());
        }
        let mut g = make("rotire")?;
        g.play("c3/2d")?;
        let b = parse("rotire", g.as_ref())?;
        let names: Vec<&str> = board::last_cells(&b, "c3/2d", Player::One, g.to_move())
            .iter()
            .map(|&i| b.cells[i].name.as_str())
            .collect();
        if names != ["c3", "q2d"] {
            return Err(format!("rotire {names:?}"));
        }
        Ok(())
    })();
    check(
        "ultima mutare: latura la Puncte, piesa + butonul la Rotire",
        r,
    )?;
    Ok(())
}

fn new_boards_checks(
    games: &[polytabletop_games::Playable],
    check: &mut Check,
) -> Result<(), String> {
    use polytabletop_core::Player;
    let make = |id: &str| -> Result<Box<dyn polytabletop_core::GameObj>, String> {
        let p = games
            .iter()
            .find(|p| p.id == id)
            .ok_or(format!("lipsește {id}"))?;
        Ok((p.make)())
    };
    let parse = |id: &str, g: &dyn polytabletop_core::GameObj| -> Result<board::Board, String> {
        board::parse(kind_of(id).ok_or("kind")?, &g.render())
            .map(|b| b.with_mover(g.to_move()))
            .ok_or(format!("parsare {id}"))
    };

    // Shogi: 81 + 14 celule, mâna fără piese nu se apasă, plasare din mână prin click
    let r: Result<(), String> = (|| {
        let g = make("shogi")?;
        let b = parse("shogi", g.as_ref())?;
        if b.cells.len() != 95 {
            return Err(format!("{} celule", b.cells.len()));
        }
        let k = &b.cells[b.find("5i").ok_or("5i")?];
        if k.ch != 'K' {
            return Err("regele lui sente nu e pe 5i".into());
        }
        if b.cells[b.find("5a").ok_or("5a")?].ch != 'k' {
            return Err("regele lui gote nu e pe 5a".into());
        }
        let i = b.find("P*").ok_or("P*")?;
        if b.clickable(i) {
            return Err("mâna goală nu trebuie să fie apăsabilă".into());
        }
        // o partidă scurtă cu captură, apoi plasare din mână, mutate prin click-uri
        let mut g = make("shogi")?;
        let mut played_drop = false;
        for m in ["7g7f", "3c3d", "8h2b+", "3a2b"] {
            let b = parse("shogi", g.as_ref())?;
            let legal = g.legal_moves();
            let got = play_by_clicks(&b, &legal, m)?;
            g.play(&got)?;
        }
        let legal = g.legal_moves();
        if let Some(d) = legal.iter().find(|m| m.contains('*')).cloned() {
            let b = parse("shogi", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &d)?;
            g.play(&got)?;
            played_drop = true;
        }
        if !played_drop {
            return Err("nicio plasare din mână după captură".into());
        }
        Ok(())
    })();
    check(
        "shogi: celule, mâini, promovare '+' și plasare P*5e prin click",
        r,
    )?;

    // Table: 24 puncte + bară + tavă, zaruri, mutări 13/7, bar/.., ./off
    let r: Result<(), String> = (|| {
        let g = make("backgammon")?;
        let b = parse("backgammon", g.as_ref())?;
        if b.cells.len() != 26 {
            return Err(format!("{} celule", b.cells.len()));
        }
        let n = |name: &str| -> Result<i32, String> {
            Ok(b.cells[b.find(name).ok_or(name.to_string())?].num)
        };
        if (n("24")?, n("13")?, n("8")?, n("6")?) != (2, 5, 3, 5) {
            return Err("poziția de start greșită".into());
        }
        match &b.extra {
            board::Extra::Bg { dice, .. } if !dice.is_empty() => {}
            _ => return Err("zaruri lipsă".into()),
        }
        // hit-test pe jumătăți de coloană + bară + tavă
        let l = layout(
            &b,
            Rect::from_min_size(Pos2::new(3.0, 4.0), Vec2::new(900.0, 500.0)),
        );
        for (i, c) in b.cells.iter().enumerate() {
            if hit(&b, &l, l.center(c)) != Some(i) {
                return Err(format!("hit {}", c.name));
            }
        }
        // mutări text complete, prin click-uri, inclusiv bar și off
        let mut rng = Rng::new(77);
        let mut g = make("backgammon")?;
        let (mut saw_bar, mut saw_off) = (false, false);
        for _ in 0..400 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            saw_bar |= m.starts_with("bar");
            saw_off |= m.ends_with("off");
            let b = parse("backgammon", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        let _ = (saw_bar, saw_off);
        Ok(())
    })();
    check(
        "backgammon: 26 celule, start 2/5/3/5, zaruri, hit-test, click-uri",
        r,
    )?;

    // Ludo / Pachisi: piese pe pistă, baza, progres = nume de mutare
    for id in ["ludo", "pachisi"] {
        let r: Result<(), String> = (|| {
            let g = make(id)?;
            let b = parse(id, g.as_ref())?;
            if b.cells.len() != 8 {
                return Err(format!("{} piese (trebuie 8)", b.cells.len()));
            }
            if b.cells.iter().filter(|c| c.name.contains(':')).count() != 8 {
                return Err("toate piesele sunt în bază: nimic apăsabil".into());
            }
            // după câteva mutări apar piese pe pistă, apăsabile, cu nume = progres
            let mut rng = Rng::new(5);
            let mut g = make(id)?;
            let mut clicked = 0;
            for _ in 0..300 {
                if g.outcome().is_some() {
                    break;
                }
                let legal = g.legal_moves();
                let m = legal[rng.below(legal.len())].clone();
                let b = parse(id, g.as_ref())?;
                let played = if compose::is_special(&m) {
                    m
                } else {
                    clicked += 1;
                    play_by_clicks(&b, &legal, &m)?
                };
                g.play(&played)?;
            }
            if clicked == 0 {
                return Err("nicio mutare făcută prin click pe piesă".into());
            }
            Ok(())
        })();
        check(
            &format!("{id}: 8 piese, baza, mutări prin click pe progres"),
            r,
        )?;
    }

    eight_boards_checks(games, check)?;

    // ultima mutare: evidențiere pe toate tipurile de grilă
    let r: Result<(), String> = (|| {
        let mut g = make("chess")?;
        g.play("e2e4")?;
        let b = parse("chess", g.as_ref())?;
        let v = board::last_cells(&b, "e2e4", Player::One, g.to_move());
        let names: Vec<&str> = v.iter().map(|&i| b.cells[i].name.as_str()).collect();
        if names != ["e2", "e4"] {
            return Err(format!("șah: {names:?}"));
        }
        let mut g = make("connect4")?;
        g.play("4")?;
        let b = parse("connect4", g.as_ref())?;
        let v = board::last_cells(&b, "4", Player::One, g.to_move());
        if v.len() != 1 || b.cells[v[0]].row != b.rows - 1 || b.cells[v[0]].ch == '.' {
            return Err("patru în linie: piesa de sus din coloană".into());
        }
        let mut g = make("mancala")?;
        let first = g.legal_moves()[0].clone();
        g.play(&first)?;
        let b = parse("mancala", g.as_ref())?;
        let v = board::last_cells(&b, &first, Player::One, g.to_move());
        if v.len() != 1 || b.cells[v[0]].row != 1 {
            return Err("mancala: groapa jucătorului 1".into());
        }
        // table: după schimbarea perspectivei numerotarea se oglindește (25 - n)
        let mut g = make("backgammon")?;
        let m = g.legal_moves()[0].clone();
        let before = g.to_move();
        g.play(&m)?;
        let b = parse("backgammon", g.as_ref())?;
        let v = board::last_cells(&b, &m, before, g.to_move());
        if v.is_empty() {
            return Err("table: nicio evidențiere".into());
        }
        // shogi: plasările nu evidențiază mâna, mutările evidențiază 2 pătrate
        let mut g = make("shogi")?;
        g.play("7g7f")?;
        let b = parse("shogi", g.as_ref())?;
        if board::last_cells(&b, "7g7f", Player::One, g.to_move()).len() != 2 {
            return Err("shogi: 7g7f".into());
        }
        Ok(())
    })();
    check(
        "ultima mutare: șah, patru în linie, mancala, table, shogi",
        r,
    )?;

    // hover: destinațiile piesei, fără selecție
    let r: Result<(), String> = (|| {
        let g = make("chess")?;
        let legal = g.legal_moves();
        let h = compose::next_squares(&legal, &["g1".to_string()]);
        if h.iter().map(String::as_str).collect::<Vec<_>>() != ["f3", "h3"] {
            return Err(format!("hover g1: {h:?}"));
        }
        let g = make("shogi")?;
        let h = compose::next_squares(&g.legal_moves(), &["7g".to_string()]);
        if h.iter().map(String::as_str).collect::<Vec<_>>() != ["7f"] {
            return Err(format!("hover 7g: {h:?}"));
        }
        Ok(())
    })();
    check(
        "hover/selecție: puncte de destinații legale (Șah, Shogi)",
        r,
    )?;

    // anulare: istoric de stări clonate
    let r: Result<(), String> = (|| {
        let mut g = make("chess")?;
        let mut h = session::History::default();
        let mut renders = vec![g.render()];
        for m in ["e2e4", "e7e5", "g1f3"] {
            h.push(g.as_ref(), 0, 0);
            g.play(m)?;
            renders.push(g.render());
        }
        for k in (0..3).rev() {
            let s = h.pop().ok_or("istoric gol")?;
            if s.game.render() != renders[k] {
                return Err(format!("anulare {k}: poziție diferită"));
            }
        }
        if !h.is_empty() || h.pop().is_some() {
            return Err("istoricul trebuia golit".into());
        }
        // la jocuri cu zaruri, starea clonată păstrează aruncarea (nu se poate „reroll”)
        let mut g = make("backgammon")?;
        let r0 = g.render();
        h.push(g.as_ref(), 0, 0);
        let m = g.legal_moves()[0].clone();
        g.play(&m)?;
        if h.pop().ok_or("gol")?.game.render() != r0 {
            return Err("table: anulare".into());
        }
        Ok(())
    })();
    check(
        "anulare: history clone_box (Șah 3 mutări, Table cu zaruri)",
        r,
    )?;

    let r = if session::window_title("Șah") == "PolyTabletop – Șah" {
        Ok(())
    } else {
        Err("titlu".to_string())
    };
    check("titlul ferestrei conține numele jocului", r)?;
    Ok(())
}

/// Hex, Amazoane, Linii, Konane, Fanorona, Capre și tigri, Surakarta, Hasami.
fn eight_boards_checks(
    games: &[polytabletop_games::Playable],
    check: &mut Check,
) -> Result<(), String> {
    use crate::draw::surakarta_arcs;
    let make = |id: &str| -> Result<Box<dyn polytabletop_core::GameObj>, String> {
        let p = games
            .iter()
            .find(|p| p.id == id)
            .ok_or(format!("lipsește {id}"))?;
        Ok((p.make)())
    };
    let parse = |id: &str, g: &dyn polytabletop_core::GameObj| -> Result<board::Board, String> {
        board::parse(kind_of(id).ok_or("kind")?, &g.render())
            .map(|b| b.with_mover(g.to_move()))
            .ok_or(format!("parsare {id}"))
    };

    // hit-test: fiecare celulă, în mai multe dreptunghiuri de desen; în afara tablei nimic
    for (id, n) in [
        ("hex", 121),
        ("amazoane", 100),
        ("linii", 64),
        ("konane", 64),
        ("fanorona", 45),
        ("tigri", 25),
        ("surakarta", 36),
        ("hasami", 81),
    ] {
        let r: Result<(), String> = (|| {
            let g = make(id)?;
            let b = parse(id, g.as_ref())?;
            if b.cells.len() != n {
                return Err(format!("{} celule, trebuiau {n}", b.cells.len()));
            }
            for (w, h) in [(720.0, 640.0), (300.0, 900.0), (1000.0, 400.0)] {
                let l = layout(
                    &b,
                    Rect::from_min_size(Pos2::new(7.0, 9.0), Vec2::new(w, h)),
                );
                for (i, c) in b.cells.iter().enumerate() {
                    if hit(&b, &l, l.center(c)) != Some(i) {
                        return Err(format!("hit {} la {w}x{h}", c.name));
                    }
                }
                if hit(&b, &l, Pos2::new(-50.0, -50.0)).is_some() {
                    return Err("hit în afara tablei".into());
                }
                // tabla încape în dreptunghiul dat
                for c in &b.cells {
                    let p = l.center(c);
                    if p.x < 7.0 || p.y < 9.0 || p.x > 7.0 + w || p.y > 9.0 + h {
                        return Err(format!("{} în afara ariei", c.name));
                    }
                }
            }
            Ok(())
        })();
        check(
            &format!("{id}: {n} celule, hit-test pe celule și în afară"),
            r,
        )?;
    }

    // Hex: rândurile deplasate, vecini distincți la hit între centre
    let r: Result<(), String> = (|| {
        let g = make("hex")?;
        let b = parse("hex", g.as_ref())?;
        let l = layout(&b, Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 700.0)));
        let c = |n: &str| -> Result<Pos2, String> {
            Ok(l.center(&b.cells[b.find(n).ok_or(n.to_string())?]))
        };
        let (a1, a2, b1) = (c("a1")?, c("a2")?, c("b1")?);
        if (a2.x - a1.x - l.cell / 2.0).abs() > 0.01 || (b1.x - a1.x - l.cell).abs() > 0.01 {
            return Err("deplasare greșită".into());
        }
        // mijlocul dintre a1 și a2 cade pe una dintre ele (nu pe o a treia)
        let mid = a1 + (a2 - a1) * 0.45;
        if hit(&b, &l, mid) != b.find("a1") {
            return Err("hit lângă a1 pe diagonală".into());
        }
        // o partidă de 6 mutări prin click
        let mut g = make("hex")?;
        for _ in 0..6 {
            let b = parse("hex", g.as_ref())?;
            let legal = g.legal_moves();
            let m = legal[legal.len() / 2].clone();
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        Ok(())
    })();
    check(
        "hex: romb 11×11, deplasare ½ celulă, hit pe hexagon, click-uri",
        r,
    )?;

    // Amazoane: mutare + săgeată = trei click-uri; după două încă Pending
    let r: Result<(), String> = (|| {
        let g = make("amazoane")?;
        let legal = g.legal_moves();
        let target = legal[legal.len() / 3].clone();
        let (sq, _) = compose::tokenize(&target);
        if sq.len() != 3 {
            return Err(format!("{target}: {} pătrate", sq.len()));
        }
        let b = parse("amazoane", g.as_ref())?;
        let l = layout(&b, Rect::from_min_size(Pos2::ZERO, Vec2::new(700.0, 700.0)));
        let click_at = |sel: &[String], name: &str| -> Result<Click, String> {
            let i = b.find(name).ok_or(name.to_string())?;
            let h = hit(&b, &l, l.center(&b.cells[i])).ok_or("hit")?;
            Ok(compose::click_k(
                Some(b.kind),
                &legal,
                sel,
                &b.cells[h].name,
            ))
        };
        let c1 = click_at(&[], &sq[0])?;
        let Click::Pending { sel, .. } = c1 else {
            return Err(format!("primul click: {c1:?}"));
        };
        let c2 = click_at(&sel, &sq[1])?;
        let Click::Pending { sel, exact: None } = c2 else {
            return Err(format!("al doilea click trebuia să rămână Pending: {c2:?}"));
        };
        // destinațiile săgeții sunt evidențiate
        if compose::next_squares(&legal, &sel).is_empty() {
            return Err("fără destinații pentru săgeată".into());
        }
        match click_at(&sel, &sq[2])? {
            Click::Play(m) if m == target => {}
            x => return Err(format!("al treilea click: {x:?}")),
        }
        // săgeata nu poate fi plasată pe un pătrat ocupat de altă piesă
        let occupied = b
            .cells
            .iter()
            .find(|c| c.ch == 'O')
            .map(|c| c.name.clone())
            .ok_or("fără O")?;
        if compose::next_squares(&legal, &sel).contains(&occupied) {
            return Err("săgeată pe pătrat ocupat".into());
        }
        // un joc scurt: pătratele cu săgeți (#) apar în tablă
        let mut g = make("amazoane")?;
        let mut arrows = 0;
        for _ in 0..8 {
            let b = parse("amazoane", g.as_ref())?;
            let legal = g.legal_moves();
            let m = legal[legal.len() / 2].clone();
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
            arrows = parse("amazoane", g.as_ref())?
                .cells
                .iter()
                .filter(|c| c.ch == '#')
                .count();
        }
        if arrows != 8 {
            return Err(format!("{arrows} săgeți după 8 mutări"));
        }
        Ok(())
    })();
    check(
        "amazoane: 3 click-uri (piesă, destinație, săgeată), săgeți # pe tablă",
        r,
    )?;

    // Linii de acțiune: capturi cu 'x' prin click
    let r: Result<(), String> = (|| {
        let mut rng = Rng::new(31);
        let mut g = make("linii")?;
        let mut caps = 0;
        for _ in 0..120 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            caps += m.contains('x') as u32;
            let b = parse("linii", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        if caps == 0 {
            return Err("nicio captură în 120 de mutări".into());
        }
        Ok(())
    })();
    check("linii: mutări și capturi (x) prin click-uri", r)?;

    // Konane: deschidere cu `remove`, apoi sărituri
    let r: Result<(), String> = (|| {
        let mut g = make("konane")?;
        let legal = g.legal_moves();
        if !legal.iter().all(|m| m.starts_with("remove ")) {
            return Err("deschiderea trebuie să fie remove".into());
        }
        let b = parse("konane", g.as_ref())?;
        let got = play_by_clicks(&b, &legal, &legal[0])?;
        g.play(&got)?;
        let legal = g.legal_moves();
        let b = parse("konane", g.as_ref())?;
        let got = play_by_clicks(&b, &legal, &legal[0])?;
        g.play(&got)?;
        let mut rng = Rng::new(3);
        let mut jumps = 0;
        for _ in 0..40 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            jumps += m.contains('-') as u32;
            let b = parse("konane", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        if jumps == 0 {
            return Err("nicio săritură".into());
        }
        Ok(())
    })();
    check("konane: remove la deschidere, apoi sărituri prin click", r)?;

    // Fanorona: lanțuri de capturi (≥3 pătrate) și alegerea abordare/retragere
    let r: Result<(), String> = (|| {
        let mut rng = Rng::new(11);
        let (mut chains, mut choose) = (0, 0);
        for _ in 0..6 {
            let mut g = make("fanorona")?;
            for _ in 0..80 {
                if g.outcome().is_some() {
                    break;
                }
                let legal = g.legal_moves();
                let m = legal[rng.below(legal.len())].clone();
                let (sq, _) = compose::tokenize(&m);
                chains += (sq.len() >= 3) as u32;
                let same = legal
                    .iter()
                    .filter(|x| compose::tokenize(x).0 == sq)
                    .count();
                choose += (same > 1) as u32;
                let b = parse("fanorona", g.as_ref())?;
                let got = play_by_clicks(&b, &legal, &m)?;
                g.play(&got)?;
            }
        }
        if chains == 0 {
            return Err("niciun lanț de capturi întâlnit".into());
        }
        let _ = choose;
        Ok(())
    })();
    check("fanorona: 9×5, lanțuri de capturi prin click-uri", r)?;

    // Capre și tigri: faza de plasare (un click), apoi mutări
    let r: Result<(), String> = (|| {
        let mut g = make("tigri")?;
        let legal = g.legal_moves();
        let (placing, _) = compose::split_special_k(None, &legal);
        if placing.is_empty() || !legal.iter().any(|m| compose::tokenize(m).0.len() == 1) {
            return Err("faza de plasare nu are mutări cu un singur pătrat".into());
        }
        let mut rng = Rng::new(17);
        let (mut one, mut two) = (0, 0);
        for _ in 0..120 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            match compose::tokenize(&m).0.len() {
                1 => one += 1,
                _ => two += 1,
            }
            let b = parse("tigri", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        if one == 0 || two == 0 {
            return Err(format!("plasări {one}, mutări {two}"));
        }
        Ok(())
    })();
    check(
        "tigri: plasare (1 click) apoi mutări/capturi (2 click-uri)",
        r,
    )?;

    // Surakarta: arcurile leagă capetele liniilor 2/5 și 3/4
    let r: Result<(), String> = (|| {
        let g = make("surakarta")?;
        let b = parse("surakarta", g.as_ref())?;
        let l = layout(&b, Rect::from_min_size(Pos2::ZERO, Vec2::new(720.0, 640.0)));
        let arcs = surakarta_arcs(&b, &l);
        if arcs.len() != 8 || arcs.iter().filter(|a| a.big).count() != 4 {
            return Err(format!("{} arcuri", arcs.len()));
        }
        let c = |col: usize, row: usize| -> Pos2 {
            l.center(
                &b.cells[b
                    .cells
                    .iter()
                    .position(|x| x.col == col && x.row == row)
                    .unwrap()],
            )
        };
        let near = |a: Pos2, b: Pos2| (a - b).length() < 0.01;
        // fiecare arc începe pe o linie și se termină pe o linie perpendiculară
        // din același circuit; capetele sunt chiar punctele grilei de pe margine
        let mut ok = 0;
        for a in &arcs {
            let k = if a.big { 2 } else { 1 };
            let (s, e) = (a.start(), a.end());
            let ends = [
                (c(0, k), c(k, 0)),
                (c(5, k), c(5 - k, 0)),
                (c(0, 5 - k), c(k, 5)),
                (c(5, 5 - k), c(5 - k, 5)),
            ];
            if ends
                .iter()
                .any(|&(p, q)| (near(s, p) && near(e, q)) || (near(s, q) && near(e, p)))
            {
                ok += 1;
            }
            // arcul stă în afara tablei (colțul e centrul)
            if ((s - a.ctr).length() - a.r).abs() > 0.01 {
                return Err("rază".into());
            }
        }
        if ok != 8 {
            return Err(format!("doar {ok} din 8 arcuri se leagă de grilă"));
        }
        // piese 12 + 12 și o partidă prin click-uri
        if b.cells.iter().filter(|c| c.ch == 'O').count() != 12 {
            return Err("poziție de start".into());
        }
        let mut rng = Rng::new(23);
        let mut g = make("surakarta")?;
        let mut caps = 0;
        for _ in 0..150 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            caps += m.contains('x') as u32;
            let b = parse("surakarta", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        let _ = caps;
        Ok(())
    })();
    check(
        "surakarta: 8 arcuri de colț se leagă exact de liniile 2/3/4/5, click-uri",
        r,
    )?;

    // Hasami: mutări glisante prin click
    let r: Result<(), String> = (|| {
        let mut rng = Rng::new(41);
        let mut g = make("hasami")?;
        for _ in 0..100 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.legal_moves();
            let m = legal[rng.below(legal.len())].clone();
            let b = parse("hasami", g.as_ref())?;
            let got = play_by_clicks(&b, &legal, &m)?;
            g.play(&got)?;
        }
        Ok(())
    })();
    check("hasami: 9×9, mutări glisante prin click-uri", r)?;
    Ok(())
}

pub fn run() -> Result<(), String> {
    let games = playable();
    let mut ok = 0usize;
    let mut check = |name: &str, r: Result<(), String>| -> Result<(), String> {
        match r {
            Ok(()) => {
                ok += 1;
                println!("ok   {name}");
                Ok(())
            }
            Err(e) => {
                println!("FAIL {name}: {e}");
                Err(format!("{name}: {e}"))
            }
        }
    };

    // 1. meniu
    let items: Vec<(&str, &str, &str)> =
        games.iter().map(|g| (g.id, g.title, g.category)).collect();
    let all = menu::groups(&items, "");
    check(
        "meniu: toate jocurile în categorii",
        if all.iter().map(|(_, v)| v.len()).sum::<usize>() == games.len() && all.len() > 3 {
            Ok(())
        } else {
            Err("număr greșit".into())
        },
    )?;
    let f = menu::groups(&items, "sah");
    check(
        "meniu: căutare fără diacritice (sah → Șah)",
        if f.iter()
            .any(|(_, v)| v.iter().any(|&i| games[i].id == "chess"))
        {
            Ok(())
        } else {
            Err("lipsă".into())
        },
    )?;

    // 2. tablele: parsare + jocuri complete prin click-uri
    let mut rng = Rng::new(20261125);
    for id in GRAPHIC {
        let p = games
            .iter()
            .find(|p| p.id == id)
            .ok_or(format!("lipsește jocul {id}"))?;
        let kind = kind_of(id).unwrap();
        let mut g = (p.make)();
        let mut plies = 0;
        let r: Result<(), String> = (|| {
            while g.outcome().is_none() && plies < 160 {
                let b = board::parse(kind, &g.render())
                    .ok_or("parsare eșuată")?
                    .with_mover(g.to_move());
                let legal = g.legal_moves();
                let (clickable, special) = compose::split_special_k(Some(kind), &legal);
                let pick = if (!special.is_empty() && rng.below(6) == 0) || clickable.is_empty() {
                    special[rng.below(special.len())].clone()
                } else {
                    clickable[rng.below(clickable.len())].clone()
                };
                let played = if compose::is_special_k(Some(kind), &pick) {
                    pick.clone()
                } else {
                    play_by_clicks(&b, &legal, &pick)?
                };
                g.play(&played)?;
                plies += 1;
            }
            Ok(())
        })();
        check(&format!("{id}: parsare + {plies} mutări prin click-uri"), r)?;
    }

    // 2a. plăci noi: structură, evidențiere ultima mutare, hover, anulare, titlu
    new_boards_checks(&games, &mut check)?;
    more_boards_checks(&games, &mut check)?;

    // 2b. „la masă”: privitor, ecran de predare, sugestie
    hotseat_checks(&games, &mut check)?;

    // 3. jurnal + replay
    let ci = games.iter().position(|g| g.id == "chess").unwrap();
    let mut g = (games[ci].make)();
    let mut moves = vec![];
    for _ in 0..30 {
        if let Some(m) = g.ai_move(rng.next_u64()) {
            moves.push(g.play(&m)?);
        }
    }
    let j = session::make_journal(&games[ci], None, &moves, &[], g.as_ref());
    let j2 = Journal::parse(&j.to_text())?;
    let r = session::replay(&games, &j2)?;
    check(
        "jurnal: salvare → text → replay identic (Șah, 30 mutări)",
        if r.game.render() == g.render() {
            Ok(())
        } else {
            Err("poziții diferite".into())
        },
    )?;
    let nondet = games.iter().filter(|p| !session::deterministic(p)).count();
    check(
        "jurnal: există jocuri nedeterministe → doar export",
        if nondet > 0 {
            Ok(())
        } else {
            Err("niciunul detectat".into())
        },
    )?;
    println!(
        "determinist: {} din {} jocuri",
        games.len() - nondet,
        games.len()
    );
    println!("SELFTEST OK ({ok} verificări)");
    Ok(())
}
