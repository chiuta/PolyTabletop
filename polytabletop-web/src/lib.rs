//! Legătura WebAssembly: expune lista de jocuri și o partidă (`Match`) către JavaScript.
//! Toată logica (reguli, AI, informație ascunsă) vine neschimbată din `polytabletop-games`.
use polytabletop_core::i18n::Lang;
use polytabletop_core::{ai, GameObj, Outcome, Player};
use polytabletop_games as pg;
use wasm_bindgen::prelude::*;

fn lang_of(code: &str) -> Lang {
    Lang::from_code(code).unwrap_or(Lang::En)
}

fn json_str(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => {}
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push(' '),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// Lista jocurilor ca JSON: `[{"id":..,"title":..,"cat":..}, ...]`, cu titlul în limba cerută.
#[wasm_bindgen]
pub fn games_json(lang: &str) -> String {
    let l = lang_of(lang);
    let items: Vec<String> = pg::playable()
        .iter()
        .map(|p| {
            let t = pg::i18n::title(p.id, l).unwrap_or(p.title);
            format!(
                "{{\"id\":{},\"title\":{},\"cat\":{},\"hidden\":{}}}",
                json_str(p.id),
                json_str(t),
                json_str(p.category),
                pg::HOTSEAT_SAFE.contains(&p.id)
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

/// Regulile unui joc în limba cerută (română dacă nu există traducere).
#[wasm_bindgen]
pub fn rules_text(id: &str, lang: &str) -> String {
    pg::i18n::rules(id, lang_of(lang)).unwrap_or("").to_string()
}

fn level_of(n: u8) -> ai::Level {
    match n {
        0 => ai::Level::Easy,
        2 => ai::Level::Hard,
        3 => ai::Level::Expert,
        _ => ai::Level::Normal,
    }
}

/// Setează nivelul AI-ului (0 ușor, 1 normal, 2 greu, 3 expert) și bugetul de gândire în milisecunde.
#[wasm_bindgen]
pub fn set_ai(level: u8, budget_ms: u32) {
    ai::set_level(level_of(level));
    ai::set_time_budget_override_ms(if budget_ms > 0 {
        Some(budget_ms as u64)
    } else {
        None
    });
}

/// O partidă. `mode`: 0 = tu joci primul contra AI, 1 = tu joci al doilea, 2 = doi oameni (același ecran).
#[wasm_bindgen]
pub struct Match {
    g: Box<dyn GameObj>,
    mode: u8,
    seed: u64,
    step: u64,
}

#[wasm_bindgen]
impl Match {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str, mode: u8, seed: f64) -> Result<Match, JsValue> {
        let p = pg::find(id).ok_or_else(|| JsValue::from_str("joc necunoscut"))?;
        let mut g = (p.make)();
        let viewer = match mode {
            0 => Some(Player::One),
            1 => Some(Player::Two),
            _ => None,
        };
        g.set_viewer(viewer);
        Ok(Match {
            g,
            mode,
            seed: seed as u64,
            step: 0,
        })
    }

    /// Textul tablei, din punctul de vedere al jucătorului (informația ascunsă a adversarului nu apare).
    pub fn render(&self) -> String {
        self.g.render()
    }

    /// Mutările legale, ca JSON (listă de texte).
    pub fn moves(&self) -> String {
        let v: Vec<String> = self.g.legal_moves().iter().map(|m| json_str(m)).collect();
        format!("[{}]", v.join(","))
    }

    /// Joacă o mutare scrisă de om. Întoarce textul canonic sau mesajul de eroare precedat de „!”.
    pub fn play(&mut self, mv: &str) -> String {
        match self.g.play(mv) {
            Ok(t) => t,
            Err(e) => format!("!{e}"),
        }
    }

    /// Este rândul AI-ului?
    pub fn ai_turn(&self) -> bool {
        if self.g.outcome().is_some() || self.mode > 1 {
            return false;
        }
        let human = if self.mode == 0 {
            Player::One
        } else {
            Player::Two
        };
        self.g.to_move() != human
    }

    /// AI-ul alege și joacă o mutare; întoarce textul ei (sau gol dacă nu e rândul lui).
    pub fn ai_step(&mut self) -> String {
        if !self.ai_turn() {
            return String::new();
        }
        self.step += 1;
        let s = self.seed ^ self.step.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        match self.g.ai_move(s) {
            Some(m) => self.g.play(&m).unwrap_or_default(),
            None => String::new(),
        }
    }

    /// 1 sau 2 = jucătorul la mutare.
    pub fn to_move(&self) -> u8 {
        match self.g.to_move() {
            Player::One => 1,
            Player::Two => 2,
        }
    }

    /// Rezultatul: "" = în desfășurare, "1" sau "2" = a câștigat, "d" = remiză.
    pub fn outcome(&self) -> String {
        match self.g.outcome() {
            None => String::new(),
            Some(Outcome::Win(Player::One)) => "1".into(),
            Some(Outcome::Win(Player::Two)) => "2".into(),
            Some(Outcome::Draw) => "d".into(),
        }
    }

    /// Hot-seat: arată informația privată a jucătorului la mutare.
    pub fn show_mover(&mut self) {
        if self.mode > 1 {
            self.g.set_viewer(None);
        }
    }
}
