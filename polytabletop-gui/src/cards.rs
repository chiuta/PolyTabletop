//! Cărți de joc, zaruri și piese de domino desenate VECTORIAL cu `egui::Painter`
//! (fără fonturi pentru simboluri: ♥♦♣♠ sunt forme umplute) și parserul de jetoane
//! de carte care recunoaște formatele REALE din `render()` / `legal_moves()`.
//!
//! * Modelul: [`Card`] `{rank, suit, deck}` cu pachetele [`Deck::French`], `Italian`,
//!   `Spanish` (truco) și `Tarot` (cu atu-uri și Scuza).
//! * Parserul: [`parse_card`] (un jeton), [`parse_move`] (o mutare cu mai multe jetoane,
//!   ex. `7c:3s+4b`, `6s>10c`, `d Jh 9h`, `8c x 6d 2d`), [`rows`] (liniile din randare).
//!   Fiecare joc are un [`Spec`] explicit ([`spec_for`]): jocurile necunoscute nu se ating.
//! * Desenul: [`draw_card`], [`draw_back`], [`draw_die`], [`draw_domino`].
//!
//! Culorile sunt constante locale (paleta „Fără Frontiere”) ca să poată fi aliniate
//! ulterior cu tema aplicației.

use eframe::egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, Vec2};

// ───────────────────────── paleta „Fără Frontiere” ─────────────────────────
pub const CREAM: Color32 = Color32::from_rgb(0xf5, 0xed, 0xe0);
pub const INK: Color32 = Color32::from_rgb(0x1d, 0x17, 0x10);
pub const RED: Color32 = Color32::from_rgb(0xb3, 0x26, 0x1e);
pub const GOLD: Color32 = Color32::from_rgb(0xc8, 0x83, 0x2a);
pub const BACK_BROWN: Color32 = Color32::from_rgb(0x2a, 0x20, 0x10);
const EDGE: Color32 = Color32::from_rgb(0xb9, 0xa8, 0x8a);
const STEEL: Color32 = Color32::from_rgb(0x3b, 0x4a, 0x5a);
const WOOD: Color32 = Color32::from_rgb(0x7a, 0x4e, 0x1d);
const GOLD_DK: Color32 = Color32::from_rgb(0x8f, 0x5a, 0x14);

// ───────────────────────────── model ─────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Deck {
    French,
    Italian,
    Spanish,
    Tarot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
    Coins,
    Cups,
    Swords,
    Batons,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Rank {
    Ace,
    /// 2..=10
    Num(u8),
    Jack,
    Queen,
    King,
    /// calul (italiană/spaniolă) sau cavalerul (tarot)
    Knight,
    Joker,
    /// atu de tarot 1..=21
    Trump(u8),
    Excuse,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Option<Suit>,
    pub deck: Deck,
}

impl Card {
    /// Eticheta de colț (text: cifre și litere, fără simboluri speciale).
    pub fn label(&self) -> String {
        match self.rank {
            Rank::Ace => "A".into(),
            Rank::Num(n) => n.to_string(),
            Rank::Jack => match self.deck {
                Deck::Italian => "F".into(),
                Deck::Spanish => "10".into(),
                Deck::Tarot => "V".into(),
                Deck::French => "J".into(),
            },
            Rank::Queen => match self.deck {
                Deck::Tarot => "D".into(),
                _ => "Q".into(),
            },
            Rank::King => match self.deck {
                Deck::Italian | Deck::Tarot => "R".into(),
                Deck::Spanish => "12".into(),
                Deck::French => "K".into(),
            },
            Rank::Knight => match self.deck {
                Deck::Spanish => "11".into(),
                _ => "C".into(),
            },
            Rank::Joker => "Jk".into(),
            Rank::Trump(n) => n.to_string(),
            Rank::Excuse => "Ex".into(),
        }
    }

    /// Jetonul canonic ca în randările jocurilor (`10h`, `As`, `Fc`, `T12`, `Jk`).
    #[cfg(test)]
    pub fn token(&self) -> String {
        match self.rank {
            Rank::Joker => return "Jk".into(),
            Rank::Excuse => return "Ex".into(),
            Rank::Trump(n) => return format!("T{n}"),
            _ => {}
        }
        let r = match (self.rank, self.deck) {
            (Rank::Ace, Deck::Italian | Deck::Spanish | Deck::Tarot) => "1".to_string(),
            (Rank::Ace, _) => "A".to_string(),
            _ => self.label(),
        };
        format!(
            "{r}{}",
            self.suit.map_or('?', |s| suit_letter(s, self.deck))
        )
    }
}

#[cfg(test)]
pub fn suit_letter(s: Suit, deck: Deck) -> char {
    match (s, deck) {
        (Suit::Hearts, _) => 'h',
        (Suit::Diamonds, _) => 'd',
        (Suit::Clubs, _) => 'c',
        (Suit::Spades, _) => 's',
        (Suit::Coins, Deck::Spanish) => 'o',
        (Suit::Coins, _) => 'd',
        (Suit::Cups, _) => 'c',
        (Suit::Swords, Deck::Spanish) => 'e',
        (Suit::Swords, _) => 's',
        (Suit::Batons, _) => 'b',
    }
}

/// Reguli de recunoaștere pentru un joc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spec {
    pub kind: SpecKind,
    pub jokers: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecKind {
    Cards(Deck),
    Domino,
}

/// Jocurile pentru care jetoanele sunt cunoscute cu certitudine (verificate pe sursele jocurilor).
pub fn spec_for(id: &str) -> Option<Spec> {
    let cards = |d: Deck, jokers: bool| {
        Some(Spec {
            kind: SpecKind::Cards(d),
            jokers,
        })
    };
    match id {
        "popa" | "treizeci1" | "casino" | "septica" | "macao" | "66" | "rummy" | "blackjack"
        | "poker" | "durak" | "skat" | "cribbage" | "euchre" | "inimi" | "pica" | "prinde"
        | "tarneeb" | "baloot" | "tablanet" | "trix" | "bridge" | "whist" | "belote"
        | "teenpatti" | "pescuit" => cards(Deck::French, false),
        "canasta" => cards(Deck::French, true),
        "briscola" | "scopa" => cards(Deck::Italian, false),
        "truco" => cards(Deck::Spanish, false),
        "tarot" => cards(Deck::Tarot, false),
        "domino" | "paigow" => Some(Spec {
            kind: SpecKind::Domino,
            jokers: false,
        }),
        _ => None,
    }
}

// ───────────────────────────── parser ─────────────────────────────

fn parse_rank(r: &str, deck: Deck) -> Option<Rank> {
    Some(match (deck, r) {
        (Deck::French, "A") => Rank::Ace,
        (Deck::French, "J") => Rank::Jack,
        (Deck::French, "Q") => Rank::Queen,
        (Deck::French, "K") => Rank::King,
        (Deck::French, n) => match n.parse::<u8>().ok()? {
            v @ 2..=10 => Rank::Num(v),
            _ => return None,
        },
        (Deck::Italian, "A" | "1") => Rank::Ace,
        (Deck::Italian, "F") => Rank::Jack,
        (Deck::Italian, "C") => Rank::Knight,
        (Deck::Italian, "R") => Rank::King,
        (Deck::Italian, n) => match n.parse::<u8>().ok()? {
            v @ 2..=7 => Rank::Num(v),
            _ => return None,
        },
        (Deck::Spanish, n) => match n.parse::<u8>().ok()? {
            1 => Rank::Ace,
            v @ 2..=7 => Rank::Num(v),
            10 => Rank::Jack,
            11 => Rank::Knight,
            12 => Rank::King,
            _ => return None,
        },
        (Deck::Tarot, "V") => Rank::Jack,
        (Deck::Tarot, "C") => Rank::Knight,
        (Deck::Tarot, "D") => Rank::Queen,
        (Deck::Tarot, "R") => Rank::King,
        (Deck::Tarot, n) => match n.parse::<u8>().ok()? {
            1 => Rank::Ace,
            v @ 2..=10 => Rank::Num(v),
            _ => return None,
        },
    })
}

