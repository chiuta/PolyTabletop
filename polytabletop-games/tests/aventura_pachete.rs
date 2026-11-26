//! Cele 16 pachete `aventura`: catalog, traduceri în 8 limbi, partide complete cu AI prin API-ul public.

use polytabletop_core::i18n::Lang;
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome};
use polytabletop_games::aventura::Aventura;
use polytabletop_games::aventura_pachete::PACKS;
use polytabletop_games::{catalog, find, i18n, Kind};

#[test]
fn every_pack_is_in_the_catalog_registry_and_translated() {
    assert_eq!(PACKS.len(), 16);
    for p in PACKS.iter() {
        let e = catalog()
            .iter()
            .find(|e| e.impl_id == p.id)
            .unwrap_or_else(|| panic!("{} lipsește din catalog", p.id));
        assert_eq!(e.kind, Kind::Implemented, "{}", p.id);
        assert_eq!(e.category, "Rol (RPG)", "{}", p.id);
        let g = find(p.id).unwrap_or_else(|| panic!("{} neînregistrat", p.id));
        assert!(g.title.starts_with(p.title), "{}", p.id);
        for l in Lang::ALL {
            let t = i18n::title(p.id, l).unwrap();
            let r = i18n::rules(p.id, l).unwrap();
            assert!(!t.is_empty() && r.contains("alege N"), "{} {:?}", p.id, l);
            if l == Lang::Ro {
                assert_eq!((t, r), (g.title, g.rules), "{}", p.id);
            }
        }
    }
    // cele 16 intrări RPG originale din catalog au acum aventuri jucabile
    assert!(catalog().iter().all(|e| !(e.category == "Rol (RPG)"
        && e.kind == Kind::Toolkit
        && [
            "Dungeons & Dragons",
            "Call of Cthulhu",
            "Pathfinder",
            "Vampire: The Masquerade",
            "Shadowrun",
            "Mörk Borg",
            "Cyberpunk RED",
            "Tormenta",
            "Starfinder",
            "The Witcher TRPG",
            "InSANe",
        ]
        .contains(&e.name))));
}

#[test]
fn registry_games_finish_with_the_ai_and_are_deterministic() {
    for p in PACKS.iter() {
        let run = |seed: u64| {
            let mut g: Box<dyn GameObj> = Box::new(Aventura::with_seed(p.id, seed));
            let mut rng = Rng::new(seed ^ 77);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.ai_move(rng.next_u64()).expect("mutare AI");
                g.play(&m).expect("mutare legală");
                n += 1;
                assert!(n < 3000, "{} nu se termină", p.id);
            }
            (g.outcome(), g.render(), n)
        };
        let a = run(5);
        assert_eq!(a, run(5), "{} nedeterminist", p.id);
        assert!(matches!(a.0, Some(Outcome::Win(_)) | Some(Outcome::Draw)));
    }
}
