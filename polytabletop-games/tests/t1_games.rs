//! Lista T1 (RPG, variante originale): partide complete prin API-ul public, mutări unice, AI legal, catalog.
use polytabletop_core::rng::Rng;
use polytabletop_core::GameObj;
use polytabletop_games::{catalog, find, Kind};

const IDS: &[&str] = &[
    "curierii_stelelor",
    "fisa_fara_clase",
    "runa_si_lama",
    "saga_in_graba",
    "aspectul_destinului",
    "pustiul_cenusiu",
    "hrube_cu_comori",
    "pivnita_strabuna",
    "palnia_mortilor",
    "nava_fara_raspuns",
    "faclia_stinsa",
    "mutantii_zonei",
    "ceremonia_sabiei",
    "paradoxul_trezit",
    "furia_haitei",
    "ordinul_luminii",
    "pragul_dificultatii",
    "ruinele_eonilor",
    "vara_masinilor",
    "padurea_coruptiei",
    "iluzia_cenusie",
    "tinutul_uitat",
    "cargoul_din_abis",
    "carti_si_gloante",
    "trepte_de_destin",
    "clona_credincioasa",
    "planul_scapat",
    "jurament_si_obstacol",
    "galerii_cu_dubluri",
    "cavaler_si_magician",
    "tabele_fara_sfarsit",
    "puncte_nu_clase",
    "lupta_cu_pelerine",
    "tabara_regelui",
    "spadasinii_marilor",
    "aurul_solar",
    "echipajul_exploratorilor",
    "inchizitorul_umbrei",
    "contrabandistii_cetii",
    "legiunea_in_retragere",
    "inimi_din_umbra",
    "tineri_cu_pelerine",
    "biciclete_si_taine",
    "juramant_de_otel",
    "lumina_care_scade",
    "turnul_clatinat",
];

#[test]
fn t1_games_are_in_the_catalog_as_implemented() {
    for id in IDS {
        let e = catalog()
            .iter()
            .find(|e| e.impl_id == *id)
            .unwrap_or_else(|| panic!("{id} lipsește din catalog"));
        assert_eq!(e.kind, Kind::Implemented, "{id}");
        assert_eq!(e.category, "Rol (RPG)", "{id}");
        assert!(find(id).is_some(), "{id} neînregistrat");
    }
}

#[test]
fn t1_random_and_ai_games_finish_with_unique_moves() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(6));
    for id in IDS {
        for seed in 1..=4u64 {
            let mut g: Box<dyn GameObj> = (find(id).unwrap().make)();
            let mut rng = Rng::new(seed * 31);
            let mut n = 0;
            while g.outcome().is_none() && n < 1500 {
                let lm = g.legal_moves();
                assert!(
                    !lm.is_empty(),
                    "{id}: fără mutări
{}",
                    g.render()
                );
                let mut s = lm.clone();
                s.sort();
                s.dedup();
                assert_eq!(s.len(), lm.len(), "{id}: mutări duplicate");
                let m = if seed % 2 == 0 && n % 2 == 0 {
                    let a = g.ai_move(rng.next_u64()).expect("AI");
                    assert!(lm.contains(&a), "{id}: AI ilegal {a}");
                    a
                } else {
                    lm[rng.below(lm.len())].clone()
                };
                g.play(&m).expect("mutare legală");
                n += 1;
            }
            assert!(
                g.outcome().is_some(),
                "{id}: partida nu s-a terminat
{}",
                g.render()
            );
            assert!(g.render().contains("PV"), "{id}");
        }
    }
}