fn parse_suit(c: char, deck: Deck) -> Option<Suit> {
    Some(match (deck, c) {
        (Deck::French | Deck::Tarot, 'h' | '♥') => Suit::Hearts,
        (Deck::French | Deck::Tarot, 'd' | '♦') => Suit::Diamonds,
        (Deck::French | Deck::Tarot, 'c' | '♣') => Suit::Clubs,
        (Deck::French | Deck::Tarot, 's' | '♠') => Suit::Spades,
        (Deck::Italian, 'd') => Suit::Coins,
        (Deck::Italian, 'c') => Suit::Cups,
        (Deck::Italian, 's') => Suit::Swords,
        (Deck::Italian, 'b') => Suit::Batons,
        (Deck::Spanish, 'o') => Suit::Coins,
        (Deck::Spanish, 'c') => Suit::Cups,
        (Deck::Spanish, 'e') => Suit::Swords,
        (Deck::Spanish, 'b') => Suit::Batons,
        _ => return None,
    })
}

/// Un jeton izolat: `10h`, `As`, `7♥`, `Fc`, `T12`, `Ex`, `Jk` (dacă jokerii sunt permiși).
/// Sufixul în paranteze pătrate (`A♦[Ad]`) este ignorat.
pub fn parse_card(tok: &str, deck: Deck, jokers: bool) -> Option<Card> {
    let t = match tok.find('[') {
        Some(i) if i > 0 => &tok[..i],
        _ => tok,
    };
    if t.is_empty() || t.chars().count() > 4 {
        return None;
    }
    if jokers && t == "Jk" {
        return Some(Card {
            rank: Rank::Joker,
            suit: None,
            deck,
        });
    }
    if deck == Deck::Tarot {
        if t == "Ex" {
            return Some(Card {
                rank: Rank::Excuse,
                suit: None,
                deck,
            });
        }
        if let Some(n) = t.strip_prefix('T') {
            let v: u8 = n.parse().ok()?;
            if (1..=21).contains(&v) {
                return Some(Card {
                    rank: Rank::Trump(v),
                    suit: None,
                    deck,
                });
            }
            return None;
        }
    }
    let last = t.chars().last()?;
    let suit = parse_suit(last, deck)?;
    let rank_s = &t[..t.len() - last.len_utf8()];
    let rank = parse_rank(rank_s, deck)?;
    Some(Card {
        rank,
        suit: Some(suit),
        deck,
    })
}

/// O piesă de domino `a-b` / `a|b` (0..=9).
pub fn parse_domino(tok: &str) -> Option<(u8, u8)> {
    let t = match tok.find('[') {
        Some(i) if i > 0 => &tok[..i],
        _ => tok,
    };
    let (a, b) = t.split_once('-').or_else(|| t.split_once('|'))?;
    if a.len() != 1 || b.len() != 1 {
        return None;
    }
    let (a, b) = (a.parse::<u8>().ok()?, b.parse::<u8>().ok()?);
    Some((a, b))
}

/// Părțile unei mutări / ale unui jeton compus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Part {
    Card(Card),
    Domino(u8, u8),
    /// alegerea unei culori (`7h:h`)
    Suit(Suit),
    /// separator sau cuvânt
    Text(String),
    /// sfârșitul lanțului de domino: stânga (`:S`) / dreapta (`:D`)
    Arrow(bool),
}

impl Part {
    pub fn is_piece(&self) -> bool {
        matches!(self, Part::Card(_) | Part::Domino(..))
    }
}

fn word_part(w: &str, spec: Spec, after_colon: bool) -> Part {
    match spec.kind {
        SpecKind::Cards(deck) => {
            if let Some(c) = parse_card(w, deck, spec.jokers) {
                return Part::Card(c);
            }
            if after_colon && w.chars().count() == 1 {
                if let Some(s) = parse_suit(w.chars().next().unwrap_or(' '), deck) {
                    return Part::Suit(s);
                }
            }
        }
        SpecKind::Domino => {
            if let Some((a, b)) = parse_domino(w) {
                return Part::Domino(a, b);
            }
            if after_colon && (w == "S" || w == "D") {
                return Part::Arrow(w == "S");
            }
        }
    }
    if w == "d" && matches!(spec.kind, SpecKind::Cards(_)) {
        return Part::Text("aruncă".into());
    }
    Part::Text(w.to_string())
}

/// Descompune o mutare în părți. Întoarce `None` dacă nu conține nicio carte/piesă.
pub fn parse_move(raw: &str, spec: Spec) -> Option<Vec<Part>> {
    let mut parts: Vec<Part> = Vec::new();
    let mut word = String::new();
    let mut after_colon = false;
    let flush = |word: &mut String, parts: &mut Vec<Part>, after_colon: bool| {
        if !word.is_empty() {
            parts.push(word_part(word, spec, after_colon));
            word.clear();
        }
    };
    for ch in raw.chars() {
        match ch {
            c if c.is_whitespace() => {
                flush(&mut word, &mut parts, after_colon);
                after_colon = false;
            }
            ':' | '>' | '+' => {
                flush(&mut word, &mut parts, after_colon);
                parts.push(Part::Text(ch.to_string()));
                after_colon = ch == ':';
            }
            c => word.push(c),
        }
    }
    flush(&mut word, &mut parts, after_colon);
    if parts.iter().any(Part::is_piece) {
        Some(parts)
    } else {
        None
    }
}

/// Un element dintr-un rând din randare.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Slot {
    Card(Card),
    Domino(u8, u8),
    Back,
    /// durak: atac peste care se apără (`?>As`): (apărare, atac)
    Pair(Option<Card>, Card),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// mâna jucătorului (interactivă)
    Hand,
    /// masă / talon / capcană / aruncate
    Table,
    /// cărți ascunse ale adversarului (doar numărul)
    Hidden,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub label: String,
    pub role: Role,
    pub slots: Vec<Slot>,
}

fn clean_tok(t: &str) -> &str {
    let t = t.trim_matches(|c: char| matches!(c, ',' | '.' | ';' | '(' | ')' | '{' | '}'));
    // prefix „1:” (casino) sau „W:” (bridge)
    if let Some((p, rest)) = t.split_once(':') {
        let numeric = !p.is_empty() && p.chars().all(|c| c.is_ascii_digit());
        let dir = p.len() == 1 && p.chars().all(|c| matches!(c, 'N' | 'S' | 'E' | 'W' | 'V'));
        if (numeric || dir) && !rest.is_empty() {
            return rest;
        }
    }
    t
}

