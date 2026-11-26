//! Logica de sesiune fără UI: jurnal, replay, determinism.

use crate::gui_i18n::{lookup, tr, trf, Lang, APP_NAME};
use crate::journal::{fnv, Journal};
use crate::markers;
use polytabletop_core::{GameObj, Player};
use polytabletop_games::{Playable, HOTSEAT_SAFE};

/// `--seed N` din linia de comandă (pentru partide reproductibile).
pub fn seed_from_args(args: &[String]) -> Result<Option<u64>, String> {
    match args.iter().position(|a| a == "--seed") {
        None => Ok(None),
        Some(i) => args
            .get(i + 1)
            .and_then(|v| v.parse::<u64>().ok())
            .map(Some)
            .ok_or_else(|| tr("seed_err").to_string()),
    }
}

pub fn who(p: Player) -> &'static str {
    match p {
        Player::One => tr("player1"),
        Player::Two => tr("player2"),
    }
}

pub fn human_str(h: Option<Player>) -> &'static str {
    match h {
        Some(Player::One) => "1",
        Some(Player::Two) => "2",
        None => "ambele",
    }
}

pub fn parse_human(s: &str) -> Option<Player> {
    match s {
        "1" => Some(Player::One),
        "2" => Some(Player::Two),
        _ => None,
    }
}

/// Un joc este reluabil dacă două instanțe noi sunt identice (fără seed ascuns).
pub fn deterministic(p: &Playable) -> bool {
    let a = (p.make)();
    let b = (p.make)();
    a.render() == b.render() && a.legal_moves() == b.legal_moves()
}

/// Jocul își ascunde corect informația privată în modul „la masă”
/// (`render` respectă `set_viewer(None)` = mâna celui la mutare).
pub fn hotseat_hides(p: &Playable) -> bool {
    HOTSEAT_SAFE.contains(&p.id)
}

/// Jocul poate arăta informația privată a Jucătorului 1 în „la masă”:
/// stare aleatoare, fără randare pe privitor. Zarurile singure (Abstract,
/// Clasic, Zaruri) nu contează ca informație privată.
pub fn hotseat_leaks(p: &Playable) -> bool {
    !hotseat_hides(p)
        && !matches!(p.category, "Abstract" | "Clasic" | "Zaruri")
        && !deterministic(p)
}

/// După o mutare în „la masă” cu informație ascunsă: trebuie predat ecranul?
pub fn needs_handover(p: &Playable, human: Option<Player>) -> bool {
    human.is_none() && hotseat_hides(p)
}

/// Poate fi cerută o sugestie? `handover` = ecranul de predare e afișat.
pub fn hint_allowed(p: &Playable, human: Option<Player>, handover: bool) -> Result<(), String> {
    if handover {
        return Err(tr("hint_locked").into());
    }
    if human.is_none() && hotseat_leaks(p) {
        return Err(tr("hint_unavailable").into());
    }
    Ok(())
}

/// Starea pe care se calculează sugestia: o copie cu privitorul pus pe cel la
/// mutare (nu informația adversarului).
pub fn hint_state(game: &dyn GameObj, human: Option<Player>) -> Box<dyn GameObj> {
    let mut g = game.clone_box();
    g.set_viewer(Some(human.unwrap_or_else(|| game.to_move())));
    g
}

/// Setează privitorul: un om (1/2) vede mereu mâna lui; „la masă” = mutatorul.
pub fn apply_viewer(game: &mut dyn GameObj, human: Option<Player>) {
    game.set_viewer(human);
}

/// Titlul ferestrei pentru jocul curent.
pub fn window_title(game_title: &str) -> String {
    format!("{APP_NAME} – {game_title}")
}

/// Cine a făcut ultima mutare, după ultima linie din jurnalul afișat.
pub fn last_mover(log: &[String]) -> Option<Player> {
    let l = log.last()?;
    // jurnalul poate conține linii scrise într-o altă limbă (încărcat / limbă schimbată)
    let starts = |key: &str| {
        Lang::ALL
            .iter()
            .filter_map(|&g| lookup(key, g))
            .any(|w| l.starts_with(w))
    };
    if starts("player1") {
        Some(Player::One)
    } else if starts("player2") {
        Some(Player::Two)
    } else {
        None
    }
}

/// Istoric pentru „Anulează mutarea”: stări clonate (`clone_box`) înaintea fiecărei
/// mutări a omului, împreună cu lungimile jurnalelor.
pub struct Snap {
    pub game: Box<dyn GameObj>,
    pub log: usize,
    pub moves: usize,
}

#[derive(Default)]
pub struct History {
    snaps: Vec<Snap>,
}

impl History {
    const CAP: usize = 1000;

    pub fn push(&mut self, game: &dyn GameObj, log: usize, moves: usize) {
        if self.snaps.len() >= Self::CAP {
            self.snaps.remove(0);
        }
        self.snaps.push(Snap {
            game: game.clone_box(),
            log,
            moves,
        });
    }

    pub fn pop(&mut self) -> Option<Snap> {
        self.snaps.pop()
    }

    pub fn clear(&mut self) {
        self.snaps.clear();
    }

    pub fn len(&self) -> usize {
        self.snaps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.snaps.is_empty()
    }
}

