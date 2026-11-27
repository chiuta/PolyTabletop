//! C3: jocurile cooperative / cărți / copii (variante originale): partide AI contra AI se termină,
//! mutările AI sunt legale, iar titlurile există în toate limbile.
use polytabletop_core::ai::set_time_budget_override_ms;
use polytabletop_games::playable;

const IDS: &[&str] = &[
    "ritualul_adancului",
    "sigiliul_batran",
    "ceata_tenebrelor",
    "galeria_prabusita",
    "ceasul_templului",
    "scrisori_amestecate",
    "urcus_coboras",
    "paleta_evadarii",
    "ancheta_nocturna",
    "tunelul_blocat",
    "cripta_eroilor",
    "pachetul_aventurierilor",
    "orasul_asediat",
    "fisuri_temporale",
    "camera_cu_ghicitori",
    "zborul_bufnitelor",
    "ogrul_pe_urme",
    "cetele_pustiei",
    "tinutul_ceturilor",
    "garnizoana_cuibului",
    "calatoria_cetei",
    "vanatorii_de_umbre",
    "focar_zero",
    "calea_cavalerului",
    "hotarul_cazut",
    "padurea_juramintelor",
    "atu_si_perechi",
    "randurile_cu_boi",
    "mana_sigura",
    "hipopotamii_lacomi",
    "pioni_prinsi",
    "cursa_cascavalului",
    "bilele_in_tub",
    "livada_cu_cirese",
    "gheata_subtire",
    "avionul_hapsan",
    "livada_si_corbul",
    "gainile_pe_cerc",
    "clopotelul_fructelor",
];

#[test]
fn c3_games_finish_with_legal_ai() {
    set_time_budget_override_ms(Some(5));
    let all = playable();
    for id in IDS {
        let p = all
            .iter()
            .find(|p| p.id == *id)
            .unwrap_or_else(|| panic!("lipsește {id}"));
        let mut wins = 0;
        for k in 0..3u64 {
            let mut g = (p.make)();
            let mut n = 0;
            while g.outcome().is_none() {
                let lm = g.legal_moves();
                assert!(!lm.is_empty(), "{id}: fără mutări");
                let m = g.ai_move(k * 31 + n as u64).expect("AI fără mutare");
                assert!(lm.contains(&m), "{id}: AI ilegal {m}");
                g.play(&m).unwrap();
                n += 1;
                assert!(n < 6000, "{id}: partida nu se termină");
            }
            if g.outcome()
                .is_some_and(|o| format!("{o:?}").contains("Win"))
            {
                wins += 1;
            }
        }
        eprintln!("RATE {id}: {wins}/3 decisive");
    }
}
