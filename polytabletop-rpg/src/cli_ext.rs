//! Extra CLI commands: battle, chargen, gen, encounter, loot, hexmap, travel,
//! weather (kept separate from `cli.rs` to keep both files manageable).

use crate::chargen::{self, GameSystem};
use crate::cli::{Output, Session};
use crate::combat::Battle;
use crate::encounter::{self, Difficulty, Foe};
use crate::gen::{GenLib, Rarity};
use crate::overland::{self, HexMap, Season, Weather};

pub struct Ext {
    pub genlib: GenLib,
    pub foes: Vec<Foe>,
    pub map: Option<HexMap>,
    pub season: Season,
}

impl Default for Ext {
    fn default() -> Ext {
        Ext {
            genlib: GenLib::builtin(),
            foes: encounter::builtin_foes(),
            map: None,
            season: Season::Summer,
        }
    }
}

fn out(s: impl Into<String>) -> Result<Output, String> {
    Ok(Output {
        text: s.into(),
        quit: false,
    })
}

fn num<T: std::str::FromStr>(s: &str, what: &str) -> Result<T, String> {
    s.parse::<T>().map_err(|_| format!("{what} invalid: {s}"))
}

fn read(p: &str) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"))
}

impl Session {
    pub fn ext_cmd(&mut self, cmd: &str, a: &[&str]) -> Result<Output, String> {
        match cmd {
            "battle" => self.battle_cmd(a),
            "chargen" => self.chargen_cmd(a),
            "gen" => self.gen_cmd(a),
            "encounter" => self.encounter_cmd(a),
            "loot" => self.loot_cmd(a),
            "hexmap" => self.hexmap_cmd(a),
            "travel" => self.travel_cmd(a),
            "weather" => self.weather_cmd(a),
            "season" => {
                let s = a
                    .first()
                    .and_then(|s| Season::parse(s))
                    .ok_or("folosire: season spring|summer|autumn|winter")?;
                self.ext.season = s;
                out("ok\n")
            }
            _ => Err(format!("comandă necunoscută: {cmd}")),
        }
    }