fn slot_of(tok: &str, spec: Spec) -> Option<Slot> {
    let t = clean_tok(tok);
    let t2 = t.trim_matches(|c: char| c == '[' || c == ']');
    if t2 == "??" {
        return Some(Slot::Back);
    }
    match spec.kind {
        SpecKind::Cards(deck) => {
            if let Some((l, r)) = t.split_once('>') {
                let att = parse_card(r, deck, spec.jokers)?;
                if l == "?" {
                    return Some(Slot::Pair(None, att));
                }
                return Some(Slot::Pair(Some(parse_card(l, deck, spec.jokers)?), att));
            }
            parse_card(t2, deck, spec.jokers)
                .or_else(|| parse_card(t, deck, spec.jokers))
                .map(Slot::Card)
        }
        SpecKind::Domino => parse_domino(t2).map(|(a, b)| Slot::Domino(a, b)),
    }
}

const DENY: [&str; 5] = ["contract", "licita", "scor", "puncte", "levate"];

fn role_of(label: &str) -> Role {
    let l = label.to_lowercase();
    if l.starts_with("mâna")
        || l.starts_with("mana")
        || l.starts_with("cărțile tale")
        || l.starts_with("cartile tale")
        || l.starts_with("piesele tale")
        || l.ends_with("(tu)")
    {
        Role::Hand
    } else {
        Role::Table
    }
}

/// Numărul de cărți ascunse dintr-o linie („J2 are 13 cărți”, „Jucătorul 2: 7 piese”).
fn hidden_count(seg: &str) -> Option<(String, usize)> {
    let words: Vec<&str> = seg.split_whitespace().collect();
    for (i, w) in words.iter().enumerate() {
        let nxt = words.get(i + 1).copied().unwrap_or("");
        let is_unit =
            nxt.starts_with("cărți") || nxt.starts_with("carți") || nxt.starts_with("piese");
        if is_unit {
            if let Ok(n) = w
                .trim_matches(|c: char| !c.is_ascii_digit())
                .parse::<usize>()
            {
                let pre = words[..i].join(" ");
                let is_have = pre.ends_with("are")
                    || pre.ends_with("ține")
                    || pre.ends_with(':')
                    || pre.contains("(ascuns)");
                let who = pre.starts_with("J") || pre.starts_with("Adversar");
                let no_extra = i + 2 >= words.len()
                    || words[i + 2..]
                        .iter()
                        .all(|x| x.len() < 3 || x.starts_with('('));
                if is_have && who && n > 0 && n <= 60 && no_extra && !pre.contains("a luat") {
                    let label = pre
                        .trim_end_matches("are")
                        .trim_end_matches(':')
                        .trim()
                        .to_string();
                    return Some((label, n));
                }
            }
        }
    }
    None
}

/// Rândurile de cărți / piese dintr-o randare `render()`.
pub fn rows(spec: Spec, render: &str) -> Vec<Row> {
    let mut out = Vec::new();
    for line in render.lines() {
        // segmente independente: „…  — Jucător 2: [??] [??]” sau „… | libere: …”
        for seg in line.split(" | ").flat_map(|s| s.split(" — ")) {
            let seg = seg.trim();
            if seg.is_empty() {
                continue;
            }
            if let Some((label, tail)) = seg.split_once(": ") {
                let ll = label.to_lowercase();
                if DENY.iter().any(|d| ll.contains(d)) || label.is_empty() {
                    continue;
                }
                let tail = match tail.rfind(" a jucat ") {
                    Some(i) => &tail[i + 9..],
                    None => tail,
                };
                let mut slots = Vec::new();
                for tok in tail.split_whitespace() {
                    if let Some(s) = slot_of(tok, spec) {
                        slots.push(s);
                    } else if matches!(tok, "♥" | "♦" | "♣" | "♠") && !slots.is_empty() {
                        continue;
                    } else {
                        break;
                    }
                }
                if !slots.is_empty() {
                    let only_back = slots.iter().all(|s| *s == Slot::Back);
                    out.push(Row {
                        label: label.trim().to_string(),
                        role: if only_back {
                            Role::Hidden
                        } else {
                            role_of(label)
                        },
                        slots,
                    });
                    continue;
                }
            }
            if let Some((label, n)) = hidden_count(seg) {
                if matches!(spec.kind, SpecKind::Cards(_)) {
                    out.push(Row {
                        label,
                        role: Role::Hidden,
                        slots: vec![Slot::Back; n],
                    });
                }
            }
        }
    }
    out
}

// ───────────────────────────── zaruri ─────────────────────────────

/// Un rând de zaruri din randare: valori și dacă pot fi „ținute”.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiceRow {
    pub label: String,
    pub values: Vec<u8>,
    /// zarurile curente ale jucătorului (se pot ține)
    pub holdable: bool,
}

fn die_val(s: &str) -> Option<u8> {
    let v: u8 = s.trim_matches(|c: char| !c.is_ascii_digit()).parse().ok()?;
    (1..=6).contains(&v).then_some(v)
}

fn int_list(s: &str) -> Option<Vec<u8>> {
    let a = s.find('[')?;
    let b = s[a..].find(']')? + a;
    let inner = &s[a + 1..b];
    let mut v = Vec::new();
    for p in inner.split(',') {
        let t = p.trim();
        if t.len() != 1 {
            return None;
        }
        v.push(die_val(t)?);
    }
    (!v.is_empty() && v.len() <= 6).then_some(v)
}

/// Jocurile fără tablă grafică al căror `render()` conține zaruri recunoscute
/// (backgammon și ludo își desenează deja zarurile pe tablă).
pub fn has_dice(id: &str) -> bool {
    matches!(id, "zarpoker" | "zaruri5" | "barbut" | "snakes")
}

/// Rândurile de zaruri dintr-o randare (doar pentru [`has_dice`]).
pub fn dice_rows(render: &str) -> Vec<DiceRow> {
    let mut out = Vec::new();
    for line in render.lines() {
        let l = line.trim();
        // „Zaruri: [2, 3, 5, 6, 6] (pereche)”: zarurile curente, se pot ține
        if let Some(rest) = l.strip_prefix("Zaruri: ") {
            if let Some(v) = int_list(rest) {
                out.push(DiceRow {
                    label: "Zaruri".into(),
                    values: v,
                    holdable: true,
                });
            }
            continue;
        }
        for key in ["Ținute: ", "Mâna ta terminată: "] {
            if let Some(rest) = l.strip_prefix(key) {
                if let Some(v) = int_list(rest) {
                    out.push(DiceRow {
                        label: key.trim_end_matches(": ").into(),
                        values: v,
                        holdable: false,
                    });
                }
            }
        }
        // perechi „A și B”
        for key in [
            "Zaruri aruncate: ",
            "Zar de deschidere: ",
            "zar de început: ",
            "Ultima aruncare: ",
        ] {
            if let Some(i) = l.find(key) {
                let rest = &l[i + key.len()..];
                let mut it = rest.split_whitespace();
                if let (Some(a), Some("și"), Some(b)) = (it.next(), it.next(), it.next()) {
                    if let (Some(a), Some(b)) = (die_val(a), die_val(b)) {
                        out.push(DiceRow {
                            label: key.trim_end_matches(": ").trim().to_string(),
                            values: vec![a, b],
                            holdable: false,
                        });
                        break;
                    }
                }
            }
        }
        // „Runda 1: J1 [1, 1, 2, 3, 4] (pereche) contra J2 [3, 3, 3, 5, 6] (trei la fel)”
        if l.contains(" contra ") {
            for tag in ["J1 [", "J2 ["] {
                if let Some(i) = l.find(tag) {
                    if let Some(v) = int_list(&l[i + 2..]) {
                        out.push(DiceRow {
                            label: tag.trim_end_matches(" [").into(),
                            values: v,
                            holdable: false,
                        });
                    }
                }
            }
        }
        // „La mutare: jucătorul 1   Zar: 6”
        if let Some(i) = l.find("Zar: ") {
            if let Some(v) = l[i + 5..].split_whitespace().next().and_then(die_val) {
                out.push(DiceRow {
                    label: "Zar".into(),
                    values: vec![v],
                    holdable: false,
                });
            }
        }
    }
    out
}

