#![allow(unused_imports)]
//! Generic `set_viewer` checks for the hidden-information games (via `GameObj`).
use polytabletop_core::Player;
use polytabletop_games::playable;

const IDS: &[&str] = &[
    "battleship",
    "mastermind",
    "bingo",
    "loteria",
    "mahjong",
    "placute",
    "ceata",
    "umbra",
    "portret",
    "tcg",
    "tcg_evo",
    "tcg_zone",
    "tcg_lant",
    "tcg_cerneala",
    "nave",
    "expeditii",
    "artificii",
    "bufet",
];

fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    let p = playable()
        .into_iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("lipsește {id}"));
    (p.make)()
}

#[test]
fn default_view_is_player_one_and_hotseat_follows_mover() {
    for id in IDS {
        let mut g = make(id);
        let default = g.render();
        g.set_viewer(Some(Player::One));
        assert_eq!(default, g.render(), "{id}: implicit viewer != Some(One)");
        // hot-seat == the mover's view, along a short random-ish playout
        for step in 0..40 {
            if g.outcome().is_some() {
                break;
            }
            g.set_viewer(None);
            let hot = g.render();
            g.set_viewer(Some(g.to_move()));
            assert_eq!(
                hot,
                g.render(),
                "{id}: hot-seat != mover view at step {step}"
            );
            let mv = g.legal_moves();
            if mv.is_empty() {
                break;
            }
            let m = mv[step % mv.len()].clone();
            g.play(&m).unwrap();
        }
    }
}

#[test]
fn the_two_views_differ_somewhere_early() {
    for id in IDS {
        let mut g = make(id);
        let mut differs = false;
        for step in 0..60 {
            if g.outcome().is_some() {
                break;
            }
            g.set_viewer(Some(Player::One));
            let a = g.render();
            g.set_viewer(Some(Player::Two));
            let b = g.render();
            if a != b {
                differs = true;
                break;
            }
            let mv = g.legal_moves();
            if mv.is_empty() {
                break;
            }
            let m = mv[step % mv.len()].clone();
            g.play(&m).unwrap();
        }
        assert!(differs, "{id}: vederile lui J1 și J2 nu diferă niciodată");
    }
}
