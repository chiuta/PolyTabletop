#![allow(unused_imports)]
use polytabletop_core::rng::Rng;
use polytabletop_rpg::chargen::{self, GameSystem};
use polytabletop_rpg::cli::Session;
use polytabletop_rpg::combat::{Battle, Terrain, Unit};
use polytabletop_rpg::encounter::{self, Difficulty};
use polytabletop_rpg::gen::{GenLib, Rarity};
use polytabletop_rpg::journal::Journal;
use polytabletop_rpg::overland::{self, HexMap, Season};

fn run(s: &mut Session, l: &str) -> String {
    s.exec(l).unwrap_or_else(|e| panic!("{l}: {e}")).text
}

fn demo() -> Battle {
    Battle::parse(include_str!("../data/battle-demo.toml")).unwrap()
}

#[test]
fn battle_is_deterministic_and_conserves() {
    let (mut b1, mut b2) = (demo(), demo());
    let o1 = b1.run(&mut Rng::new(42));
    let o2 = b2.run(&mut Rng::new(42));
    assert_eq!(o1, o2);
    assert_eq!(b1.units, b2.units);
    for u in &b1.units {
        assert!(u.count <= u.start);
    }
    assert!(o1.rounds >= 1 && o1.rounds <= 25);
    assert!(b1.report(&o1).contains("runde"));
}

#[test]
fn battle_stronger_side_usually_wins_and_morale_breaks() {
    let mut wins = 0;
    for seed in 0..20 {
        let units = vec![
            Unit::new("big", 0, 200, 5, 4, 8),
            Unit::new("small", 1, 40, 3, 3, 4),
        ];
        let mut b = Battle::new("A", "B", units, 30);
        let o = b.run(&mut Rng::new(seed));
        if o.winner == Some(0) {
            wins += 1;
        }
        // the weak side must be broken (routed or dead) by the end
        assert!(b.strength(1) == 0 || b.round == 30);
    }
    assert!(wins >= 18, "wins = {wins}");
}

#[test]
fn battle_terrain_matters() {
    let mut open_loss = 0u32;
    let mut fort_loss = 0u32;
    for seed in 0..30 {
        for (t, acc) in [
            (Terrain::Open, &mut open_loss),
            (Terrain::Fort, &mut fort_loss),
        ] {
            let mut d = Unit::new("d", 1, 100, 3, 3, 100);
            d.terrain = t;
            let mut b = Battle::new("A", "B", vec![Unit::new("a", 0, 100, 4, 3, 100), d], 3);
            b.run(&mut Rng::new(seed));
            *acc += 100 - b.units[1].count;
        }
    }
    assert!(fort_loss < open_loss, "{fort_loss} !< {open_loss}");
}

#[test]
fn battle_parse_errors_and_cli() {
    assert!(Battle::parse("[battle]\n").is_err());
    assert!(Battle::parse("[unit.x]\nside = \"c\"\n").is_err());
    assert!(
        Battle::parse("[unit.x]\nside = \"a\"\nterrain = \"lava\"\n[unit.y]\nside = \"b\"\n")
            .is_err()
    );
    let mut s = Session::new(5);
    let o = run(&mut s, "battle demo rounds=5");
    assert!(o.contains("Garda Văii"));
    let again = {
        let mut s2 = Session::new(5);
        run(&mut s2, "battle demo rounds=5")
    };
    assert_eq!(o, again);
    assert_eq!(s.journal.of_kind("battle").len(), 1);
}

#[test]
fn chargen_all_builtin_systems() {
    for (id, _) in chargen::BUILTIN {
        let sys = chargen::builtin(id).unwrap();
        let a = sys.generate("X", &mut Rng::new(7)).unwrap();
        let b = sys.generate("X", &mut Rng::new(7)).unwrap();
        assert_eq!(a.to_text(), b.to_text(), "{id}");
        for at in &sys.attributes {
            assert!(a.attr(at).is_some(), "{id}: {at}");
        }
        let c = sys.generate("X", &mut Rng::new(8)).unwrap();
        assert_ne!(a.to_text(), c.to_text(), "{id}");
    }
}

#[test]
fn chargen_system_rules_hold() {
    let pc = chargen::builtin("percentile").unwrap();
    let sh = pc.generate("P", &mut Rng::new(1)).unwrap();
    for a in &pc.attributes {
        let v = sh.attr(a).unwrap();
        assert!((15..=90).contains(&v) && v % 5 == 0);
    }
    for (n, base) in &pc.skills {
        let v = sh.skill(n).unwrap();
        assert!(v >= *base && v <= pc.skill_cap);
    }
    let pool = chargen::builtin("grit-pool").unwrap();
    let sh = pool.generate("Q", &mut Rng::new(2)).unwrap();
    let total: i32 = pool.attributes.iter().map(|a| sh.attr(a).unwrap()).sum();
    assert_eq!(total, 6 + 9);
    let d20 = chargen::builtin("d20-heroic").unwrap();
    let sh = d20.generate("D", &mut Rng::new(3)).unwrap();
    assert_eq!(
        d20.skills
            .iter()
            .filter(|(n, _)| sh.skill(n).is_some())
            .count(),
        5
    );
    assert!(sh.resource("Hit points").unwrap().max >= 1);
}