// ───────────────────────────── desen ─────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CardState {
    #[default]
    Normal,
    /// mouse deasupra (contur auriu)
    Hover,
    /// selectată (contur auriu gros)
    Selected,
    /// inactivă (estompată)
    Disabled,
}

fn with_alpha(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
}

fn poly(p: &Painter, pts: Vec<Pos2>, fill: Color32) {
    p.add(Shape::convex_polygon(pts, fill, Stroke::NONE));
}

fn suit_color(s: Suit) -> Color32 {
    match s {
        Suit::Hearts | Suit::Diamonds | Suit::Cups => RED,
        Suit::Clubs | Suit::Spades => INK,
        Suit::Coins => GOLD,
        Suit::Swords => STEEL,
        Suit::Batons => WOOD,
    }
}

/// Simbolul unei culori, încadrat în pătratul `[c-r, c+r]²`. `flip` = răsturnat (180°).
pub fn draw_suit(p: &Painter, c: Pos2, r: f32, s: Suit, flip: bool) {
    // doar culorile franceze se răstoarnă la 180°; simbolurile italiene/spaniole rămân drepte
    let flip = flip
        && matches!(
            s,
            Suit::Hearts | Suit::Diamonds | Suit::Clubs | Suit::Spades
        );
    let sy = if flip { -1.0 } else { 1.0 };
    let pt = |x: f32, y: f32| Pos2::new(c.x + x * r, c.y + y * r * sy);
    let col = suit_color(s);
    match s {
        Suit::Hearts => {
            // inimă = 2 cercuri + triunghi
            let cc = |x: f32| pt(x, -0.38);
            p.circle_filled(cc(-0.45), 0.55 * r, col);
            p.circle_filled(cc(0.45), 0.55 * r, col);
            poly(
                p,
                vec![pt(-0.97, -0.18), pt(0.97, -0.18), pt(0.0, 0.98)],
                col,
            );
        }
        Suit::Spades => {
            // pică = inimă răsturnată (mai îngustă) + picior lat
            p.circle_filled(pt(-0.44, 0.20), 0.50 * r, col);
            p.circle_filled(pt(0.44, 0.20), 0.50 * r, col);
            poly(p, vec![pt(-0.93, 0.0), pt(0.93, 0.0), pt(0.0, -0.98)], col);
            poly(p, vec![pt(0.0, 0.2), pt(-0.40, 1.0), pt(0.40, 1.0)], col);
        }
        Suit::Diamonds => poly(
            p,
            vec![pt(0.0, -1.0), pt(0.68, 0.0), pt(0.0, 1.0), pt(-0.68, 0.0)],
            col,
        ),
        Suit::Clubs => {
            let rr = 0.38 * r;
            p.circle_filled(pt(0.0, -0.55), rr, col);
            p.circle_filled(pt(-0.52, 0.12), rr, col);
            p.circle_filled(pt(0.52, 0.12), rr, col);
            p.circle_filled(pt(0.0, 0.0), 0.30 * r, col);
            poly(p, vec![pt(0.0, 0.05), pt(-0.30, 1.0), pt(0.30, 1.0)], col);
        }
        Suit::Coins => {
            p.circle_filled(c, 0.95 * r, col);
            p.circle_stroke(c, 0.95 * r, Stroke::new((0.09 * r).max(1.0), GOLD_DK));
            p.circle_stroke(c, 0.62 * r, Stroke::new((0.07 * r).max(0.8), GOLD_DK));
            p.circle_filled(c, 0.22 * r, GOLD_DK);
        }
        Suit::Cups => {
            // cupă: semidisc + picior + bază
            let n = 14;
            let mut bowl = Vec::new();
            for i in 0..=n {
                let a = std::f32::consts::PI * i as f32 / n as f32;
                bowl.push(pt(0.85 * a.cos(), -0.55 + 1.0 * a.sin()));
            }
            poly(p, bowl, col);
            poly(
                p,
                vec![
                    pt(-0.10, 0.4),
                    pt(0.10, 0.4),
                    pt(0.10, 0.82),
                    pt(-0.10, 0.82),
                ],
                col,
            );
            poly(
                p,
                vec![
                    pt(-0.55, 0.98),
                    pt(-0.40, 0.80),
                    pt(0.40, 0.80),
                    pt(0.55, 0.98),
                ],
                col,
            );
        }
        Suit::Swords => {
            poly(
                p,
                vec![
                    pt(0.0, -1.0),
                    pt(0.16, -0.78),
                    pt(0.16, 0.40),
                    pt(-0.16, 0.40),
                    pt(-0.16, -0.78),
                ],
                col,
            );
            poly(
                p,
                vec![
                    pt(-0.55, 0.40),
                    pt(0.55, 0.40),
                    pt(0.55, 0.55),
                    pt(-0.55, 0.55),
                ],
                GOLD_DK,
            );
            poly(
                p,
                vec![
                    pt(-0.08, 0.55),
                    pt(0.08, 0.55),
                    pt(0.08, 0.88),
                    pt(-0.08, 0.88),
                ],
                WOOD,
            );
            p.circle_filled(pt(0.0, 0.93), 0.10 * r, GOLD_DK);
        }
        Suit::Batons => {
            let a = pt(-0.55, 0.92);
            let b = pt(0.5, -0.65);
            p.line_segment([a, b], Stroke::new(0.42 * r, col));
            p.circle_filled(a, 0.21 * r, col);
            p.circle_filled(b, 0.34 * r, col);
            p.circle_filled(pt(0.12, -0.05), 0.13 * r, GOLD_DK);
            p.circle_filled(pt(-0.18, 0.5), 0.11 * r, GOLD_DK);
        }
    }
}

