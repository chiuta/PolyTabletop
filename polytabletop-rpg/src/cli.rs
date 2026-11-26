//! Command interpreter behind the `polytabletop-rpg` binary (kept in the library so it
//! can be unit-tested).

use crate::check::{preset, presets, CheckRequest, CheckSystem};
use crate::data::Compendium;
use crate::dice::Expr;
use crate::dungeon::Dungeon;
use crate::initiative::Tracker;
use crate::journal::Journal;
use crate::sheet::{ModStyle, Sheet};
use crate::tables::TableSet;
use polytabletop_core::rng::Rng;

pub struct Output {
    pub text: String,
    pub quit: bool,
}

pub struct Session {
    pub rng: Rng,
    pub journal: Journal,
    pub sheet: Option<Sheet>,
    pub tracker: Tracker,
    pub tables: TableSet,
    pub compendium: Compendium,
    pub preset_id: String,
    pub dungeon: Option<Dungeon>,
    pub ext: crate::cli_ext::Ext,
    system: Box<dyn CheckSystem>,
    mod_style: ModStyle,
    default_dc: i32,
}

fn out(s: impl Into<String>) -> Result<Output, String> {
    Ok(Output {
        text: s.into(),
        quit: false,
    })
}

fn int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("număr invalid: {s}"))
}

pub const HELP: &str = "\
Comenzi:
  roll EXPR                  ex. roll 4d6kh3 | 2d10!>=9 | 8d6s>=5 | d%
  presets | use ID           listează / alege sistemul de teste
  check RATING [DC] [edge=N] [mod=N] [attrs=a,b,c]
                             RATING = număr sau nume de abilitate/atribut din fișă
  sheet new NUME | show | text | save F | load F
  sheet attr N V | skill N V | res N CUR MAX | adj N DELTA
  sheet item add N [Q] | item rm N [Q] | cond add N [RUNDE] | cond rm N | tick
  init add NUME VAL [tiebreak] | roll NUME [mod] [EXPR] | start | next | remove NUME | set NUME VAL | show | reset
  table load F | list | roll NUME
  dungeon [lat inalt camere]
  bestiary load F | show ID | spawn ID
  note TEXT | log [N] | log search TEXT | log save F | log load F | log md [F|-] [titlu]
  battle [demo|F] [rounds=N]  simulator de bătălie (unități, moral, teren); F = fișier [battle]/[unit.ID]
  chargen [list|ID|F] [NUME]  generator de personaje (sisteme: percentile d20-heroic grit-pool star-narrative)
  gen spell|ability|item [N] [min=rar]  | gen load F   generatoare cu ponderi de raritate
  encounter budget P L | rate P L CR.. | build P L DIF [max=N] | foes F   echilibrare (CR)
  loot VALOARE [NUME..|N]     comoară (monede + obiecte) împărțită între personaje
  hexmap [L H] | travel X1 Y1 X2 Y2 [pace=N] | weather [anotimp] | season ANOTIMP   hartă hex + vreme
  seed N | help | q";

impl Session {
    pub fn new(seed: u64) -> Session {
        let p = preset("d20-dc").expect("default preset");
        Session {
            rng: Rng::new(seed),
            journal: Journal::new(),
            sheet: None,
            tracker: Tracker::new(),
            tables: TableSet::default(),
            compendium: Compendium::default(),
            preset_id: p.id.to_string(),
            dungeon: None,
            ext: crate::cli_ext::Ext::default(),
            system: (p.build)(),
            mod_style: p.mod_style,
            default_dc: p.default_difficulty,
        }
    }