#[test]
fn chargen_custom_system_and_errors() {
    let src = "[system]\nid=\"mini\"\nattributes=[\"A\",\"B\"]\nattr_method=\"spread\"\nattr_points=3\nattr_min=1\nattr_max=3\nskills=[\"X:1\",\"Y\"]\nskill_points=4\nskill_cap=3\nresources=[\"HP:1d4+@A\"]\n";
    let s = GameSystem::parse(src).unwrap();
    let sh = s.generate("m", &mut Rng::new(1)).unwrap();
    assert_eq!(sh.attr("A").unwrap() + sh.attr("B").unwrap(), 5);
    assert!(GameSystem::parse("[system]\nattributes=[]\n").is_err());
    assert!(GameSystem::parse("[system]\nattributes=[\"A\"]\nattr_method=\"x\"\n").is_err());
    assert!(
        GameSystem::parse("[system]\nattributes=[\"A\"]\nresources=[\"HP:1d4+@Z\"]\n").is_err()
    );
    assert!(GameSystem::parse(
        "[system]\nattributes=[\"A\"]\nattr_method=\"spread\"\nattr_points=99\nattr_max=3\n"
    )
    .is_err());
    let mut ses = Session::new(3);
    let o = run(&mut ses, "chargen percentile Ana");
    assert!(o.contains("Ana") && o.contains("d100-under"));
    assert!(ses.sheet.is_some());
    assert!(run(&mut ses, "chargen list").contains("star-narrative"));
}

#[test]
fn generators_respect_rarity_and_seed() {
    let lib = GenLib::builtin();
    let a = lib.spell(Rarity::Common, &mut Rng::new(9)).unwrap();
    let b = lib.spell(Rarity::Common, &mut Rng::new(9)).unwrap();
    assert_eq!(a, b);
    for seed in 0..100 {
        let mut r = Rng::new(seed);
        assert!(lib.spell(Rarity::Rare, &mut r).unwrap().rarity >= Rarity::Rare);
        assert!(lib.ability(Rarity::Epic, &mut r).unwrap().rarity >= Rarity::Epic);
        assert!(lib.item(Rarity::Uncommon, &mut r).unwrap().rarity >= Rarity::Uncommon);
    }
    // frequency follows weights
    let mut r = Rng::new(1);
    let (mut common, mut legend) = (0, 0);
    for _ in 0..3000 {
        match lib.ability(Rarity::Common, &mut r).unwrap().rarity {
            Rarity::Common => common += 1,
            Rarity::Legendary => legend += 1,
            _ => {}
        }
    }
    assert!(common > 1000 && legend == 0);
    assert!(lib.spell(Rarity::Legendary, &mut r).is_some());
    assert!(GenLib::parse("[spell.x]\nrarity = \"mythic\"\n").is_err());
    let mut s = Session::new(1);
    assert!(run(&mut s, "gen item 3 min=rare").lines().count() == 3);
    assert!(s.exec("gen potion").is_err());
}

#[test]
fn encounter_budget_and_build() {
    assert_eq!(encounter::budget(4, 5, Difficulty::Medium), 200.0);
    assert!(encounter::group_cost(&[1.0, 1.0]) > 2.0 * encounter::foe_cost(1.0));
    assert_eq!(encounter::rate(&[1.0], 4, 5), None);
    assert_eq!(
        encounter::rate(&[7.0, 7.0, 5.0], 4, 3),
        Some(Difficulty::Deadly)
    );
    let pool = encounter::builtin_foes();
    for seed in 0..50 {
        let bud = encounter::budget(4, 3, Difficulty::Hard);
        let pick = encounter::build(&pool, bud, 8, &mut Rng::new(seed));
        let crs: Vec<f64> = pick.iter().map(|&i| pool[i].cr).collect();
        assert!(encounter::group_cost(&crs) <= bud);
        assert!(pick.len() <= 8);
    }
    let mut s = Session::new(2);
    assert!(run(&mut s, "encounter budget 4 3").contains("deadly"));
    assert!(run(&mut s, "encounter build 4 3 medium").contains("cost"));
    assert!(run(&mut s, "encounter rate 4 3 0.5 0.5").contains("->"));
    assert!(s.exec("encounter build 4 3 nasty").is_err());
}

