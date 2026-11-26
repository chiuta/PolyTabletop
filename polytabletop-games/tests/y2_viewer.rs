#![allow(unused_imports)]
use polytabletop_core::{GameObj, Player};
use polytabletop_games::*;

fn hand_line(r: &str) -> Option<String> {
    r.lines()
        .find(|l| l.starts_with("Mâna ta") || l.starts_with("Piesele tale"))
        .map(|l| l.to_string())
}

/// Hand text without the "(J1)" / "(Jucător 1)" label.
fn hand_body(l: &str) -> String {
    l.split_once(": ")
        .map(|x| x.1.to_string())
        .unwrap_or_default()
}

fn check(name: &str, mut g: Box<dyn GameObj>) {
    // Hot-seat equals the viewer fixed on the mover.
    g.set_viewer(None);
    let first = g.render();
    let mut one = g.clone_box();
    one.set_viewer(Some(g.to_move()));
    assert_eq!(first, one.render(), "{name}: None == Some(mover)");
    // Advance until Player Two is to move and a private-hand line is shown.
    let mut n = 0;
    while !(g.to_move() == Player::Two && hand_line(&g.render()).is_some()) {
        let m = g.legal_moves();
        assert!(!m.is_empty() && g.outcome().is_none(), "{name}: stuck");
        g.play(&m[0]).unwrap();
        n += 1;
        assert!(n < 300, "{name}: Player Two never to move");
    }
    let hot = g.render();
    let mut v1 = g.clone_box();
    v1.set_viewer(Some(Player::One));
    let mut v2 = g.clone_box();
    v2.set_viewer(Some(Player::Two));
    let (h_hot, h1, h2) = (
        hand_body(&hand_line(&hot).unwrap()),
        hand_body(&hand_line(&v1.render()).unwrap()),
        hand_body(&hand_line(&v2.render()).unwrap()),
    );
    assert_eq!(h_hot, h2, "{name}: hot-seat shows the mover's hand");
    assert_ne!(h_hot, h1, "{name}: hot-seat must not show the other hand");
    assert!(!hot.contains(&h1), "{name}: other hand leaked:\n{hot}");
    assert!(v2.render().contains(&h2), "{name}");
    assert!(
        hand_line(&hot).unwrap().contains('2'),
        "{name}: label names J2"
    );
}

macro_rules! t {
    ($n:ident, $e:expr) => {
        #[test]
        fn $n() {
            check(stringify!($n), Box::new($e));
        }
    };
}

t!(tarneeb, tarneeb::Tarneeb::with_seed(3));
t!(baloot, baloot::Baloot::with_seed(3));
t!(tablanet, tablanet::Tablanet::with_seed(3));
t!(truco, truco::Truco::with_seed(3));
t!(trix, trix::Trix::with_seed(3));
t!(briscola, briscola::Briscola::with_seed(3));
t!(scopa, scopa::Scopa::with_seed(3));
t!(durak, durak::Durak::with_seed(3));
t!(sixtysix, sixtysix::SixtySix::with_seed(3));
t!(dou_dizhu, dou_dizhu::DouDizhu::with_seed(3));
t!(teen_patti, teen_patti::TeenPatti::with_seed(3));
t!(pai_gow, pai_gow::PaiGow::with_seed(3));
t!(ganjifa, ganjifa::Ganjifa::with_seed(3));
t!(hanafuda, hanafuda::Hanafuda::with_seed(3));
t!(hwatu, hwatu::with_seed(3));