    pub fn new_from_clock() -> Session {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1);
        Session::new(n)
    }

    fn sheet_mut(&mut self) -> Result<&mut Sheet, String> {
        self.sheet
            .as_mut()
            .ok_or_else(|| "nicio fișă (sheet new NUME)".to_string())
    }

    pub fn exec(&mut self, line: &str) -> Result<Output, String> {
        let toks: Vec<&str> = line.split_whitespace().collect();
        let Some(&cmd) = toks.first() else {
            return out("");
        };
        let args = &toks[1..];
        match cmd.to_lowercase().as_str() {
            "help" | "?" => out(format!("{HELP}\n")),
            "q" | "quit" | "exit" => Ok(Output {
                text: String::new(),
                quit: true,
            }),
            "seed" => {
                let n: u64 = args
                    .first()
                    .and_then(|s| s.parse().ok())
                    .ok_or("folosire: seed N")?;
                self.rng = Rng::new(n);
                out(format!("seed = {n}\n"))
            }
            "roll" => {
                if args.is_empty() {
                    return Err("folosire: roll EXPR".into());
                }
                let src = args.join("");
                let e = Expr::parse(&src).map_err(|e| e.to_string())?;
                let r = e.roll(&mut self.rng);
                let msg = format!("{e}: {r}");
                self.journal.add("roll", &msg);
                out(format!("{msg}\n"))
            }
            "presets" => {
                let mut o = String::new();
                for p in presets() {
                    let mark = if p.id == self.preset_id { "*" } else { " " };
                    o += &format!("{mark} {:<16} {}\n", p.id, p.summary);
                }
                out(o)
            }
            "use" => {
                let id = args.first().ok_or("folosire: use ID")?;
                let p = preset(id).ok_or_else(|| format!("presetare necunoscută: {id}"))?;
                self.preset_id = p.id.to_string();
                self.system = (p.build)();
                self.mod_style = p.mod_style;
                self.default_dc = p.default_difficulty;
                out(format!("sistem: {} ({})\n", p.id, self.system.name()))
            }
            "check" => self.check(args),
            "sheet" => self.sheet_cmd(args),
            "init" => self.init_cmd(args),
            "table" => self.table_cmd(args),
            "dungeon" => {
                let n = |i: usize, d: usize| args.get(i).and_then(|s| s.parse().ok()).unwrap_or(d);
                let d = Dungeon::generate(n(0, 60), n(1, 22), n(2, 8), &mut self.rng);
                let mut o = d.render();
                o += &format!("{} camere\n", d.rooms.len());
                self.journal.add(
                    "dungeon",
                    &format!("{}x{}, {} camere", d.w, d.h, d.rooms.len()),
                );
                self.dungeon = Some(d);
                out(o)
            }
            "bestiary" => self.bestiary_cmd(args),
            "note" => {
                let t = line.trim_start()[cmd.len()..].trim();
                if t.is_empty() {
                    return Err("folosire: note TEXT".into());
                }
                let n = self.journal.add("note", t);
                out(format!("#{n}\n"))
            }
            "log" => self.log_cmd(args),
            "battle" | "chargen" | "gen" | "encounter" | "loot" | "hexmap" | "travel"
            | "weather" | "season" => self.ext_cmd(&cmd.to_lowercase(), args),
            other => Err(format!("comandă necunoscută: {other} (help)")),
        }
    }

    fn rating_of(&self, tok: &str) -> Result<i32, String> {
        if let Ok(n) = tok.parse::<i32>() {
            return Ok(n);
        }
        let sh = self.sheet.as_ref().ok_or("nume fără fișă încărcată")?;
        if let Some(v) = sh.skill(tok) {
            return Ok(v);
        }
        if let Some(v) = sh.modifier(tok, self.mod_style) {
            return Ok(v);
        }
        Err(format!("nu găsesc '{tok}' în fișă"))
    }

    fn check(&mut self, args: &[&str]) -> Result<Output, String> {
        let first = args
            .first()
            .ok_or("folosire: check RATING [DC] [edge=N] [mod=N] [attrs=a,b,c]")?;
        let mut req = CheckRequest::new(self.rating_of(first)?, self.default_dc);
        for a in &args[1..] {
            if let Some(v) = a.strip_prefix("edge=") {
                req.edge = int(v)?;
            } else if let Some(v) = a.strip_prefix("mod=") {
                req.modifier = int(v)?;
            } else if let Some(v) = a.strip_prefix("attrs=") {
                req.extra = v.split(',').map(int).collect::<Result<_, _>>()?;
            } else if a.eq_ignore_ascii_case("adv") {
                req.edge = 1;
            } else if a.eq_ignore_ascii_case("dis") {
                req.edge = -1;
            } else {
                req.difficulty = int(a)?;
            }
        }
        let r = self.system.resolve(&req, &mut self.rng)?;
        let mut msg = format!(
            "[{}] {} (marjă {}) — {}",
            self.preset_id, r.degree, r.margin, r.detail
        );
        if r.complications > 0 {
            msg += &format!("; complicații: {}", r.complications);
        }
        if r.advantages > 0 {
            msg += &format!("; avantaje: {}", r.advantages);
        }
        self.journal.add("check", &msg);
        out(format!("{msg}\n"))
    }

    fn sheet_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let sub = a.first().copied().unwrap_or("show");
        let rest = &a[a.len().min(1)..];
        match sub {
            "new" => {
                if rest.is_empty() {
                    return Err("folosire: sheet new NUME".into());
                }
                self.sheet = Some(Sheet::new(&rest.join(" ")));
                out("fișă nouă\n")
            }
            "show" => out(self.sheet.as_ref().ok_or("nicio fișă")?.render()),
            "text" => out(self.sheet.as_ref().ok_or("nicio fișă")?.to_text()),
            "attr" | "skill" => {
                let [n, v] = rest else {
                    return Err(format!("folosire: sheet {sub} NUME VALOARE"));
                };
                let v = int(v)?;
                let sh = self.sheet_mut()?;
                if sub == "attr" {
                    sh.set_attr(n, v)
                } else {
                    sh.set_skill(n, v)
                }
                out("ok\n")
            }
            "res" => {
                let [n, c, m] = rest else {
                    return Err("folosire: sheet res NUME CUR MAX".into());
                };
                let (c, m) = (int(c)?, int(m)?);
                self.sheet_mut()?.set_resource(n, c, m);
                out("ok\n")
            }
            "adj" => {
                let [n, d] = rest else {
                    return Err("folosire: sheet adj NUME DELTA".into());
                };
                let d = int(d)?;
                let v = self.sheet_mut()?.adjust(n, d)?;
                out(format!("{n} = {v}\n"))
            }
            "item" => match rest {
                ["add", n] => {
                    self.sheet_mut()?.add_item(n, 1, "");
                    out("ok\n")
                }
                ["add", n, q] => {
                    let q = q
                        .parse::<u32>()
                        .map_err(|_| "cantitate invalidă".to_string())?;
                    self.sheet_mut()?.add_item(n, q, "");
                    out("ok\n")
                }
                ["rm", n] => out(format!(
                    "rămase: {}\n",
                    self.sheet_mut()?.remove_item(n, 1)?
                )),
                ["rm", n, q] => {
                    let q = q
                        .parse::<u32>()
                        .map_err(|_| "cantitate invalidă".to_string())?;
                    out(format!(
                        "rămase: {}\n",
                        self.sheet_mut()?.remove_item(n, q)?
                    ))
                }
                _ => Err("folosire: sheet item add|rm NUME [Q]".into()),
            },
            "cond" => match rest {
                ["add", n] => {
                    self.sheet_mut()?.add_condition(n, None);
                    out("ok\n")
                }
                ["add", n, t] => {
                    let t = t
                        .parse::<u32>()
                        .map_err(|_| "durată invalidă".to_string())?;
                    self.sheet_mut()?.add_condition(n, Some(t));
                    out("ok\n")
                }
                ["rm", n] => {
                    let had = self.sheet_mut()?.remove_condition(n);
                    out(if had { "scoasă\n" } else { "nu exista\n" })
                }
                _ => Err("folosire: sheet cond add|rm NUME [RUNDE]".into()),
            },
            "tick" => {
                let gone = self.sheet_mut()?.tick_conditions();
                out(if gone.is_empty() {
                    "nicio condiție expirată\n".to_string()
                } else {
                    format!("expirate: {}\n", gone.join(", "))
                })
            }
            "save" => {
                let p = rest.first().ok_or("folosire: sheet save FIȘIER")?;
                let t = self.sheet.as_ref().ok_or("nicio fișă")?.to_text();
                std::fs::write(p, t).map_err(|e| e.to_string())?;
                out("salvat\n")
            }
            "load" => {
                let p = rest.first().ok_or("folosire: sheet load FIȘIER")?;
                let t = std::fs::read_to_string(p).map_err(|e| e.to_string())?;
                self.sheet = Some(Sheet::from_text(&t)?);
                out("încărcat\n")
            }
            _ => Err("sheet: new|show|text|attr|skill|res|adj|item|cond|tick|save|load".into()),
        }
    }

    fn init_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let sub = a.first().copied().unwrap_or("show");
        let rest = &a[a.len().min(1)..];
        match sub {
            "add" => {
                let (n, v) = (
                    rest.first()
                        .ok_or("folosire: init add NUME VAL [tiebreak]")?,
                    rest.get(1).ok_or("lipsește valoarea")?,
                );
                let tb = rest.get(2).map_or(Ok(0), |s| int(s))?;
                self.tracker.add(n, int(v)?, tb)?;
                out(self.tracker.render())
            }
            "roll" => {
                let n = rest
                    .first()
                    .ok_or("folosire: init roll NUME [mod] [EXPR]")?;
                let m = rest.get(1).map_or(Ok(0), |s| int(s))?;
                let e = rest.get(2).copied().unwrap_or("1d20");
                let v = self.tracker.roll_add(n, e, m, &mut self.rng)?;
                self.journal.add("init", &format!("{n}: {v}"));
                out(format!("{n}: {v}\n"))
            }
            "start" => {
                self.tracker.start()?;
                self.journal.add("combat", "început");
                out(self.tracker.render())
            }
            "next" => {
                let (who, nr) = self.tracker.advance()?;
                if nr {
                    self.journal
                        .add("combat", &format!("runda {}", self.tracker.round()));
                }
                let mut o = format!("{who} (runda {})\n", self.tracker.round());
                if nr {
                    if let Some(sh) = self.sheet.as_mut() {
                        let g = sh.tick_conditions();
                        if !g.is_empty() {
                            o += &format!("condiții expirate: {}\n", g.join(", "));
                        }
                    }
                }
                out(o)
            }
            "remove" => {
                self.tracker
                    .remove(rest.first().ok_or("folosire: init remove NUME")?)?;
                out(self.tracker.render())
            }
            "set" => {
                let [n, v] = rest else {
                    return Err("folosire: init set NUME VAL".into());
                };
                self.tracker.set_init(n, int(v)?)?;
                out(self.tracker.render())
            }
            "show" => out(self.tracker.render()),
            "reset" => {
                self.tracker.reset();
                out("resetat\n")
            }
            _ => Err("init: add|roll|start|next|remove|set|show|reset".into()),
        }
    }

    fn table_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        match a {
            ["load", p] => {
                let t = std::fs::read_to_string(p).map_err(|e| e.to_string())?;
                self.tables = TableSet::from_text(&t)?;
                out(format!("{} tabele\n", self.tables.tables.len()))
            }
            ["list"] => out(self
                .tables
                .tables
                .iter()
                .map(|t| format!("{} ({})\n", t.name, t.dice))
                .collect::<String>()),
            ["roll", rest @ ..] if !rest.is_empty() => {
                let r = self.tables.roll(&rest.join(" "), &mut self.rng)?;
                self.journal
                    .add("table", &format!("{}: {r}", rest.join(" ")));
                out(format!("{r}\n"))
            }
            _ => Err("table: load F | list | roll NUME".into()),
        }
    }

    fn bestiary_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        match a {
            ["load", p] => {
                let t = std::fs::read_to_string(p).map_err(|e| e.to_string())?;
                self.compendium = Compendium::parse(&t)?;
                out(format!(
                    "{} creaturi, {} obiecte\n",
                    self.compendium.creatures.len(),
                    self.compendium.items.len()
                ))
            }
            ["show", id] => {
                let c = self
                    .compendium
                    .creature(id)
                    .ok_or_else(|| format!("necunoscut: {id}"))?;
                out(format!(
                    "{} — PV {}, apărare {}, atac {:+}, daună {}{}\n{}\n",
                    c.name,
                    c.hp,
                    c.defense,
                    c.attack,
                    c.damage,
                    if c.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", c.tags.join(", "))
                    },
                    c.desc
                ))
            }
            ["spawn", id] => {
                let c = self
                    .compendium
                    .creature(id)
                    .ok_or_else(|| format!("necunoscut: {id}"))?
                    .clone();
                let mut name = c.name.clone();
                let mut k = 1;
                while self
                    .tracker
                    .entries()
                    .iter()
                    .any(|e| e.name.eq_ignore_ascii_case(&name))
                {
                    k += 1;
                    name = format!("{} {k}", c.name);
                }
                let v = self
                    .tracker
                    .roll_add(&name, "1d20", c.attack, &mut self.rng)?;
                self.journal
                    .add("combat", &format!("{name} apare (init {v})"));
                out(format!("{name}: init {v}, PV {}\n", c.hp))
            }
            _ => Err("bestiary: load F | show ID | spawn ID".into()),
        }
    }

    fn log_cmd(&mut self, a: &[&str]) -> Result<Output, String> {
        let fmt = |es: &[crate::journal::LogEntry]| -> String {
            es.iter()
                .map(|e| format!("{:>3} [{}] {}\n", e.seq, e.kind, e.text))
                .collect()
        };
        match a {
            [] => out(fmt(self.journal.last(10))),
            ["search", rest @ ..] if !rest.is_empty() => {
                let r: Vec<_> = self
                    .journal
                    .search(&rest.join(" "))
                    .into_iter()
                    .cloned()
                    .collect();
                out(fmt(&r))
            }
            ["save", p] => {
                std::fs::write(p, self.journal.to_text()).map_err(|e| e.to_string())?;
                out("salvat\n")
            }
            ["load", p] => {
                let t = std::fs::read_to_string(p).map_err(|e| e.to_string())?;
                self.journal = Journal::from_text(&t)?;
                out(format!("{} intrări\n", self.journal.len()))
            }
            [n] => {
                let n = n
                    .parse::<usize>()
                    .map_err(|_| "folosire: log [N]".to_string())?;
                out(fmt(self.journal.last(n)))
            }
            ["md", rest @ ..] => {
                let title = if rest.len() > 1 {
                    rest[1..].join(" ")
                } else {
                    "Jurnal de campanie".to_string()
                };
                let md = self.journal.to_markdown(&title, self.sheet.as_ref());
                match rest.first() {
                    Some(p) if *p != "-" => {
                        std::fs::write(p, &md).map_err(|e| e.to_string())?;
                        out("exportat\n")
                    }
                    _ => out(md),
                }
            }
            _ => Err("log: [N] | search TEXT | save F | load F | md [F] [titlu]".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &mut Session, l: &str) -> String {
        s.exec(l).unwrap_or_else(|e| panic!("{l}: {e}")).text
    }

    #[test]
    fn roll_and_log() {
        let mut s = Session::new(1);
        let o = run(&mut s, "roll 2d6 + 3");
        assert!(o.contains("2d6+3") && o.contains('='));
        assert_eq!(s.journal.len(), 1);
        assert!(s.exec("roll").is_err());
        assert!(s.exec("roll 2d").is_err());
        assert!(s.exec("frobnicate").is_err());
        assert!(s.exec("q").unwrap().quit);
        assert_eq!(run(&mut s, "   "), "");
    }

    #[test]
    fn seed_makes_runs_repeatable() {
        let mut a = Session::new(1);
        let mut b = Session::new(2);
        run(&mut a, "seed 9");
        run(&mut b, "seed 9");
        assert_eq!(run(&mut a, "roll 10d20"), run(&mut b, "roll 10d20"));
    }

    #[test]
    fn presets_and_checks() {
        let mut s = Session::new(3);
        assert!(run(&mut s, "presets").contains("d100-under"));
        for p in presets() {
            run(&mut s, &format!("use {}", p.id));
            let o = run(&mut s, "check 5 attrs=10,10,10");
            assert!(o.contains(p.id), "{o}");
        }
        assert!(s.exec("use nope").is_err());
        run(&mut s, "use d20-dc");
        assert!(run(&mut s, "check 3 12 adv mod=1").contains("d20-dc"));
        assert!(s.exec("check x").is_err());
        assert!(s.exec("check").is_err());
        run(&mut s, "use triple-d20");
        assert!(s.exec("check 5").is_err()); // needs attrs
    }

    #[test]
    fn sheet_flow_and_named_rating() {
        let mut s = Session::new(4);
        assert!(s.exec("sheet attr A 1").is_err());
        run(&mut s, "sheet new Aria Voss");
        run(&mut s, "sheet attr Vigor 14");
        run(&mut s, "sheet skill Stealth 4");
        run(&mut s, "sheet res HP 10 12");
        assert_eq!(run(&mut s, "sheet adj HP -3"), "HP = 7\n");
        run(&mut s, "sheet item add Rope 2");
        assert_eq!(run(&mut s, "sheet item rm Rope"), "rămase: 1\n");
        run(&mut s, "sheet cond add Poisoned 1");
        assert!(run(&mut s, "sheet tick").contains("Poisoned"));
        assert!(run(&mut s, "sheet show").contains("Aria Voss"));
        assert!(run(&mut s, "check Stealth 10").contains("[d20-dc]"));
        assert!(run(&mut s, "check Vigor 10").contains("[d20-dc]"));
        assert!(s.exec("check Nothing").is_err());
        assert!(s.exec("sheet res HP 1").is_err());
        let dir =
            std::env::temp_dir().join(format!("polytabletop_rpg_sheet_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("a.sheet");
        run(&mut s, &format!("sheet save {}", f.display()));
        let before = s.sheet.clone();
        s.sheet = None;
        run(&mut s, &format!("sheet load {}", f.display()));
        assert_eq!(s.sheet, before);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn combat_flow() {
        let mut s = Session::new(5);
        run(&mut s, "init add Orc 12");
        run(&mut s, "init add Elf 18 1");
        run(&mut s, "init roll Troll 2");
        assert!(s.exec("init add Orc 3").is_err());
        run(&mut s, "init start");
        let first = run(&mut s, "init next");
        assert!(first.contains("runda"));
        run(&mut s, "init next");
        run(&mut s, "init next");
        assert!(s.tracker.round() >= 2);
        run(&mut s, "init remove Orc");
        run(&mut s, "init set Elf 1");
        assert_eq!(s.tracker.len(), 2);
        run(&mut s, "init reset");
        assert!(s.tracker.is_empty());
        assert!(s.exec("init next").is_err());
    }

    #[test]
    fn data_driven_commands() {
        let dir =
            std::env::temp_dir().join(format!("polytabletop_rpg_data_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let t = dir.join("t.txt");
        std::fs::write(&t, "table: Loot\n1-3: coin\n4-6: gem\n").unwrap();
        let b = dir.join("b.toml");
        std::fs::write(&b, "[creature.rat]\nhp = 2\nattack = 1\n").unwrap();
        let mut s = Session::new(6);
        assert!(run(&mut s, &format!("table load {}", t.display())).contains('1'));
        assert!(run(&mut s, "table list").contains("Loot"));
        let r = run(&mut s, "table roll loot");
        assert!(r == "coin\n" || r == "gem\n");
        assert!(s.exec("table roll nope").is_err());
        assert!(run(&mut s, &format!("bestiary load {}", b.display())).contains("1 creaturi"));
        assert!(run(&mut s, "bestiary show rat").contains("PV 2"));
        run(&mut s, "bestiary spawn rat");
        assert!(run(&mut s, "bestiary spawn rat").contains("rat 2"));
        assert_eq!(s.tracker.len(), 2);
        assert!(s.exec("bestiary show ghost").is_err());
        assert!(s.exec("table load /nonexistent/x").is_err());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn dungeon_and_journal() {
        let mut s = Session::new(7);
        let o = run(&mut s, "dungeon 40 16 5");
        assert!(o.contains("camere"));
        assert!(s.dungeon.is_some());
        run(&mut s, "note a găsit o cheie");
        assert!(run(&mut s, "log search cheie").contains("cheie"));
        assert!(run(&mut s, "log 2").lines().count() <= 2);
        assert!(s.exec("note").is_err());
        let dir = std::env::temp_dir().join(format!("polytabletop_rpg_log_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("j.log");
        run(&mut s, &format!("log save {}", f.display()));
        let n = s.journal.len();
        let mut s2 = Session::new(1);
        run(&mut s2, &format!("log load {}", f.display()));
        assert_eq!(s2.journal.len(), n);
        let _ = std::fs::remove_dir_all(dir);
    }
}
