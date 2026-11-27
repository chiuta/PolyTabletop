//! T5: jocuri de cărți (TCG), variante originale: mână ascunsă (HOTSEAT_SAFE), viewer, AI fără trișat.
use polytabletop_core::{GameObj, Player};
use polytabletop_games::{playable, HOTSEAT_SAFE};

const IDS: &[&str] = &[
    "cristale_chemari",
    "duhuri_pirjol",
    "cursa_prin_retea",
    "chei_chihlimbar",
    "provincii_de_jad",
    "mercenari_stelari",
    "pecetea_magilor",
    "biblioteca_umbrelor",
    "tinutul_cenusiu",
    "cavaleri_mecanici",
    "arena_aliatilor",
    "creaturi_necunoscute",
    "sangele_noptii",
    "misiuni_dincolo_de_stele",
    "lumea_dintre_lumini",
    "drum_peste_munti",
    "pumnul_din_umbra",
    "tinuturi_si_vraji",
    "papusarii_umbrei",
    "povesti_din_ceata",
    "scaunul_cu_colti",
    "trei_cetati",
    "sase_lumi_in_flacari",
    "magii_mici",
    "luptatori_cu_aura",
    "ucenicii_vantului",
    "cosasii_de_suflete",
    "tovarasul_din_gong",
    "zero_contra_infinit",
    "duel_fulger",
    "domeniul_monstrilor",
    "taramuri_paralele",
    "cristale_de_aura",
    "lupta_pe_etape",
    "eroi_contra_raufacatori",
    "roata_anotimpurilor",
    "roboti_de_lupta",
    "statia_dintre_stele",
    "scoala_de_vraji",
    "varful_fumegand",
    "cetatile_din_furtuna",
    "dragoni_la_hotar",
    "ucenicii_cenusii",
    "eroi_de_targ",
    "spartura_dintre_lumi",
    "orasul_prafuit",
];

fn make(id: &str) -> Box<dyn GameObj> {
    let p = playable()
        .into_iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| panic!("lipsește {id}"));
    (p.make)()
}

fn hand_line(r: &str) -> String {
    r.lines()
        .find(|l| l.starts_with("Mâna ta"))
        .unwrap_or_else(|| panic!("fără linia mâinii:\n{r}"))
        .to_string()
}

#[test]
fn all_t5_ids_are_registered_and_hotseat_safe() {
    for id in IDS {
        assert!(HOTSEAT_SAFE.contains(id), "{id}");
        let _ = make(id);
    }
}

#[test]
fn viewer_semantics_and_no_leak() {
    for id in IDS {
        let mut g = make(id);
        let default = g.render();
        g.set_viewer(Some(Player::One));
        assert_eq!(default, g.render(), "{id}: implicit viewer != Some(One)");
        let mut differs = false;
        for step in 0..60 {
            if g.outcome().is_some() {
                break;
            }
            g.set_viewer(None);
            let hot = g.render();
            g.set_viewer(Some(g.to_move()));
            assert_eq!(hot, g.render(), "{id}: hot-seat != mover view");
            let mut a = g.clone_box();
            a.set_viewer(Some(Player::One));
            let mut b = g.clone_box();
            b.set_viewer(Some(Player::Two));
            let (ra, rb) = (a.render(), b.render());
            let (ha, hb) = (hand_line(&ra), hand_line(&rb));
            if ha.split_once(": ").map(|x| x.1) != hb.split_once(": ").map(|x| x.1) {
                differs = true;
            }
            // tot ce nu e linia mâinii este identic pentru ambii privitori (nicio scurgere)
            let strip = |r: &str| {
                r.lines()
                    .filter(|l| !l.starts_with("Mâna ta"))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            assert_eq!(
                strip(&ra),
                strip(&rb),
                "{id}: restul randării depinde de privitor"
            );
            assert!(ha.contains("(J1)") && hb.contains("(J2)"), "{id}");
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{id}");
            let m = mv[(step * 5 + 1) % mv.len()].clone();
            g.play(&m).unwrap();
        }
        assert!(differs, "{id}: cele două vederi nu diferă niciodată");
    }
}

#[test]
fn ai_plays_whole_games_legally() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(6));
    for id in IDS {
        for seed in [1u64, 2] {
            let mut g = make(id);
            let mut n = 0;
            while g.outcome().is_none() {
                let lm = g.legal_moves();
                assert!(!lm.is_empty(), "{id}");
                let m = g.ai_move(seed + n).expect("AI fără mutare");
                assert!(lm.contains(&m), "{id}: mutare ilegală {m}");
                g.play(&m).unwrap();
                n += 1;
                assert!(n < 4000, "{id}: partida nu se termină");
            }
        }
    }
}
