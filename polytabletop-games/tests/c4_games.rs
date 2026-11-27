//! Teste pentru jocurile de copii C4 (variante originale): partide aleatorii, mutări unice, AI legal.

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Player};
use polytabletop_games::playable;

const IDS: &[&str] = &[
    "trezirea_reginelor",
    "gheata_alunecoasa",
    "turnul_rinocerului",
    "turnul_fermei",
    "oua_din_cuib",
    "vulpea_dibace",
    "plansa_cu_poze",
    "memoria_luminilor",
    "catarul_incarcat",
    "butoiasul_corsar",
    "piesa_la_timp",
    "iepuras_galop",
    "balansul_cerdacului",
    "gandacelul_adunat",
    "camara_adormita",
    "piratii_din_golf",
    "cetatea_si_drumul",
    "insula_dragonilor",
    "copii_contra_stafiilor",
    "labirint_nevazut",
    "fantoma_de_pe_turn",
    "trepte_soaptelor",
    "stafia_din_castel",
    "soricei_pe_branza",
    "stafii_comoara",
    "viermi_in_mar",
    "fantoma_fugara",
    "paianjen_pe_fir",
    "satul_din_piatra",
    "zaruri_grabite",
    "pietre_sclipitoare",
    "corabii_din_vale",
    "piramida_bolovani",
    "fantoma_blanda",
    "chei_si_usi",
    "tort_pe_tort",
    "insula_cu_furtuni",
    "parcul_perechilor",
    "purcei_cursa_mare",
];

fn make(id: &str) -> Box<dyn GameObj> {
    (playable()
        .into_iter()
        .find(|p| p.id == id)
        .expect("id")
        .make)()
}

#[test]
fn all_c4_ids_are_registered_and_translated() {
    assert_eq!(IDS.len(), 39);
    for id in IDS {
        let p = playable()
            .into_iter()
            .find(|p| p.id == *id)
            .expect("registered");
        assert_eq!(p.category, "Copii");
        assert!(polytabletop_games::i18n::title(id, polytabletop_core::i18n::Lang::De).is_some());
    }
}

#[test]
fn random_games_finish_with_unique_legal_moves() {
    for id in IDS {
        for seed in 1..=8u64 {
            let mut g = make(id);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{id}: fără mutări\n{}", g.render());
                let mut s = mv.clone();
                s.sort();
                s.dedup();
                assert_eq!(s.len(), mv.len(), "{id}: mutări duplicate");
                g.play(&mv[rng.below(mv.len())]).expect("mutare legală");
                n += 1;
            }
            assert!(g.outcome().is_some(), "{id} nu s-a terminat (seed {seed})");
            assert!(!g.render().is_empty());
        }
    }
}

#[test]
fn garbage_is_rejected_and_ai_is_legal() {
    for id in IDS {
        let mut g = make(id);
        assert!(g.play("zzzz").is_err(), "{id}");
        assert!(g.play("").is_err(), "{id}");
        let mut rng = Rng::new(7);
        let mut n = 0;
        while g.outcome().is_none() && n < 3000 {
            let mv = g.legal_moves();
            let m = if g.to_move() == Player::One {
                let a = g.ai_move(rng.next_u64()).expect("AI fără mutare");
                assert!(mv.contains(&a), "{id}: AI ilegal {a}");
                a
            } else {
                mv[rng.below(mv.len())].clone()
            };
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(g.outcome().is_some(), "{id}: partida cu AI nu s-a terminat");
    }
}