    fn battle_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let mut src = None;
        let mut rounds = None;
        for t in a {
            if let Some(n) = t.strip_prefix("rounds=") {
                rounds = Some(num::<u32>(n, "rounds")?);
            } else {
                src = Some(*t);
            }
        }
        let text = match src {
            None | Some("demo") => include_str!("../data/battle-demo.toml").to_string(),
            Some(p) => read(p)?,
        };
        let mut b = Battle::parse(&text)?;
        if let Some(r) = rounds {
            b.max_rounds = r.clamp(1, 1000);
        }
        let o = b.run(&mut self.rng);
        let rep = b.report(&o);
        let w = o
            .winner
            .map_or("egalitate".to_string(), |w| b.side_names[w].clone());
        self.journal.add(
            "battle",
            &format!(
                "{} vs {}: {w} ({} runde)",
                b.side_names[0], b.side_names[1], o.rounds
            ),
        );
        out(rep)
    }

    fn chargen_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        if a.first() == Some(&"list") {
            let mut o = String::new();
            for (id, _) in chargen::BUILTIN {
                let s = chargen::builtin(id).ok_or("sistem intern invalid")?;
                o += &format!("{:<16} {} [{}] -> use {}\n", s.id, s.name, s.kind, s.preset);
            }
            return out(o);
        }
        let which = a.first().copied().unwrap_or("d20-heroic");
        let sys: GameSystem = match chargen::builtin(which) {
            Some(s) => s,
            None => GameSystem::parse(&read(which)?)?,
        };
        let name = if a.len() > 1 {
            a[1..].join(" ")
        } else {
            chargen::random_name(&mut self.rng)
        };
        let sh = sys.generate(&name, &mut self.rng)?;
        let r = sh.render();
        self.sheet = Some(sh);
        let _ = self.exec(&format!("use {}", sys.preset));
        self.journal
            .add("chargen", &format!("{name} ({}, {})", sys.id, sys.kind));
        out(format!("{r}(preset de teste: {})\n", sys.preset))
    }

    fn gen_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        if let ["load", p] = a {
            self.ext.genlib = GenLib::parse(&read(p)?)?;
            return out("încărcat\n");
        }
        let kind = a
            .first()
            .ok_or("folosire: gen spell|ability|item [N] [min=rar]")?;
        let mut n = 1usize;
        let mut min = Rarity::Common;
        for t in &a[1..] {
            if let Some(r) = t.strip_prefix("min=") {
                min = Rarity::parse(r).ok_or_else(|| format!("raritate necunoscută: {r}"))?;
            } else {
                n = num::<usize>(t, "N")?.clamp(1, 100);
            }
        }
        let mut o = String::new();
        for _ in 0..n {
            let g = match *kind {
                "spell" => self.ext.genlib.spell(min, &mut self.rng),
                "ability" => self.ext.genlib.ability(min, &mut self.rng),
                "item" => self.ext.genlib.item(min, &mut self.rng),
                k => return Err(format!("gen: tip necunoscut {k}")),
            }
            .ok_or("tabel gol pentru această raritate")?;
            o += &format!("{}\n", g.text);
            self.journal.add("gen", &g.text);
        }
        out(o)
    }

    fn encounter_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        match a {
            ["foes", p] => {
                self.ext.foes = encounter::parse_foes(&read(p)?)?;
                out(format!("{} creaturi\n", self.ext.foes.len()))
            }
            ["budget", p, l] => {
                let (p, l) = (num::<u32>(p, "P")?, num::<u32>(l, "L")?);
                let mut o = String::new();
                for d in [
                    Difficulty::Easy,
                    Difficulty::Medium,
                    Difficulty::Hard,
                    Difficulty::Deadly,
                ] {
                    o += &format!("{:<7} {}\n", d.name(), encounter::budget(p, l, d));
                }
                out(o)
            }
            ["rate", p, l, crs @ ..] if !crs.is_empty() => {
                let (p, l) = (num::<u32>(p, "P")?, num::<u32>(l, "L")?);
                let v: Vec<f64> = crs
                    .iter()
                    .map(|c| num::<f64>(c, "CR"))
                    .collect::<Result<_, _>>()?;
                let r = encounter::rate(&v, p, l).map_or("trivial", |d| d.name());
                let msg = format!("cost ajustat {} -> {r}", encounter::group_cost(&v));
                self.journal.add("encounter", &msg);
                out(format!("{msg}\n"))
            }
            ["build", p, l, d, rest @ ..] => {
                let (p, l) = (num::<u32>(p, "P")?, num::<u32>(l, "L")?);
                let d = Difficulty::parse(d).ok_or("dificultate: easy|medium|hard|deadly")?;
                let mut max = 8usize;
                for t in rest {
                    if let Some(m) = t.strip_prefix("max=") {
                        max = num::<usize>(m, "max")?.clamp(1, 30);
                    }
                }
                let bud = encounter::budget(p, l, d);
                let pick = encounter::build(&self.ext.foes, bud, max, &mut self.rng);
                if pick.is_empty() {
                    return out(format!("buget {bud}: nicio creatură nu încape\n"));
                }
                let crs: Vec<f64> = pick.iter().map(|&i| self.ext.foes[i].cr).collect();
                let cost = encounter::group_cost(&crs);
                let mut names: Vec<(String, usize)> = vec![];
                for &i in &pick {
                    let n = &self.ext.foes[i].name;
                    match names.iter_mut().find(|(k, _)| k == n) {
                        Some((_, c)) => *c += 1,
                        None => names.push((n.clone(), 1)),
                    }
                }
                let list: Vec<String> = names.iter().map(|(n, c)| format!("{c}x {n}")).collect();
                let msg = format!(
                    "{}: {} | cost {cost}/{bud} | comoară ~{}",
                    d.name(),
                    list.join(", "),
                    encounter::treasure_value(cost)
                );
                self.journal.add("encounter", &msg);
                out(format!("{msg}\n"))
            }
            _ => {
                Err("encounter: budget P L | rate P L CR.. | build P L DIF [max=N] | foes F".into())
            }
        }
    }

    fn loot_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let v = num::<i64>(
            a.first().ok_or("folosire: loot VALOARE [NUME..|N]")?,
            "valoare",
        )?;
        if v < 0 {
            return Err("valoare negativă".into());
        }
        let members: Vec<String> = match &a[1..] {
            [] => (1..=4).map(|i| format!("PC{i}")).collect(),
            [n] if n.parse::<usize>().is_ok() => (1..=num::<usize>(n, "N")?.clamp(1, 20))
                .map(|i| format!("PC{i}"))
                .collect(),
            names => names.iter().map(|s| s.to_string()).collect(),
        };
        let h = encounter::make_hoard(v, &self.ext.genlib, &mut self.rng);
        let sh = encounter::distribute(&h, &members, &mut self.rng);
        let mut o = format!("{} monede, {} obiecte\n", h.coins, h.items.len());
        for (n, c, items) in &sh {
            o += &format!(
                "{n}: {c} monede; {}\n",
                if items.is_empty() {
                    "-".into()
                } else {
                    items.join(", ")
                }
            );
        }
        self.journal.add(
            "loot",
            &format!("comoară {v}: {} monede, {} obiecte", h.coins, h.items.len()),
        );
        out(o)
    }

    fn hexmap_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let n = |i: usize, d: usize| a.get(i).and_then(|s| s.parse().ok()).unwrap_or(d);
        let m = HexMap::generate(n(0, 24), n(1, 14), &mut self.rng);
        let o = m.render(&[]);
        self.journal.add("hexmap", &format!("{}x{}", m.w, m.h));
        self.ext.map = Some(m);
        out(o)
    }

    fn travel_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let m = self.ext.map.as_ref().ok_or("nicio hartă (hexmap)")?;
        let mut pace = 4u32;
        let mut c = vec![];
        for t in a {
            if let Some(p) = t.strip_prefix("pace=") {
                pace = num::<u32>(p, "pace")?;
            } else {
                c.push(num::<usize>(t, "coordonată")?);
            }
        }
        let [x1, y1, x2, y2] = c[..] else {
            return Err("folosire: travel X1 Y1 X2 Y2 [pace=N]".into());
        };
        let (path, cost) = m
            .path((x1, y1), (x2, y2))
            .ok_or("nu există drum (apă sau în afara hărții)")?;
        let map_txt = m.render(&path);
        let j = overland::travel(cost, pace, self.ext.season, &mut self.rng);
        let wx: Vec<&str> = j.weather.iter().map(|w| w.name()).collect();
        let msg = format!(
            "({x1},{y1}) -> ({x2},{y2}): {} hexuri, cost {cost}, {} zile ({} ore), vreme: {}",
            path.len() - 1,
            j.days,
            j.total_hours,
            wx.join(", ")
        );
        self.journal.add("travel", &msg);
        out(format!("{map_txt}{msg}\n"))
    }

    fn weather_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let season = match a.first() {
            Some(s) => Season::parse(s).ok_or("anotimp necunoscut")?,
            None => self.ext.season,
        };
        let w = Weather::roll(season, &mut self.rng);
        self.journal.add("weather", w.name());
        out(format!("{} (cost deplasare {}%)\n", w.name(), w.cost_pct()))
    }
}