#[test]
fn loot_split_is_exact_and_fair() {
    let lib = GenLib::builtin();
    for seed in 0..30 {
        let mut r = Rng::new(seed);
        let t = encounter::split_random(1000, 5, &mut r);
        assert_eq!(t.iter().sum::<i64>(), 1000);
        let h = encounter::make_hoard(2000, &lib, &mut r);
        let worth: i64 = h.coins + h.items.iter().map(|i| i.value).sum::<i64>();
        assert_eq!(worth, 2000);
        let names: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        let d = encounter::distribute(&h, &names, &mut r);
        assert_eq!(d.iter().map(|x| x.1).sum::<i64>(), h.coins);
        assert_eq!(d.iter().map(|x| x.2.len()).sum::<usize>(), h.items.len());
    }
    let mut s = Session::new(4);
    let o = run(&mut s, "loot 900 Ana Bo");
    assert!(o.contains("Ana:") && o.contains("Bo:"));
    assert!(run(&mut s, "loot 300 3").contains("PC3"));
    assert!(s.exec("loot -5").is_err());
}

#[test]
fn hexmap_paths_weather_travel() {
    let m1 = HexMap::generate(20, 12, &mut Rng::new(11));
    let m2 = HexMap::generate(20, 12, &mut Rng::new(11));
    assert_eq!(m1, m2);
    assert_eq!(m1.render(&[]).lines().count(), 12 + 1);
    // find two land hexes with a path and verify cost >= hex count
    let land: Vec<(usize, usize)> = (0..12)
        .flat_map(|y| (0..20).map(move |x| (x, y)))
        .filter(|&(x, y)| m1.at(x, y).cost().is_some())
        .collect();
    assert!(!land.is_empty());
    let mut found = false;
    for &to in land.iter().skip(1).take(40) {
        if let Some((p, c)) = m1.path(land[0], to) {
            found = true;
            assert_eq!(p[0], land[0]);
            assert_eq!(*p.last().unwrap(), to);
            assert!(c as usize >= p.len() - 1);
            for w in p.windows(2) {
                assert!(m1.neighbors(w[0].0, w[0].1).contains(&w[1]));
            }
        }
    }
    assert!(found);
    assert!(m1.path((0, 0), (99, 99)).is_none());
    let j1 = overland::travel(20, 4, Season::Winter, &mut Rng::new(3));
    let j2 = overland::travel(20, 4, Season::Winter, &mut Rng::new(3));
    assert_eq!(j1, j2);
    assert!(j1.days >= 5 && j1.weather.len() as u32 == j1.days);
    assert_eq!(
        overland::travel(0, 4, Season::Spring, &mut Rng::new(1)).days,
        0
    );
    // summer never snows, winter never heat
    let mut r = Rng::new(5);
    for _ in 0..500 {
        assert_ne!(
            overland::Weather::roll(Season::Summer, &mut r),
            overland::Weather::Snow
        );
        assert_ne!(
            overland::Weather::roll(Season::Winter, &mut r),
            overland::Weather::Heat
        );
    }
}

#[test]
fn hexmap_cli() {
    let mut s = Session::new(21);
    assert!(s.exec("travel 0 0 1 1").is_err());
    let o = run(&mut s, "hexmap 16 10");
    assert!(o.contains("apă"));
    let m = s.ext.map.clone().unwrap();
    let land: Vec<(usize, usize)> = (0..m.h)
        .flat_map(|y| (0..m.w).map(move |x| (x, y)))
        .filter(|&(x, y)| m.at(x, y).cost().is_some())
        .collect();
    let (a, b) = (land[0], land[land.len() - 1]);
    run(&mut s, "season winter");
    let r = s.exec(&format!("travel {} {} {} {}", a.0, a.1, b.0, b.1));
    if let Ok(o) = r {
        assert!(o.text.contains("zile"));
    }
    assert!(run(&mut s, "weather summer").contains('%'));
    assert!(s.exec("season monsoon").is_err());
}

#[test]
fn journal_markdown_export() {
    let mut j = Journal::new();
    j.add("roll", "2d6: 7 | *bold*");
    j.add("note", "linia 1\nlinia 2 [x]");
    let md = j.to_markdown("Campanie #1", None);
    assert!(md.starts_with("# Campanie \\#1\n"));
    assert!(md.contains("| roll | 1 |"));
    assert!(md.contains("1. **roll** — 2d6: 7 \\| \\*bold\\*"));
    assert!(md.contains("\n   linia 2 \\[x\\]\n"));
    assert!(Journal::new().to_markdown("T", None).contains("_(gol)_"));
    let mut s = Session::new(1);
    run(&mut s, "chargen d20-heroic Zed");
    run(&mut s, "note salut");
    let md = run(&mut s, "log md - Titlu");
    assert!(md.contains("Zed") || md.contains("Fișa"));
    let dir = std::env::temp_dir().join(format!("z7_md_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("j.md");
    run(&mut s, &format!("log md {} Sesiunea 1", f.display()));
    let t = std::fs::read_to_string(&f).unwrap();
    assert!(t.starts_with("# Sesiunea 1") && t.contains("## Fișa personajului"));
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn help_lists_new_commands() {
    let mut s = Session::new(1);
    let h = run(&mut s, "help");
    for k in [
        "battle",
        "chargen",
        "gen spell",
        "encounter",
        "loot",
        "hexmap",
        "travel",
        "log md",
    ] {
        assert!(h.contains(k), "{k}");
    }
}