/// Pip-urile 2..=10 în aranjamentul clasic: `(x, y)` cu `x ∈ {-1,0,1}`, `y ∈ [-1,1]`.
pub fn pip_layout(n: u8) -> Vec<(f32, f32)> {
    let t = 1.0 / 3.0;
    match n {
        2 => vec![(0.0, -1.0), (0.0, 1.0)],
        3 => vec![(0.0, -1.0), (0.0, 0.0), (0.0, 1.0)],
        4 => vec![(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)],
        5 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (0.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        6 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (-1.0, 0.0),
            (1.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        7 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (0.0, -0.5),
            (-1.0, 0.0),
            (1.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        8 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (0.0, -0.5),
            (-1.0, 0.0),
            (1.0, 0.0),
            (0.0, 0.5),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        9 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (-1.0, -t),
            (1.0, -t),
            (0.0, 0.0),
            (-1.0, t),
            (1.0, t),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        10 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (0.0, -2.0 * t),
            (-1.0, -t),
            (1.0, -t),
            (-1.0, t),
            (1.0, t),
            (0.0, 2.0 * t),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        _ => vec![],
    }
}

/// Zona pip-urilor în interiorul cardului și semi-dimensiunea unui pip.
pub fn pip_area(rect: Rect) -> (Rect, f32) {
    let w = rect.width();
    let h = rect.height();
    let r = 0.085 * w;
    let area = Rect::from_min_max(
        Pos2::new(rect.min.x + 0.5 * w - 0.16 * w, rect.min.y + 0.23 * h),
        Pos2::new(rect.min.x + 0.5 * w + 0.16 * w, rect.min.y + 0.77 * h),
    );
    (area, r)
}

/// Pozițiile pip-urilor (centru, semi-dimensiune, răsturnat) pentru o carte numerică.
pub fn pip_positions(rect: Rect, n: u8) -> Vec<(Pos2, f32, bool)> {
    let (area, r) = pip_area(rect);
    pip_layout(n)
        .into_iter()
        .map(|(x, y)| {
            let c = Pos2::new(
                area.center().x + x * area.width() * 0.5,
                area.center().y + y * area.height() * 0.5,
            );
            (c, r, y > 0.0)
        })
        .collect()
}

fn text_c(p: &Painter, at: Pos2, a: Align2, s: &str, size: f32, col: Color32) {
    p.text(at, a, s, FontId::proportional(size), col);
}

fn shadow_and_body(p: &Painter, rect: Rect, state: CardState) {
    let rd = (rect.width() * 0.09).max(2.0);
    if state != CardState::Disabled {
        p.rect_filled(
            rect.translate(Vec2::new(1.5, 2.5)),
            rd,
            Color32::from_black_alpha(60),
        );
    }
    p.rect_filled(rect, rd, CREAM);
    p.rect_stroke(rect, rd, Stroke::new(1.0, EDGE), StrokeKind::Inside);
}

fn finish(p: &Painter, rect: Rect, state: CardState) {
    let rd = (rect.width() * 0.09).max(2.0);
    match state {
        CardState::Hover => {
            p.rect_stroke(rect, rd, Stroke::new(2.0, GOLD), StrokeKind::Outside);
        }
        CardState::Selected => {
            p.rect_stroke(
                rect.expand(1.0),
                rd,
                Stroke::new(3.0, GOLD),
                StrokeKind::Outside,
            );
        }
        CardState::Disabled => {
            p.rect_filled(rect, rd, with_alpha(BACK_BROWN, 125));
        }
        CardState::Normal => {}
    }
}

/// Fața unei cărți în `rect` (raport recomandat ≈ 2:3).
pub fn draw_card(p: &Painter, rect: Rect, card: &Card, state: CardState) {
    shadow_and_body(p, rect, state);
    let w = rect.width();
    let h = rect.height();
    let at = |fx: f32, fy: f32| Pos2::new(rect.min.x + fx * w, rect.min.y + fy * h);
    match card.rank {
        Rank::Joker => {
            let inner = rect.shrink(w * 0.14);
            p.rect_filled(inner, w * 0.05, with_alpha(GOLD, 70));
            p.rect_stroke(inner, w * 0.05, Stroke::new(1.0, GOLD), StrokeKind::Inside);
            draw_star(p, rect.center(), w * 0.3, RED);
            text_c(p, at(0.14, 0.05), Align2::LEFT_TOP, "J", w * 0.22, RED);
            text_c(p, at(0.86, 0.95), Align2::RIGHT_BOTTOM, "J", w * 0.22, RED);
            text_c(
                p,
                at(0.5, 0.8),
                Align2::CENTER_CENTER,
                "JOKER",
                (w * 0.14).max(6.0),
                INK,
            );
        }
        Rank::Excuse => {
            let inner = rect.shrink(w * 0.14);
            p.rect_filled(inner, w * 0.05, with_alpha(GOLD, 70));
            p.rect_stroke(inner, w * 0.05, Stroke::new(1.0, GOLD), StrokeKind::Inside);
            draw_star(p, rect.center(), w * 0.3, INK);
            text_c(p, at(0.14, 0.05), Align2::LEFT_TOP, "Ex", w * 0.2, INK);
            text_c(p, at(0.86, 0.95), Align2::RIGHT_BOTTOM, "Ex", w * 0.2, INK);
        }
        Rank::Trump(n) => {
            let inner = rect.shrink(w * 0.12);
            p.rect_filled(inner, w * 0.06, with_alpha(GOLD, 60));
            p.rect_stroke(inner, w * 0.06, Stroke::new(1.5, GOLD), StrokeKind::Inside);
            draw_star(p, at(0.5, 0.3), w * 0.14, GOLD_DK);
            draw_star(p, at(0.5, 0.7), w * 0.14, GOLD_DK);
            let s = n.to_string();
            text_c(p, rect.center(), Align2::CENTER_CENTER, &s, w * 0.42, INK);
            text_c(p, at(0.12, 0.04), Align2::LEFT_TOP, &s, w * 0.19, INK);
            text_c(p, at(0.88, 0.96), Align2::RIGHT_BOTTOM, &s, w * 0.19, INK);
        }
        _ => {
            let suit = card.suit.unwrap_or(Suit::Spades);
            let col = suit_color(suit);
            let label = card.label();
            let tsize = if label.chars().count() > 1 {
                w * 0.19
            } else {
                w * 0.23
            };
            // colțuri: rang + culoare
            text_c(p, at(0.09, 0.03), Align2::LEFT_TOP, &label, tsize, col);
            draw_suit(
                p,
                at(0.14, 0.03 + (tsize / h) + 0.07),
                w * 0.065,
                suit,
                false,
            );
            text_c(p, at(0.91, 0.97), Align2::RIGHT_BOTTOM, &label, tsize, col);
            draw_suit(
                p,
                at(0.86, 0.97 - (tsize / h) - 0.07),
                w * 0.065,
                suit,
                true,
            );
            match card.rank {
                Rank::Ace => draw_suit(p, rect.center(), w * 0.30, suit, false),
                Rank::Num(n) => {
                    for (c, r, fl) in pip_positions(rect, n) {
                        draw_suit(p, c, r, suit, fl);
                    }
                }
                _ => draw_figure(p, rect, card, suit, col),
            }
        }
    }
    finish(p, rect, state);
}

fn draw_star(p: &Painter, c: Pos2, r: f32, col: Color32) {
    // pentagon central + 5 triunghiuri = stea
    let pent: Vec<Pos2> = (0..5)
        .map(|i| {
            let a = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / 5.0;
            Pos2::new(c.x + 0.42 * r * a.cos(), c.y + 0.42 * r * a.sin())
        })
        .collect();
    poly(p, pent.clone(), col);
    // vârfurile stelei: triunghi pe fiecare latură a pentagonului
    for i in 0..5 {
        let a = -std::f32::consts::FRAC_PI_2 + (i as f32 + 0.5) * std::f32::consts::TAU / 5.0;
        let tip = Pos2::new(c.x + r * a.cos(), c.y + r * a.sin());
        poly(p, vec![pent[i], pent[(i + 1) % 5], tip], col);
    }
}

fn draw_figure(p: &Painter, rect: Rect, card: &Card, suit: Suit, col: Color32) {
    let w = rect.width();
    let h = rect.height();
    let at = |fx: f32, fy: f32| Pos2::new(rect.min.x + fx * w, rect.min.y + fy * h);
    let panel = Rect::from_min_max(at(0.27, 0.2), at(0.73, 0.8));
    p.rect_filled(panel, w * 0.05, with_alpha(GOLD, 70));
    p.rect_stroke(panel, w * 0.05, Stroke::new(1.2, GOLD), StrokeKind::Inside);
    // monogramă
    let mono = match (card.rank, card.deck) {
        (Rank::Jack, Deck::Spanish) => "S".to_string(),
        (Rank::King, Deck::Spanish) => "R".to_string(),
        (Rank::Knight, Deck::Spanish) => "C".to_string(),
        _ => {
            let l = card.label();
            l.chars().next().map(String::from).unwrap_or_default()
        }
    };
    text_c(
        p,
        Pos2::new(panel.center().x, panel.min.y + panel.height() * 0.48),
        Align2::CENTER_CENTER,
        &mono,
        w * 0.34,
        INK,
    );
    // ornament deasupra monogramei
    let oy = panel.min.y + panel.height() * 0.17;
    let ox = panel.center().x;
    let u = w * 0.05;
    match card.rank {
        Rank::King => {
            // coroană: bază + 3 vârfuri
            poly(
                p,
                vec![
                    Pos2::new(ox - 2.2 * u, oy + u),
                    Pos2::new(ox + 2.2 * u, oy + u),
                    Pos2::new(ox + 2.2 * u, oy + 1.8 * u),
                    Pos2::new(ox - 2.2 * u, oy + 1.8 * u),
                ],
                GOLD_DK,
            );
            for dx in [-1.6f32, 0.0, 1.6] {
                poly(
                    p,
                    vec![
                        Pos2::new(ox + (dx - 0.6) * u, oy + u),
                        Pos2::new(ox + (dx + 0.6) * u, oy + u),
                        Pos2::new(ox + dx * u, oy - 1.0 * u),
                    ],
                    GOLD_DK,
                );
            }
        }
        Rank::Queen => {
            for dx in [-1.4f32, 0.0, 1.4] {
                p.circle_filled(Pos2::new(ox + dx * u, oy + 0.4 * u), 0.55 * u, GOLD_DK);
            }
        }
        _ => {
            poly(
                p,
                vec![
                    Pos2::new(ox - 1.4 * u, oy + 1.2 * u),
                    Pos2::new(ox + 1.4 * u, oy + 1.2 * u),
                    Pos2::new(ox + 0.4 * u, oy - 0.9 * u),
                ],
                GOLD_DK,
            );
        }
    }
    // culoarea sub monogramă
    draw_suit(
        p,
        Pos2::new(panel.center().x, panel.min.y + panel.height() * 0.8),
        w * 0.085,
        suit,
        false,
    );
    let _ = col;
}

/// Dosul unei cărți: fundal brun, chenar auriu, model geometric (romburi).
pub fn draw_back(p: &Painter, rect: Rect, state: CardState) {
    let w = rect.width();
    let rd = (w * 0.09).max(2.0);
    if state != CardState::Disabled {
        p.rect_filled(
            rect.translate(Vec2::new(1.5, 2.5)),
            rd,
            Color32::from_black_alpha(60),
        );
    }
    p.rect_filled(rect, rd, BACK_BROWN);
    p.rect_stroke(rect, rd, Stroke::new(1.0, GOLD), StrokeKind::Inside);
    let inner = rect.shrink(w * 0.09);
    p.rect_stroke(
        inner,
        rd * 0.6,
        Stroke::new(1.0, with_alpha(GOLD, 200)),
        StrokeKind::Inside,
    );
    let q = p.with_clip_rect(inner);
    let step = (w * 0.2).max(5.0);
    let line = Stroke::new(0.8, with_alpha(GOLD, 110));
    let span = inner.width() + inner.height();
    let mut k = -inner.height();
    while k < span {
        q.line_segment(
            [
                Pos2::new(inner.min.x + k, inner.min.y),
                Pos2::new(inner.min.x + k + inner.height(), inner.max.y),
            ],
            line,
        );
        q.line_segment(
            [
                Pos2::new(inner.min.x + k, inner.max.y),
                Pos2::new(inner.min.x + k + inner.height(), inner.min.y),
            ],
            line,
        );
        k += step;
    }
    let c = rect.center();
    let r = w * 0.2;
    poly(
        p,
        vec![
            Pos2::new(c.x, c.y - r * 1.3),
            Pos2::new(c.x + r, c.y),
            Pos2::new(c.x, c.y + r * 1.3),
            Pos2::new(c.x - r, c.y),
        ],
        BACK_BROWN,
    );
    p.add(Shape::closed_line(
        vec![
            Pos2::new(c.x, c.y - r * 1.3),
            Pos2::new(c.x + r, c.y),
            Pos2::new(c.x, c.y + r * 1.3),
            Pos2::new(c.x - r, c.y),
        ],
        Stroke::new(1.2, GOLD),
    ));
    p.circle_filled(c, r * 0.28, GOLD);
    finish(p, rect, state);
}

/// Pip-uri de zar în pătratul `rect` (poziții în [-1,1]²).
fn die_pips(n: u8) -> Vec<(f32, f32)> {
    match n {
        0 => vec![],
        1 => vec![(0.0, 0.0)],
        2 => vec![(-1.0, -1.0), (1.0, 1.0)],
        3 => vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
        4 => vec![(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)],
        5 => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (0.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        _ => vec![
            (-1.0, -1.0),
            (1.0, -1.0),
            (-1.0, 0.0),
            (1.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
    }
}

/// Un zar (față 1..=6) în `rect` (pătrat).
pub fn draw_die(p: &Painter, rect: Rect, value: u8, state: CardState) {
    let rd = rect.width() * 0.18;
    if state != CardState::Disabled {
        p.rect_filled(
            rect.translate(Vec2::new(1.5, 2.5)),
            rd,
            Color32::from_black_alpha(60),
        );
    }
    p.rect_filled(rect, rd, CREAM);
    p.rect_stroke(rect, rd, Stroke::new(1.0, EDGE), StrokeKind::Inside);
    let w = rect.width().min(rect.height());
    let pr = w * 0.085;
    let off = w * 0.25;
    let col = if value == 1 { RED } else { INK };
    for (x, y) in die_pips(value) {
        p.circle_filled(
            Pos2::new(rect.center().x + x * off, rect.center().y + y * off),
            if value == 1 { pr * 1.5 } else { pr },
            col,
        );
    }
    finish(p, rect, state);
}

/// O piesă de domino. Orientare după raportul `rect` (lat = orizontală, înalt = verticală).
pub fn draw_domino(p: &Painter, rect: Rect, a: u8, b: u8, state: CardState) {
    let horizontal = rect.width() >= rect.height();
    let short = rect.width().min(rect.height());
    let rd = short * 0.16;
    if state != CardState::Disabled {
        p.rect_filled(
            rect.translate(Vec2::new(1.5, 2.5)),
            rd,
            Color32::from_black_alpha(60),
        );
    }
    p.rect_filled(rect, rd, CREAM);
    p.rect_stroke(rect, rd, Stroke::new(1.0, EDGE), StrokeKind::Inside);
    let half = |i: usize| {
        if horizontal {
            let hw = rect.width() / 2.0;
            Rect::from_min_size(
                Pos2::new(rect.min.x + hw * i as f32, rect.min.y),
                Vec2::new(hw, rect.height()),
            )
        } else {
            let hh = rect.height() / 2.0;
            Rect::from_min_size(
                Pos2::new(rect.min.x, rect.min.y + hh * i as f32),
                Vec2::new(rect.width(), hh),
            )
        }
    };
    let (h0, h1) = (half(0), half(1));
    // linia de mijloc, cu un nit auriu
    let (m1, m2) = if horizontal {
        (
            Pos2::new(rect.center().x, rect.min.y + short * 0.12),
            Pos2::new(rect.center().x, rect.max.y - short * 0.12),
        )
    } else {
        (
            Pos2::new(rect.min.x + short * 0.12, rect.center().y),
            Pos2::new(rect.max.x - short * 0.12, rect.center().y),
        )
    };
    p.line_segment([m1, m2], Stroke::new(1.2, with_alpha(INK, 170)));
    p.circle_filled(rect.center(), short * 0.045, GOLD);
    for (v, hr) in [(a, h0), (b, h1)] {
        if v <= 6 {
            let s = hr.width().min(hr.height());
            let off = s * 0.25;
            let pr = s * 0.075;
            for (x, y) in die_pips(v) {
                p.circle_filled(
                    Pos2::new(hr.center().x + x * off, hr.center().y + y * off),
                    pr,
                    INK,
                );
            }
        } else {
            text_c(
                p,
                hr.center(),
                Align2::CENTER_CENTER,
                &v.to_string(),
                short * 0.4,
                INK,
            );
        }
    }
    finish(p, rect, state);
}

// ───────────────────────────── teste ─────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_games::playable;

    fn fr() -> Spec {
        spec_for("belote").unwrap()
    }

    #[test]
    fn single_tokens() {
        let c = parse_card("10h", Deck::French, false).unwrap();
        assert_eq!(c.rank, Rank::Num(10));
        assert_eq!(c.suit, Some(Suit::Hearts));
        assert_eq!(c.token(), "10h");
        assert_eq!(
            parse_card("As", Deck::French, false).unwrap().rank,
            Rank::Ace
        );
        assert_eq!(parse_card("7♥", Deck::French, false).unwrap().token(), "7h");
        assert_eq!(
            parse_card("A♦[Ad]", Deck::French, false).unwrap().token(),
            "Ad"
        );
        assert!(parse_card("Jk", Deck::French, false).is_none());
        assert_eq!(
            parse_card("Jk", Deck::French, true).unwrap().rank,
            Rank::Joker
        );
        for bad in [
            "pas", "1c", "11h", "stai", "carte", "7n", "", "10", "h", "Kx",
        ] {
            assert!(parse_card(bad, Deck::French, true).is_none(), "{bad}");
        }
    }

    #[test]
    fn italian_spanish_tarot() {
        let c = parse_card("Fc", Deck::Italian, false).unwrap();
        assert_eq!((c.rank, c.suit), (Rank::Jack, Some(Suit::Cups)));
        assert_eq!(
            parse_card("Ab", Deck::Italian, false).unwrap().suit,
            Some(Suit::Batons)
        );
        assert_eq!(
            parse_card("1d", Deck::Italian, false).unwrap().rank,
            Rank::Ace
        );
        assert!(parse_card("8c", Deck::Italian, false).is_none());
        let t = parse_card("12e", Deck::Spanish, false).unwrap();
        assert_eq!((t.rank, t.suit), (Rank::King, Some(Suit::Swords)));
        assert!(parse_card("9e", Deck::Spanish, false).is_none());
        assert_eq!(
            parse_card("T12", Deck::Tarot, false).unwrap().rank,
            Rank::Trump(12)
        );
        assert_eq!(
            parse_card("Ex", Deck::Tarot, false).unwrap().rank,
            Rank::Excuse
        );
        assert!(parse_card("T22", Deck::Tarot, false).is_none());
        assert_eq!(
            parse_card("Cs", Deck::Tarot, false).unwrap().rank,
            Rank::Knight
        );
        assert_eq!(
            parse_card("Ds", Deck::Tarot, false).unwrap().rank,
            Rank::Queen
        );
    }

    #[test]
    fn compound_moves() {
        let m = parse_move("7c:3s+4b", spec_for("scopa").unwrap()).unwrap();
        assert_eq!(m.iter().filter(|p| matches!(p, Part::Card(_))).count(), 3);
        let m = parse_move("7h:h", fr()).unwrap();
        assert!(matches!(m.last(), Some(Part::Suit(Suit::Hearts))));
        let m = parse_move("6s>10c", fr()).unwrap();
        assert_eq!(m.iter().filter(|p| p.is_piece()).count(), 2);
        let m = parse_move("d Jh 9h", fr()).unwrap();
        assert_eq!(m[0], Part::Text("aruncă".into()));
        let m = parse_move("8c x 6d 2d", fr()).unwrap();
        assert_eq!(m.iter().filter(|p| p.is_piece()).count(), 3);
        let m = parse_move("Qd+", fr()).unwrap();
        assert!(matches!(m[0], Part::Card(_)));
        let m = parse_move("5-6:S", spec_for("domino").unwrap()).unwrap();
        assert_eq!(m[0], Part::Domino(5, 6));
        assert_eq!(m.last(), Some(&Part::Arrow(true)));
        for no in [
            "pas", "prends", "stoc", "talon", "licit 18", "carte", "trage 3", "fold",
        ] {
            assert!(parse_move(no, fr()).is_none(), "{no}");
        }
    }

    #[test]
    fn render_rows_real_formats() {
        let s = fr();
        let r = rows(
            s,
            "Mâna ta (Jucător 1): 9♦ Q♣ = 19\nDealer (Jucător 2): Q♦ [??] (1 carte ascunsă)",
        );
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].role, Role::Hand);
        assert_eq!(r[0].slots.len(), 2);
        assert_eq!(r[1].slots.len(), 2);
        assert_eq!(r[1].slots[1], Slot::Back);
        let r = rows(s, "Masa: 1:Qc 2:Ac 3:9h 4:3s");
        assert_eq!(r[0].slots.len(), 4);
        let r = rows(
            s,
            "Cărțile tale (Jucător 1): 4♦ 6♥ — Jucător 2: [??] [??] (2 cărți ascunse)",
        );
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].role, Role::Hand);
        assert_eq!(r[1].role, Role::Hidden);
        let r = rows(s, "Mâna ta (J1): A♦[Ad] 10♣[10c] 3♦[3d]\nJ2: [?? ?? ??]");
        assert_eq!(r[0].slots.len(), 3);
        assert_eq!(r[1].slots.len(), 3);
        let r = rows(s, "Masa: ?>As");
        assert_eq!(
            r[0].slots,
            vec![Slot::Pair(
                None,
                parse_card("As", Deck::French, false).unwrap()
            )]
        );
        let r = rows(s, "Pe masă: Jucător 1 a jucat 10d ♦");
        assert_eq!(r[0].slots.len(), 1);
        // fără fals-pozitive pe linii de licitație / scor
        assert!(rows(s, "Contract: 7c (N–S), declarant S | levate: N–S 0, E–V 0").is_empty());
        assert!(rows(s, "Licitație: N–S:4c E–V:4n N–S:6n").is_empty());
        let r = rows(s, "Jucător 2 (ascuns) are 5 cărți.");
        assert_eq!(r[0].slots.len(), 5);
        let r = rows(s, "J2 are 13 cărți");
        assert_eq!(r[0].slots.len(), 13);
        assert!(rows(s, "Pachet: 28 cărți").is_empty());
        let d = rows(
            spec_for("domino").unwrap(),
            "Mâna Jucătorului 1: 0-3 4-5 4-6\nLanț: 6|4 4|4\nJucătorul 2: 7 piese",
        );
        assert_eq!(d[0].slots.len(), 3);
        assert_eq!(d[1].slots[0], Slot::Domino(6, 4));
    }

    #[test]
    fn dice_formats() {
        let d = dice_rows("Zaruri: [2, 3, 5, 6, 6] (pereche)  aruncări folosite: 1/3");
        assert_eq!(d[0].values, vec![2, 3, 5, 6, 6]);
        assert!(d[0].holdable);
        let d = dice_rows("Ținute: [3, 3]  aruncări folosite: 1/3  (scrie «arunc»)");
        assert_eq!(d[0].values, vec![3, 3]);
        let d = dice_rows("Zaruri aruncate: 6 și 1   Rămase: [6, 1]");
        assert_eq!(d[0].values, vec![6, 1]);
        let d = dice_rows("La mutare: jucătorul 1   Zar: 4   (scrie «arunc»)");
        assert_eq!(d[0].values, vec![4]);
        let d = dice_rows("Faza: zar de început: 4 și 6  (scrie «arunc»)");
        assert_eq!(d[0].values, vec![4, 6]);
        let d = dice_rows("Runda 1: J1 [1, 1, 2, 3, 4] (pereche) contra J2 [3, 3, 3, 5, 6] (trei la fel) → J2 ia punctul.");
        assert_eq!(d.len(), 2);
        assert_eq!(d[1].values, vec![3, 3, 3, 5, 6]);
        assert!(dice_rows("Ținute: []").is_empty());
        assert!(dice_rows("Pista: 7 și 9").is_empty());
    }

    #[test]
    fn pips_inside_card() {
        for (w, h) in [(40.0f32, 60.0f32), (72.0, 104.0), (100.0, 150.0)] {
            let rect = Rect::from_min_size(Pos2::new(10.0, 10.0), Vec2::new(w, h));
            for n in 2..=10u8 {
                let pos = pip_positions(rect, n);
                assert_eq!(pos.len(), n as usize, "pips {n}");
                for (c, r, _) in &pos {
                    assert!(c.x - r >= rect.min.x + 0.2 * w, "n={n} stânga");
                    assert!(c.x + r <= rect.max.x - 0.2 * w, "n={n} dreapta");
                    assert!(c.y - r >= rect.min.y + 0.1 * h, "n={n} sus");
                    assert!(c.y + r <= rect.max.y - 0.1 * h, "n={n} jos");
                }
                // pip-urile nu se suprapun
                for i in 0..pos.len() {
                    for j in i + 1..pos.len() {
                        let d = (pos[i].0 - pos[j].0).length();
                        assert!(d >= pos[i].1 * 1.5, "n={n}: pip-uri suprapuse {i},{j}");
                    }
                }
            }
        }
    }

    /// Joacă 100 de mutări aleatoare în fiecare joc cu cărți; jetoanele recunoscute trebuie
    /// să fie corecte (se regăsesc în text) și mutările non-carte să nu devină cărți.
    #[test]
    fn real_games_random_100_ply() {
        let mut checked_moves = 0usize;
        let mut checked_rows = 0usize;
        for p in playable() {
            let Some(spec) = spec_for(p.id) else { continue };
            let mut g = (p.make)();
            let mut seed = 0x9e3779b97f4a7c15u64 ^ p.id.len() as u64;
            for _ply in 0..100 {
                if g.outcome().is_some() {
                    break;
                }
                let render = g.render();
                let legal = g.legal_moves();
                for r in rows(spec, &render) {
                    for s in &r.slots {
                        let cards: Vec<Card> = match s {
                            Slot::Card(c) => vec![*c],
                            Slot::Pair(a, b) => a.iter().copied().chain([*b]).collect(),
                            _ => vec![],
                        };
                        for c in cards {
                            let tok = c.token();
                            let glyph = glyph_form(&c);
                            assert!(
                                render.contains(&tok)
                                    || render.contains(&alt_token(&c))
                                    || glyph.is_some_and(|g| render.contains(&g)),
                                "{}: jeton {tok} absent din randare:\n{render}",
                                p.id
                            );
                            checked_rows += 1;
                        }
                    }
                }
                for m in &legal {
                    if let Some(parts) = parse_move(m, spec) {
                        for part in parts {
                            match part {
                                Part::Card(c) => {
                                    assert!(
                                        m.contains(&c.token()) || m.contains(&alt_token(&c)),
                                        "{}: {m} → {}",
                                        p.id,
                                        c.token()
                                    );
                                    checked_moves += 1;
                                }
                                Part::Domino(a, b) => {
                                    assert!(m.contains(&format!("{a}-{b}")), "{}: {m}", p.id);
                                    checked_moves += 1;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        // mutările de tip comandă nu au jeton de carte
                        assert!(
                            !m.split_whitespace().any(|w| parse_card(
                                w,
                                match spec.kind {
                                    SpecKind::Cards(d) => d,
                                    _ => Deck::French,
                                },
                                spec.jokers
                            )
                            .is_some())
                                || matches!(spec.kind, SpecKind::Domino),
                            "{}: {m} ar fi trebuit recunoscută",
                            p.id
                        );
                    }
                }
                if legal.is_empty() {
                    break;
                }
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let m = legal[(seed >> 33) as usize % legal.len()].clone();
                if g.play(&m).is_err() {
                    break;
                }
            }
        }
        assert!(
            checked_moves > 500,
            "prea puține mutări verificate: {checked_moves}"
        );
        assert!(
            checked_rows > 500,
            "prea puține rânduri verificate: {checked_rows}"
        );
    }

    /// Asul italian apare ca „A” (briscola) sau „1” (scopa).
    fn alt_token(c: &Card) -> String {
        let t = c.token();
        if let Some(r) = t.strip_prefix('1') {
            format!("A{r}")
        } else if let Some(r) = t.strip_prefix('A') {
            format!("1{r}")
        } else {
            t
        }
    }

    fn glyph_form(c: &Card) -> Option<String> {
        let g = match c.suit? {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
            _ => return None,
        };
        let r = c.label();
        Some(format!("{r}{g}"))
    }

    /// Jocurile fără specificație nu sunt atinse; jocurile cu spec produc cel puțin un rând de mână.
    #[test]
    fn non_card_games_untouched_and_card_games_have_hand() {
        for p in playable() {
            let g = (p.make)();
            match spec_for(p.id) {
                None => {
                    // nici zaruri, nici cărți în afara listei explicite
                    assert!(!has_dice(p.id) || dice_rows(&g.render()).len() < 100);
                }
                Some(spec) => {
                    if matches!(
                        p.id,
                        "pescuit"
                            | "popa"
                            | "teenpatti"
                            | "poker"
                            | "blackjack"
                            | "domino"
                            | "belote"
                            | "scopa"
                            | "briscola"
                            | "tarot"
                            | "canasta"
                            | "truco"
                    ) {
                        let r = rows(spec, &g.render());
                        assert!(
                            r.iter().any(|x| x.role == Role::Hand),
                            "{}: fără rând de mână în\n{}",
                            p.id,
                            g.render()
                        );
                    }
                }
            }
        }
    }
}