/// Liniile de sub tablă grafică, după joc.
pub fn footer_for(kind: crate::board::Kind, render: &str) -> Vec<String> {
    use crate::board::Kind;
    match kind {
        Kind::Pits => footer_pits(render),
        Kind::Shogi => vec![],
        Kind::Backgammon => {
            let lines: Vec<&str> = render.lines().collect();
            let from = lines
                .iter()
                .position(|l| l.starts_with(markers::BACKGAMMON_BAR))
                .map(|i| i + 1)
                .unwrap_or(lines.len());
            lines[from..]
                .iter()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.trim().to_string())
                .collect()
        }
        Kind::Puncte => render
            .lines()
            .filter(|l| l.starts_with(markers::PUNCTE_BOXES))
            .map(str::to_string)
            .collect(),
        Kind::Nim => render
            .lines()
            .filter(|l| {
                !l.split_once(':')
                    .is_some_and(|(a, _)| a.chars().count() == 1)
            })
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect(),
        Kind::Ludo | Kind::Pachisi => render
            .lines()
            .map(str::trim)
            .filter(|l| {
                !l.is_empty()
                    && !l.starts_with(markers::RACE_TRACK)
                    && !l.starts_with(markers::RACE_LEGEND)
                    && !l.chars().next().is_some_and(|c| c.is_ascii_digit())
            })
            .map(str::to_string)
            .collect(),
        _ => footer(render),
    }
}

/// Ca `footer`, dar pentru Mancala: liniile de după rândul „Jucător 1”.
pub fn footer_pits(render: &str) -> Vec<String> {
    let lines: Vec<&str> = render.lines().collect();
    let from = lines
        .iter()
        .position(|l| l.trim_start().starts_with(markers::PITS_P1))
        .map(|i| i + 1)
        .unwrap_or(lines.len());
    lines[from..]
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .collect()
}

pub fn make_journal(
    p: &Playable,
    human: Option<Player>,
    moves: &[String],
    log: &[String],
    game: &dyn GameObj,
) -> Journal {
    Journal {
        game_id: p.id.to_string(),
        human: human_str(human).to_string(),
        replay: deterministic(p),
        check: fnv(&game.render()),
        moves: moves.to_vec(),
        log: log.to_vec(),
    }
}

pub struct Replayed {
    pub idx: usize,
    pub game: Box<dyn GameObj>,
    pub log: Vec<String>,
    pub human: Option<Player>,
}

pub fn replay(games: &[Playable], j: &Journal) -> Result<Replayed, String> {
    let idx = games
        .iter()
        .position(|g| g.id == j.game_id)
        .ok_or_else(|| trf("unknown_game", &[&j.game_id]))?;
    if !j.replay {
        return Err(trf(
            "not_det",
            &[crate::gui_i18n::game_title(games[idx].id, games[idx].title)],
        ));
    }
    let mut game = (games[idx].make)();
    let mut log = Vec::new();
    for (i, m) in j.moves.iter().enumerate() {
        let me = game.to_move();
        let t = game
            .play(m)
            .map_err(|e| trf("replay_rejected", &[&(i + 1).to_string(), m, &e]))?;
        let line = match j.log.get(i) {
            Some(l) if j.log.len() == j.moves.len() => l.clone(),
            _ => format!("{}: {t}", who(me)),
        };
        log.push(line);
    }
    if j.check != 0 && fnv(&game.render()) != j.check {
        return Err(tr("replay_mismatch").into());
    }
    Ok(Replayed {
        idx,
        game,
        log,
        human: parse_human(&j.human),
    })
}

/// Liniile din `render()` care nu fac parte din tablă (stare, legendă).
pub fn footer(render: &str) -> Vec<String> {
    let lines: Vec<&str> = render.lines().collect();
    let is_board = |l: &str| {
        let t = l.trim();
        let toks: Vec<&str> = t.split_whitespace().collect();
        t.starts_with('|')
            || t.starts_with('+')
            || t.starts_with('~')
            || (toks.len() >= 5 && toks.iter().all(|x| x.chars().count() == 1))
            || (toks
                .first()
                .map(|f| f.parse::<u32>().is_ok())
                .unwrap_or(false)
                && toks.len() >= 2)
    };
    let last = lines.iter().rposition(|l| is_board(l));
    let from = last.map(|i| i + 1).unwrap_or(0);
    lines[from..]
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_games::playable;

    #[test]
    fn chess_replay_roundtrip() {
        let games = playable();
        let i = games.iter().position(|g| g.id == "chess").unwrap();
        assert!(deterministic(&games[i]));
        let mut g = (games[i].make)();
        let mut moves = vec![];
        for m in ["e2e4", "e7e5", "g1f3"] {
            moves.push(g.play(m).unwrap());
        }
        let j = make_journal(&games[i], Some(Player::One), &moves, &[], g.as_ref());
        let j2 = Journal::parse(&j.to_text()).unwrap();
        let r = replay(&games, &j2).unwrap();
        assert_eq!(r.game.render(), g.render());
        assert_eq!(r.log.len(), 3);
        assert_eq!(r.human, Some(Player::One));
    }

    #[test]
    fn bad_move_rejected() {
        let games = playable();
        let j = Journal {
            game_id: "chess".into(),
            human: "1".into(),
            replay: true,
            check: 0,
            moves: vec!["e2e5".into()],
            log: vec![],
        };
        assert!(replay(&games, &j).is_err());
    }

    #[test]
    fn non_replayable_is_export_only() {
        let games = playable();
        let j = Journal {
            game_id: "chess".into(),
            human: "1".into(),
            replay: false,
            check: 0,
            moves: vec![],
            log: vec![],
        };
        assert!(replay(&games, &j).err().unwrap().contains("export"));
    }

    #[test]
    fn footer_lines() {
        let g = (playable()
            .into_iter()
            .find(|p| p.id == "chess")
            .unwrap()
            .make)();
        let f = footer(&g.render());
        assert!(f.iter().any(|l| l.contains("Mută")));
        assert!(!f.iter().any(|l| l.starts_with("8 ")));
    }
}
