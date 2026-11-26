//! Parsarea textului din `render()` în structuri de tablă + geometrie (hit-test).

use eframe::egui::{Pos2, Rect, Vec2};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Chess,
    Draughts,
    Reversi,
    Connect4,
    Gomoku,
    Go,
    Morris,
    Xiangqi,
    Makruk,
    Janggi,
    Dara,
    Morabaraba,
    /// familia Mancala (gropi + depozite): mancala, sungka, ayo, pallanguzhi
    Pits,
    Shogi,
    Backgammon,
    /// cursă pe pistă (Nu te supăra, frate)
    Ludo,
    /// cursă pe pistă (Pachisi cu cauri)
    Pachisi,
    /// Hex 11×11 (romb: rândurile sunt deplasate cu ½ celulă)
    Hex,
    /// Amazoane 10×10 (mutare + săgeată = trei click-uri)
    Amazons,
    /// Linii de acțiune 8×8
    Loa,
    Konane,
    /// Fanorona 9×5 cu diagonale
    Fanorona,
    /// Capre și tigri 5×5 cu diagonale
    Tigers,
    /// Surakarta 6×6 cu circuitele de arcuri din colțuri
    Surakarta,
    Hasami,
    /// puncte și căsuțe: puncte, laturi apăsabile, căsuțe colorate după proprietar
    Puncte,
    /// 8×8, pioni care avansează (urcă / coboară)
    Strapungere,
    /// 7×7, muți ca regina, apoi scoți un pătrat
    Izolare,
    /// 6×6 în patru cadrane, cu butoane de rotire
    Rotire,
    /// 9×9 cu ziduri: pătrate + sloturi de zid între ele
    Coridor,
    /// 5×5 cu stive de pietre
    Drum,
    /// tablă hexagonală cu latura 5
    Tripierd,
    /// grămezi de bețe
    Nim,
}

pub fn kind_of(id: &str) -> Option<Kind> {
    Some(match id {
        "chess" => Kind::Chess,
        "draughts" => Kind::Draughts,
        "reversi" => Kind::Reversi,
        "connect4" => Kind::Connect4,
        "gomoku" => Kind::Gomoku,
        "go" => Kind::Go,
        "morris" => Kind::Morris,
        "xiangqi" => Kind::Xiangqi,
        "makruk" => Kind::Makruk,
        "janggi" => Kind::Janggi,
        "dara" => Kind::Dara,
        "morabaraba" => Kind::Morabaraba,
        "mancala" | "sungka" | "ayo" | "pallanguzhi" => Kind::Pits,
        "shogi" => Kind::Shogi,
        "backgammon" => Kind::Backgammon,
        "ludo" => Kind::Ludo,
        "pachisi" => Kind::Pachisi,
        "hex" => Kind::Hex,
        "amazoane" => Kind::Amazons,
        "linii" => Kind::Loa,
        "konane" => Kind::Konane,
        "fanorona" => Kind::Fanorona,
        "tigri" => Kind::Tigers,
        "surakarta" => Kind::Surakarta,
        "hasami" => Kind::Hasami,
        "puncte" => Kind::Puncte,
        "strapungere" => Kind::Strapungere,
        "izolare" => Kind::Izolare,
        "rotire" => Kind::Rotire,
        "coridor" => Kind::Coridor,
        "drum" => Kind::Drum,
        "tripierd" => Kind::Tripierd,
        "nim" => Kind::Nim,
        _ => return None,
    })
}

#[derive(Clone, Debug)]
pub struct Cell {
    /// numele folosit în mutări (`e2`, `d7`, sau coloana `4` la Patru în linie)
    pub name: String,
    pub col: usize,
    /// 0 = sus
    pub row: usize,
    /// caracterul din render (piesă, `.`, `+`, ` `)
    pub ch: char,
    /// număr de semințe (doar familia Mancala)
    pub num: i32,
}

/// Date suplimentare pentru tablele care nu sunt grile simple.
#[derive(Clone, Debug, PartialEq)]
pub enum Extra {
    None,
    Bg {
        bar: [u8; 2],
        off: [u8; 2],
        dice: Vec<u8>,
    },
    Race {
        ring: usize,
        home: usize,
        goal: u32,
        /// pătratele cu `sq % safe == 0` sunt castele (0 = niciunul)
        safe: usize,
        roll: Option<u32>,
    },
    /// Coridor: zidurile puse (colțul stânga-jos al fiecărui zid, `(x, y)` de la 0) și cele rămase
    Walls {
        h: Vec<(usize, usize)>,
        v: Vec<(usize, usize)>,
        left: [u8; 2],
    },
    /// Drum: conținutul fiecărei celule (de jos în sus), în ordinea `cells`
    Stacks(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct Board {
    pub kind: Kind,
    pub cols: usize,
    pub rows: usize,
    pub cells: Vec<Cell>,
    /// legături între puncte (doar Moara), indici în `cells`
    pub links: Vec<(usize, usize)>,
    /// doar Mancala: rândul gropilor jucătorului la mutare (celelalte nu se pot apăsa)
    pub active_row: Option<usize>,
    pub extra: Extra,
}

impl Board {
    pub fn find(&self, name: &str) -> Option<usize> {
        self.cells.iter().position(|c| c.name == name)
    }
}

fn header_cols(lines: &[Vec<char>]) -> Option<(usize, Vec<(char, usize)>)> {
    for (li, l) in lines.iter().enumerate() {
        let toks: Vec<(char, usize)> = l
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
            .map(|(i, c)| (*c, i))
            .collect();
        if toks.len() >= 5 && toks.iter().all(|(c, _)| c.is_ascii_lowercase()) && toks[0].0 == 'a' {
            return Some((li, toks));
        }
    }
    None
}

fn parse_grid(kind: Kind, render: &str) -> Option<Board> {
    let lines: Vec<Vec<char>> = render.lines().map(|l| l.chars().collect()).collect();
    let (_hl, cols) = header_cols(&lines)?;
    let mut cells = Vec::new();
    let mut row = 0;
    for l in lines.iter() {
        let s: String = l.iter().collect();
        let first = s.split_whitespace().next().unwrap_or("");
        let Ok(label) = first.parse::<u32>() else {
            continue;
        };
        for (ci, (letter, idx)) in cols.iter().enumerate() {
            let ch = l.get(*idx).copied().unwrap_or(' ');
            cells.push(Cell {
                name: format!("{letter}{label}"),
                col: ci,
                row,
                ch,
                num: 0,
            });
        }
        row += 1;
    }
    if row == 0 {
        return None;
    }
    Some(Board {
        kind,
        cols: cols.len(),
        rows: row,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

fn parse_connect4(render: &str) -> Option<Board> {
    let mut cells = Vec::new();
    let mut row = 0;
    let mut ncols = 0;
    for l in render.lines() {
        let t = l.trim();
        if !t.starts_with('|') || !t.ends_with('|') || t.len() < 3 {
            continue;
        }
        let inner = &t[1..t.len() - 1];
        let toks: Vec<char> = inner
            .split_whitespace()
            .filter_map(|x| x.chars().next())
            .collect();
        ncols = ncols.max(toks.len());
        for (ci, ch) in toks.iter().enumerate() {
            cells.push(Cell {
                name: (ci + 1).to_string(),
                col: ci,
                row,
                ch: *ch,
                num: 0,
            });
        }
        row += 1;
    }
    if row == 0 || ncols == 0 {
        return None;
    }
    Some(Board {
        kind: Kind::Connect4,
        cols: ncols,
        rows: row,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

fn parse_morris(kind: Kind, render: &str) -> Option<Board> {
    let lines: Vec<Vec<char>> = render.lines().map(|l| l.chars().collect()).collect();
    let at = |y: usize, x: usize| -> char {
        lines
            .get(1 + y)
            .and_then(|l| l.get(2 + x))
            .copied()
            .unwrap_or(' ')
    };
    let mut cells = Vec::new();
    for y in (0..=12).step_by(2) {
        for f in 0..7 {
            let ch = at(y, 6 * f);
            if matches!(ch, 'X' | 'O' | '+') {
                let rank = 7 - y / 2;
                cells.push(Cell {
                    name: format!("{}{}", (b'a' + f as u8) as char, rank),
                    col: f,
                    row: y / 2,
                    ch,
                    num: 0,
                });
            }
        }
    }
    if cells.len() != 24 {
        return None;
    }
    let mut links = Vec::new();
    for i in 0..cells.len() {
        for j in 0..cells.len() {
            if i == j {
                continue;
            }
            let (a, b) = (&cells[i], &cells[j]);
            // doar vecinul imediat spre dreapta / în jos
            if a.row == b.row && b.col > a.col {
                let between = cells
                    .iter()
                    .any(|c| c.row == a.row && c.col > a.col && c.col < b.col);
                let ok = (6 * a.col + 1..6 * b.col).all(|x| at(a.row * 2, x) == '-');
                if !between && ok {
                    links.push((i, j));
                }
            } else if a.col == b.col && b.row > a.row {
                let between = cells
                    .iter()
                    .any(|c| c.col == a.col && c.row > a.row && c.row < b.row);
                let ok = (2 * a.row + 1..2 * b.row).all(|y| at(y, 6 * a.col) == '|');
                if !between && ok {
                    links.push((i, j));
                }
            }
        }
    }
    if kind == Kind::Morabaraba {
        // diagonale: caracterul `\` sau `/` la mijlocul segmentului dintre colțuri
        for i in 0..cells.len() {
            for j in 0..cells.len() {
                let (a, b) = (&cells[i], &cells[j]);
                if b.row == a.row + 1 && (b.col == a.col + 1 || a.col == b.col + 1) {
                    let want = if b.col > a.col { '\\' } else { '/' };
                    if at(a.row + b.row, 3 * (a.col + b.col)) == want {
                        links.push((i, j));
                    }
                }
            }
        }
    }
    Some(Board {
        kind,
        cols: 7,
        rows: 7,
        cells,
        links,
        active_row: None,
        extra: Extra::None,
    })
}

fn pit_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|t| t.trim_matches(|c| c == '[' || c == ']').parse::<i32>().ok())
        .collect()
}

/// Mancala și rudele: rândul 0 = jucătorul 2 (sus), rândul 1 = jucătorul 1.
/// Coloana 0 = depozitul din stânga, ultima = depozitul din dreapta.
fn parse_pits(render: &str) -> Option<Board> {
    let lines: Vec<&str> = render.lines().collect();
    let h = lines
        .iter()
        .position(|l| l.trim_start().starts_with("Jucător 2"))?;
    let top = pit_nums(lines.get(h + 1)?);
    let bot = pit_nums(lines.get(h + 2)?);
    if top.len() < 3 || top.len() != bot.len() {
        return None;
    }
    let n = top.len() - 1;
    let mut cells = Vec::new();
    // jos primul: la numele duplicate `find` întoarce gropile jucătorului 1
    for (j, &num) in bot.iter().enumerate().take(n) {
        cells.push(Cell {
            name: (j + 1).to_string(),
            col: j + 1,
            row: 1,
            ch: 'o',
            num,
        });
    }
    for j in 0..n {
        cells.push(Cell {
            name: (n - j).to_string(),
            col: j + 1,
            row: 0,
            ch: 'o',
            num: top[j + 1],
        });
    }
    cells.push(Cell {
        name: "depozit2".into(),
        col: 0,
        row: 0,
        ch: '[',
        num: top[0],
    });
    cells.push(Cell {
        name: "depozit1".into(),
        col: n + 1,
        row: 1,
        ch: '[',
        num: bot[n],
    });
    Some(Board {
        kind: Kind::Pits,
        cols: n + 2,
        rows: 2,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

// ---------------------------------------------------------------- Shogi

/// Ordinea pieselor în mână, de la stânga la dreapta (ca în `render`).
pub const SHOGI_HAND: [char; 7] = ['R', 'B', 'G', 'S', 'N', 'L', 'P'];

fn hand_counts(line: &str) -> [i32; 7] {
    let mut out = [0; 7];
    let after = line.split("mână:").nth(1).unwrap_or("");
    for t in after.split_whitespace() {
        let mut ch = t.chars();
        let Some(p) = ch.next() else { continue };
        let n: String = ch.collect();
        if let Some(k) = SHOGI_HAND.iter().position(|&h| h == p) {
            out[k] = if n.is_empty() {
                1
            } else {
                n.parse().unwrap_or(1)
            };
        }
    }
    out
}

/// Tablă 9×9 în rândurile 1..=9; mâna lui gote în rândul 0, a lui sente în rândul 10.
/// Numele pătratelor: `7g`; numele pieselor din mână: `S:P*` / `G:P*` (prefixul
/// dispare pentru cel la mutare, vezi [`Board::with_mover`]). `num` = 1 pentru piesă promovată.
fn parse_shogi(render: &str) -> Option<Board> {
    let lines: Vec<Vec<char>> = render.lines().map(|l| l.chars().collect()).collect();
    let gote = render.lines().find(|l| l.starts_with("gote"))?;
    let sente = render.lines().find(|l| l.starts_with("sente"))?;
    let hi = lines.iter().position(|l| {
        let t: String = l.iter().collect();
        t.split_whitespace().collect::<Vec<_>>() == ["9", "8", "7", "6", "5", "4", "3", "2", "1"]
    })?;
    let mut cells = Vec::new();
    for r in 0..9usize {
        let l = lines.get(hi + 1 + r)?;
        for c in 0..9usize {
            let chunk: Vec<char> = (0..3)
                .map(|k| l.get(2 + 3 * c + k).copied().unwrap_or(' '))
                .collect();
            let promoted = chunk[0] == '+';
            let ch = chunk
                .iter()
                .copied()
                .find(|x| x.is_ascii_alphabetic() || *x == '.')
                .unwrap_or('.');
            cells.push(Cell {
                name: format!("{}{}", 9 - c, (b'a' + r as u8) as char),
                col: c,
                row: r + 1,
                ch,
                num: promoted as i32,
            });
        }
    }
    for (who, line, row) in [("G", gote, 0usize), ("S", sente, 10)] {
        let n = hand_counts(line);
        for (k, &p) in SHOGI_HAND.iter().enumerate() {
            let ch = if who == "S" {
                p
            } else {
                p.to_ascii_lowercase()
            };
            cells.push(Cell {
                name: format!("{who}:{p}*"),
                col: k + 1,
                row,
                ch,
                num: n[k],
            });
        }
    }
    Some(Board {
        kind: Kind::Shogi,
        cols: 9,
        rows: 11,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

// ------------------------------------------------------------ Backgammon

pub const BG_COLS: usize = 14;
pub const BG_ROWS: usize = 8;
pub const BG_BAR_COL: usize = 6;
pub const BG_OFF_COL: usize = 13;

fn bg_point_col(k: usize) -> usize {
    if k < 6 {
        k
    } else {
        k + 1
    }
}

fn parse_backgammon(render: &str) -> Option<Board> {
    let lines: Vec<&str> = render.lines().collect();
    let nums = |l: &str| -> Vec<u32> {
        l.replace('|', " ")
            .split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect()
    };
    let hi = lines
        .iter()
        .position(|l| nums(l) == (13..=24).collect::<Vec<u32>>())?;
    let mut cells = Vec::new();
    for (li, top) in [(hi + 1, true), (hi + 2, false)] {
        let l = lines.get(li)?.replace('|', " ");
        let toks: Vec<&str> = l.split_whitespace().collect();
        if toks.len() != 12 {
            return None;
        }
        for (k, t) in toks.iter().enumerate() {
            let (ch, num) = if *t == "." {
                ('.', 0)
            } else {
                let sym = t.chars().last()?;
                let n: i32 = t[..t.len() - 1].parse().ok()?;
                (sym, n)
            };
            let n = if top { 13 + k } else { 12 - k };
            cells.push(Cell {
                name: n.to_string(),
                col: bg_point_col(k),
                row: if top { 2 } else { 5 },
                ch,
                num,
            });
        }
    }
    let kv = |key: &str, line: &str| -> Option<u8> {
        let i = line.find(key)? + key.len();
        let d: String = line[i..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        d.parse().ok()
    };
    let st = lines.iter().find(|l| l.starts_with("Bară"))?;
    let (sc_i, _) = st.char_indices().find(|(_, c)| *c == 'S')?;
    let (bar_s, off_s) = st.split_at(sc_i);
    let bar = [kv("X=", bar_s)?, kv("O=", bar_s)?];
    let off = [kv("X=", off_s)?, kv("O=", off_s)?];
    let dice = lines
        .iter()
        .find(|l| l.contains("Rămase"))
        .map(|l| {
            let tail = l.split("Rămase").nth(1).unwrap_or("");
            tail.split(|c: char| !c.is_ascii_digit())
                .filter_map(|t| t.parse::<u8>().ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    cells.push(Cell {
        name: "bar".into(),
        col: BG_BAR_COL,
        row: 3,
        ch: 'b',
        num: (bar[0] + bar[1]) as i32,
    });
    cells.push(Cell {
        name: "off".into(),
        col: BG_OFF_COL,
        row: 3,
        ch: 'f',
        num: (off[0] + off[1]) as i32,
    });
    Some(Board {
        kind: Kind::Backgammon,
        cols: BG_COLS,
        rows: BG_ROWS,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::Bg { bar, off, dice },
    })
}

// ------------------------------------------------------------- Cursă (Ludo/Pachisi)

/// Latura pătratului (pista e perimetrul unui pătrat `w × w`, `4(w-1)` pătrate).
pub fn race_w(ring: usize) -> usize {
    ring / 4 + 1
}

/// Poziția (col, rând) a pătratului absolut `sq` al pistei.
pub fn ring_xy(ring: usize, sq: usize) -> (usize, usize) {
    let w = race_w(ring);
    let pos = (sq + w / 2) % ring;
    let e = w - 1;
    if pos < e {
        (pos, 0)
    } else if pos < 2 * e {
        (e, pos - e)
    } else if pos < 3 * e {
        (e - (pos - 2 * e), e)
    } else {
        (0, e - (pos - 3 * e))
    }
}

/// Rotație de 180° (poziția celui de-al doilea jucător).
fn rot(w: usize, (c, r): (usize, usize)) -> (usize, usize) {
    (w - 1 - c, w - 1 - r)
}

/// Poziția pe tablă a pieselor jucătorului `pl` (0/1) cu progresul `q`;
/// `slot` = a câta piesă din bază (pentru q = 0).
pub fn race_pos(
    ring: usize,
    home: usize,
    goal: u32,
    pl: usize,
    q: u32,
    slot: usize,
) -> (usize, usize) {
    let w = race_w(ring);
    let lane = w / 2 - 1;
    let track_max = goal as usize - home - 1;
    let p1 = if q == 0 {
        (1 + slot % 2, 2 + slot / 2)
    } else if (q as usize) <= track_max {
        let start = if pl == 0 { 0 } else { ring / 2 };
        let sq = (start + q as usize - 1) % ring;
        return ring_xy(ring, sq);
    } else if q < goal {
        (lane, q as usize - track_max)
    } else {
        (lane, home + 1)
    };
    if pl == 0 {
        p1
    } else {
        rot(w, p1)
    }
}

fn digits_in(s: &str) -> Vec<u32> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter_map(|t| t.parse().ok())
        .collect()
}

fn num_after(line: &str, key: &str) -> Option<u32> {
    let i = line.find(key)? + key.len();
    let d: String = line[i..]
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    d.parse().ok()
}

/// Nume piese: `1:12` / `2:12` (prefixul dispare pentru cel la mutare); `1:0` = în bază.
fn parse_race(kind: Kind, render: &str) -> Option<Board> {
    let (ring, home, goal, safe) = if kind == Kind::Ludo {
        (40usize, 3usize, 44u32, 0usize)
    } else {
        (48, 6, 54, 6)
    };
    let w = race_w(ring);
    let mut cells = Vec::new();
    let mut seen = 0;
    for l in render.lines() {
        if !l.contains("bază") || !(l.contains("(X)") || l.contains("(O)")) {
            continue;
        }
        let pl = if l.contains("(X)") { 0usize } else { 1 };
        let ch = if pl == 0 { 'X' } else { 'O' };
        let base = num_after(l, "bază")?;
        let done = num_after(l, "terminate")
            .or_else(|| num_after(l, "acasă"))
            .unwrap_or(0);
        let inner = l
            .split('[')
            .nth(1)
            .and_then(|t| t.split(']').next())
            .unwrap_or("");
        let mut qs: Vec<u32> = vec![0; base as usize];
        qs.extend(digits_in(inner).into_iter().filter(|&q| q > 0 && q < goal));
        qs.extend(std::iter::repeat_n(goal, done as usize));
        let mut slot = 0;
        for q in qs {
            let (c, r) = race_pos(ring, home, goal, pl, q, slot);
            if q == 0 {
                slot += 1;
            }
            cells.push(Cell {
                name: format!("{}:{q}", pl + 1),
                col: c,
                row: r,
                ch,
                num: q as i32,
            });
        }
        seen += 1;
    }
    if seen != 2 {
        return None;
    }
    let roll_line = render
        .lines()
        .find(|l| l.contains("Zar:") || l.contains("a aruncat"));
    let roll = roll_line.and_then(|l| {
        num_after(
            l,
            if l.contains("Zar:") {
                "Zar:"
            } else {
                "a aruncat"
            },
        )
    });
    Some(Board {
        kind,
        cols: w,
        rows: w,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::Race {
            ring,
            home,
            goal,
            safe,
            roll,
        },
    })
}

/// Hex: rândurile (cifre) au `r` spații de indentare; fiecare rând are 11 celule.
fn parse_hex(render: &str) -> Option<Board> {
    let mut cells = Vec::new();
    let mut row = 0;
    for l in render.lines() {
        let mut it = l.split_whitespace();
        let Some(Ok(label)) = it.next().map(|t| t.parse::<u32>()) else {
            continue;
        };
        let toks: Vec<char> = it.filter_map(|t| t.chars().next()).collect();
        if toks.len() != 11 {
            continue;
        }
        for (ci, ch) in toks.into_iter().enumerate() {
            cells.push(Cell {
                name: format!("{}{}", (b'a' + ci as u8) as char, label),
                col: ci,
                row,
                ch,
                num: 0,
            });
        }
        row += 1;
    }
    if row != 11 {
        return None;
    }
    Some(Board {
        kind: Kind::Hex,
        cols: 11,
        rows: 11,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

// ------------------------------------------------- Puncte, Rotire, Coridor, Drum, Trei-pierde, Nim

pub const PUNCTE_D: usize = 6;
/// Raportul dintre grosimea unui „gol” (punct / loc de zid) și o celulă.
pub const GAP: f32 = 0.38;

fn letter(i: usize) -> char {
    (b'a' + i as u8) as char
}

/// Puncte și căsuțe: grilă dublată 11×11 (coloane impare = căsuțe / laturi orizontale).
/// Laturile au numele mutărilor (`oa1`, `va1`); punctele `~d..`, căsuțele `~b..`.
fn parse_puncte(render: &str) -> Option<Board> {
    let lines: Vec<Vec<char>> = render.lines().map(|l| l.chars().collect()).collect();
    let d = PUNCTE_D;
    let mut cells = Vec::new();
    let mut seen = 0;
    for (li, l) in lines.iter().enumerate() {
        let s: String = l.iter().collect();
        let first = s.split_whitespace().next().unwrap_or("");
        let Ok(label) = first.parse::<usize>() else {
            continue;
        };
        if !(1..=d).contains(&label) {
            continue;
        }
        seen += 1;
        let rt = d - label;
        for c in 0..d {
            cells.push(Cell {
                name: format!("~d{c}{rt}"),
                col: 2 * c,
                row: 2 * rt,
                ch: '+',
                num: 0,
            });
            if c < d - 1 {
                let drawn = l.get(5 + 4 * c) == Some(&'-');
                cells.push(Cell {
                    name: format!("o{}{}", letter(c), label),
                    col: 2 * c + 1,
                    row: 2 * rt,
                    ch: if drawn { '-' } else { ' ' },
                    num: 0,
                });
            }
        }
        if label > 1 {
            let nl = lines.get(li + 1)?;
            for c in 0..d {
                let drawn = nl.get(4 + 4 * c) == Some(&'|');
                cells.push(Cell {
                    name: format!("v{}{}", letter(c), label - 1),
                    col: 2 * c,
                    row: 2 * rt + 1,
                    ch: if drawn { '|' } else { ' ' },
                    num: 0,
                });
                if c < d - 1 {
                    let o = match nl.get(6 + 4 * c) {
                        Some('X') => 'X',
                        Some('O') => 'O',
                        _ => ' ',
                    };
                    cells.push(Cell {
                        name: format!("~b{c}{rt}"),
                        col: 2 * c + 1,
                        row: 2 * rt + 1,
                        ch: o,
                        num: 0,
                    });
                }
            }
        }
    }
    if seen != d {
        return None;
    }
    Some(Board {
        kind: Kind::Puncte,
        cols: 2 * d - 1,
        rows: 2 * d - 1,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

/// Rotire: 6×6 în rândurile/coloanele 1..=6 ale unei grile 8×8; marginea de sus/jos
/// are cele opt butoane `q<cadran><d|s>` (d = orar, s = antiorar).
fn parse_rotire(render: &str) -> Option<Board> {
    let mut cells = Vec::new();
    let mut seen = 0;
    for l in render.lines() {
        let mut it = l.split_whitespace();
        let Some(Ok(label)) = it.next().map(|t| t.parse::<usize>()) else {
            continue;
        };
        if !(1..=6).contains(&label) {
            continue;
        }
        let toks: Vec<char> = it
            .filter(|t| *t != "|")
            .filter_map(|t| t.chars().next())
            .collect();
        if toks.len() != 6 {
            return None;
        }
        for (ci, ch) in toks.iter().enumerate() {
            cells.push(Cell {
                name: format!("{}{}", letter(ci), label),
                col: 1 + ci,
                row: 1 + (6 - label),
                ch: *ch,
                num: 0,
            });
        }
        seen += 1;
    }
    if seen != 6 {
        return None;
    }
    for (q, (cx, top)) in [
        (1usize, (1usize, true)),
        (2, (4, true)),
        (3, (1, false)),
        (4, (4, false)),
    ] {
        for (k, dir) in ['s', 'd'].into_iter().enumerate() {
            cells.push(Cell {
                name: format!("q{q}{dir}"),
                col: cx + 2 * k,
                row: if top { 0 } else { 7 },
                ch: dir,
                num: q as i32,
            });
        }
    }
    Some(Board {
        kind: Kind::Rotire,
        cols: 8,
        rows: 8,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

/// Coridor: grilă dublată 17×17 (indici pari = pătrate, impari = goluri); la intersecția
/// dintre patru pătrate stă slotul de zid `w-d4` (apoi se alege `d4h` / `d4v`).
#[allow(clippy::needless_range_loop)]
fn parse_coridor(render: &str) -> Option<Board> {
    let lines: Vec<Vec<char>> = render.lines().map(|l| l.chars().collect()).collect();
    let mut cells = Vec::new();
    let mut show_v = [[false; 8]; 9]; // [y][x]
    let mut show_h = [[false; 9]; 8]; // [y][x] între rândurile y și y+1
    let mut rows_seen = 0;
    for (li, l) in lines.iter().enumerate() {
        let s: String = l.iter().collect();
        let first = s.split_whitespace().next().unwrap_or("");
        let Ok(label) = first.parse::<usize>() else {
            continue;
        };
        if !(1..=9).contains(&label) || l.len() < 18 {
            continue;
        }
        rows_seen += 1;
        let y = label - 1;
        for x in 0..9 {
            cells.push(Cell {
                name: format!("{}{}", letter(x), label),
                col: 2 * x,
                row: 2 * (8 - y),
                ch: l.get(2 + 2 * x).copied().unwrap_or('.'),
                num: 0,
            });
            if x < 8 {
                show_v[y][x] = l.get(3 + 2 * x) == Some(&'|');
            }
        }
        if y > 0 {
            if let Some(g) = lines.get(li + 1) {
                for x in 0..9 {
                    show_h[y - 1][x] = g.get(2 + 2 * x) == Some(&'-');
                }
            }
        }
    }
    if rows_seen != 9 {
        return None;
    }
    // zidurile se despart în perechi (nu se pot suprapune): de la stânga / de jos
    let mut h = Vec::new();
    let mut v = Vec::new();
    for (y, row) in show_h.iter().enumerate() {
        let mut x = 0;
        while x < 9 {
            if row[x] {
                h.push((x, y));
                x += 2;
            } else {
                x += 1;
            }
        }
    }
    for x in 0..8 {
        let mut y = 0;
        while y < 9 {
            if show_v[y][x] {
                v.push((x, y));
                y += 2;
            } else {
                y += 1;
            }
        }
    }
    for x in 0..8 {
        for y in 0..8 {
            cells.push(Cell {
                name: format!("w-{}{}", letter(x), y + 1),
                col: 2 * x + 1,
                row: 15 - 2 * y,
                ch: '+',
                num: 0,
            });
        }
    }
    let left = render
        .lines()
        .find(|l| l.starts_with("Ziduri"))
        .map(|l| {
            let d = digits_in(l);
            [
                d.first().copied().unwrap_or(0) as u8,
                d.get(1).copied().unwrap_or(0) as u8,
            ]
        })
        .unwrap_or([0, 0]);
    Some(Board {
        kind: Kind::Coridor,
        cols: 17,
        rows: 17,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::Walls { h, v, left },
    })
}

/// Drum: fiecare rând = etichetă + cinci stive (șiruri fără spații, de jos în sus).
fn parse_drum(render: &str) -> Option<Board> {
    let mut cells = Vec::new();
    let mut stacks = Vec::new();
    let mut seen = 0;
    for l in render.lines() {
        let mut it = l.split_whitespace();
        let Some(Ok(label)) = it.next().map(|t| t.parse::<usize>()) else {
            continue;
        };
        let toks: Vec<&str> = it.collect();
        if !(1..=5).contains(&label) || toks.len() != 5 {
            continue;
        }
        for (x, t) in toks.iter().enumerate() {
            let st = if *t == "." { "" } else { *t };
            cells.push(Cell {
                name: format!("{}{}", letter(x), label),
                col: x,
                row: 5 - label,
                ch: st.chars().last().unwrap_or('.'),
                num: st.chars().count() as i32,
            });
            stacks.push(st.to_string());
        }
        seen += 1;
    }
    if seen != 5 {
        return None;
    }
    Some(Board {
        kind: Kind::Drum,
        cols: 5,
        rows: 5,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::Stacks(stacks),
    })
}

/// Trei-pierde: rândurile `a`..`i` cu 5..9..5 celule; coloana = jumătăți de celulă
/// (`|r| + 2k`), vezi [`Mode::Hex`].
fn parse_tripierd(render: &str) -> Option<Board> {
    let mut cells = Vec::new();
    let mut seen = 0;
    for l in render.lines() {
        let mut it = l.split_whitespace();
        let Some(f) = it.next() else { continue };
        let mut fc = f.chars();
        let (Some(r), None) = (fc.next(), fc.next()) else {
            continue;
        };
        if !('a'..='i').contains(&r) {
            continue;
        }
        let toks: Vec<char> = it.filter_map(|t| t.chars().next()).collect();
        let i = (r as u8 - b'a') as usize;
        let want = 9 - i.abs_diff(4);
        if toks.len() != want || !toks.iter().all(|c| matches!(c, 'X' | 'O' | '.')) {
            continue;
        }
        for (k, ch) in toks.iter().enumerate() {
            cells.push(Cell {
                name: format!("{}{}", r, k + 1),
                col: i.abs_diff(4) + 2 * k,
                row: i,
                ch: *ch,
                num: 0,
            });
        }
        seen += 1;
    }
    if seen != 9 {
        return None;
    }
    Some(Board {
        kind: Kind::Tripierd,
        cols: 18,
        rows: 9,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

/// Nim: `a: | | | (3)`; băț `j` (de la stânga) al grămezii cu `h` bețe se numește
/// `<litera><h-j>` = mutarea care îl ia pe el și pe toate din dreapta lui.
fn parse_nim(render: &str) -> Option<Board> {
    let mut cells = Vec::new();
    let mut heaps = 0;
    let mut wide = 6;
    for l in render.lines() {
        let Some((lab, rest)) = l.split_once(':') else {
            continue;
        };
        let mut lc = lab.chars();
        let (Some(r), None) = (lc.next(), lc.next()) else {
            continue;
        };
        if !r.is_ascii_lowercase() {
            continue;
        }
        let h = rest.chars().filter(|c| *c == '|').count();
        wide = wide.max(h);
        for j in 0..h {
            cells.push(Cell {
                name: format!("{}{}", r, h - j),
                col: j,
                row: heaps,
                ch: '|',
                num: (h - j) as i32,
            });
        }
        heaps += 1;
    }
    if heaps == 0 {
        return None;
    }
    Some(Board {
        kind: Kind::Nim,
        cols: wide,
        rows: heaps,
        cells,
        links: vec![],
        active_row: None,
        extra: Extra::None,
    })
}

pub fn parse(kind: Kind, render: &str) -> Option<Board> {
    match kind {
        Kind::Hex => parse_hex(render),
        Kind::Connect4 => parse_connect4(render),
        Kind::Morris | Kind::Morabaraba => parse_morris(kind, render),
        Kind::Pits => parse_pits(render),
        Kind::Shogi => parse_shogi(render),
        Kind::Backgammon => parse_backgammon(render),
        Kind::Ludo | Kind::Pachisi => parse_race(kind, render),
        Kind::Puncte => parse_puncte(render),
        Kind::Rotire => parse_rotire(render),
        Kind::Coridor => parse_coridor(render),
        Kind::Drum => parse_drum(render),
        Kind::Tripierd => parse_tripierd(render),
        Kind::Nim => parse_nim(render),
        k => parse_grid(k, render),
    }
}

/// Celulele evidențiate pentru ultima mutare `mv`, făcută de `was`, când acum mută `now`.
/// Cursele (Ludo/Pachisi) nu au pătrate în text: nu evidențiem nimic.
pub fn last_cells(
    b: &Board,
    mv: &str,
    was: polytabletop_core::Player,
    now: polytabletop_core::Player,
) -> Vec<usize> {
    if matches!(b.kind, Kind::Ludo | Kind::Pachisi) {
        return vec![];
    }
    let (sq, _) = crate::compose::tokenize_k(Some(b.kind), mv);
    let mut out = Vec::new();
    for s in sq {
        let found = match b.kind {
            Kind::Pits => {
                let row = if was == polytabletop_core::Player::One {
                    1
                } else {
                    0
                };
                b.cells
                    .iter()
                    .position(|c| c.name == s && c.row == row && c.ch == 'o')
            }
            Kind::Connect4 => b
                .cells
                .iter()
                .enumerate()
                .filter(|(_, c)| c.name == s && matches!(c.ch, 'X' | 'O'))
                .min_by_key(|(_, c)| c.row)
                .map(|(i, _)| i),
            Kind::Backgammon => {
                let name = match s.parse::<u32>() {
                    Ok(n) if was != now => (25 - n).to_string(),
                    _ => s.clone(),
                };
                b.find(&name)
            }
            Kind::Shogi if s.contains('*') => None,
            _ => b.find(&s),
        };
        if let Some(i) = found {
            if !out.contains(&i) {
                out.push(i);
            }
        }
    }
    out
}

/// Dreptunghiul unei celule (pentru evidențieri): pătrat, sau jumătate de coloană la Table.
pub fn cell_rect(b: &Board, l: &Layout, c: &Cell) -> Rect {
    if b.kind == Kind::Backgammon {
        let x = l.origin.x + c.col as f32 * l.cell;
        let (y, h) = if c.col == BG_BAR_COL || c.col == BG_OFF_COL {
            (0.0, BG_ROWS as f32)
        } else if c.row < BG_ROWS / 2 {
            (0.0, (BG_ROWS / 2) as f32)
        } else {
            ((BG_ROWS / 2) as f32, (BG_ROWS / 2) as f32)
        };
        return Rect::from_min_size(
            Pos2::new(x, l.origin.y + y * l.cell),
            Vec2::new(l.cell, h * l.cell),
        );
    }
    l.rect(c)
}

/// Felul de geometrie al tablei.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// grilă (eventual forfecată / cu margine: vezi `shear`, `ry`, `pad`)
    Grid,
    /// grilă „cu goluri”: indicii de aceeași paritate sunt late (1 unitate), ceilalți
    /// înguști ([`GAP`]); `wide_odd` = indicii impari sunt cei lați (Puncte)
    Gap { wide_odd: bool },
    /// hexagoane: coloana e în jumătăți de celulă, rândurile sunt la 0,866 unități
    Hex,
}

pub const HEX_ROW: f32 = 0.866_025_4;
pub const HEX_H: f32 = 1.154_700_5;

/// Geometrie: centrul fiecărei celule = origine + (col+½, row+½)·celulă
/// (sau echivalentul pe tablele cu goluri / hexagonale, vezi [`Mode`]).
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub origin: Pos2,
    pub cell: f32,
    /// deplasare orizontală pe rând, în celule (Hex: ½)
    pub shear: f32,
    /// distanța dintre rânduri, în celule (Hex: √3/2)
    pub ry: f32,
    /// margine în jurul celulelor, în celule (Surakarta: arcurile din colțuri)
    pub pad: f32,
    pub mode: Mode,
}

/// (shear, ry, pad) după tip.
fn geometry(k: Kind) -> (f32, f32, f32) {
    match k {
        Kind::Hex => (0.5, 0.866_025_4, 0.0),
        Kind::Surakarta => (0.0, 1.0, 1.5),
        _ => (0.0, 1.0, 0.0),
    }
}

/// (început, lungime) în unități de celulă al indicelui `i` pe o axă cu goluri.
fn axis(wide_odd: bool, i: usize) -> (f32, f32) {
    let mut start = 0.0;
    for j in 0..i {
        start += if (j % 2 == 1) == wide_odd { 1.0 } else { GAP };
    }
    let len = if (i % 2 == 1) == wide_odd { 1.0 } else { GAP };
    (start, len)
}

fn mode_of(kind: Kind) -> Mode {
    match kind {
        Kind::Puncte => Mode::Gap { wide_odd: true },
        Kind::Coridor => Mode::Gap { wide_odd: false },
        Kind::Tripierd => Mode::Hex,
        _ => Mode::Grid,
    }
}

/// Dimensiunea tablei în unități de celulă.
fn units(b: &Board, mode: Mode) -> (f32, f32) {
    match mode {
        Mode::Grid => {
            let (shear, ry, pad) = geometry(b.kind);
            (
                b.cols as f32 + 2.0 * pad + (b.rows as f32 - 1.0) * shear,
                (b.rows as f32 - 1.0) * ry + 1.0 + 2.0 * pad,
            )
        }
        Mode::Gap { wide_odd } => {
            let t = |n: usize| {
                let (s, l) = axis(wide_odd, n - 1);
                s + l
            };
            (t(b.cols), t(b.rows))
        }
        Mode::Hex => (b.cols as f32 / 2.0, (b.rows - 1) as f32 * HEX_ROW + HEX_H),
    }
}

pub fn layout(b: &Board, area: Rect) -> Layout {
    let (shear, ry, pad) = geometry(b.kind);
    let mode = mode_of(b.kind);
    let (uw, uh) = units(b, mode);
    let cell = (area.width() / uw).min(area.height() / uh).max(1.0);
    let w = cell * uw;
    let h = cell * uh;
    Layout {
        origin: Pos2::new(area.center().x - w / 2.0, area.center().y - h / 2.0),
        cell,
        shear,
        ry,
        pad,
        mode,
    }
}

impl Layout {
    /// Dreptunghiul indicilor `(col, row)` (la hexagoane: pătratul din jurul centrului).
    pub fn rect_cr(&self, col: usize, row: usize) -> Rect {
        match self.mode {
            Mode::Grid => Rect::from_center_size(
                self.origin
                    + Vec2::new(
                        (col as f32 + 0.5 + self.pad + row as f32 * self.shear) * self.cell,
                        (row as f32 * self.ry + 0.5 + self.pad) * self.cell,
                    ),
                Vec2::splat(self.cell),
            ),
            Mode::Gap { wide_odd } => {
                let (sx, lx) = axis(wide_odd, col);
                let (sy, ly) = axis(wide_odd, row);
                Rect::from_min_size(
                    self.origin + Vec2::new(sx, sy) * self.cell,
                    Vec2::new(lx, ly) * self.cell,
                )
            }
            Mode::Hex => Rect::from_center_size(
                self.origin
                    + Vec2::new((col as f32 + 1.0) / 2.0, row as f32 * HEX_ROW + HEX_H / 2.0)
                        * self.cell,
                Vec2::splat(self.cell),
            ),
        }
    }

    pub fn rect(&self, c: &Cell) -> Rect {
        self.rect_cr(c.col, c.row)
    }

    pub fn center(&self, c: &Cell) -> Pos2 {
        self.rect(c).center()
    }
}

impl Board {
    /// La Mancala: doar gropile jucătorului la mutare se pot apăsa.
    pub fn with_mover(mut self, p: polytabletop_core::Player) -> Self {
        let one = p == polytabletop_core::Player::One;
        match self.kind {
            Kind::Pits => self.active_row = Some(if one { 1 } else { 0 }),
            Kind::Shogi => {
                let pre = if one { "S:" } else { "G:" };
                for c in &mut self.cells {
                    if let Some(rest) = c.name.strip_prefix(pre) {
                        c.name = rest.to_string();
                    }
                }
            }
            Kind::Ludo | Kind::Pachisi => {
                let goal = match self.extra {
                    Extra::Race { goal, .. } => goal as i32,
                    _ => i32::MAX,
                };
                let pre = if one { "1:" } else { "2:" };
                for c in &mut self.cells {
                    if c.num > 0 && c.num < goal {
                        if let Some(rest) = c.name.strip_prefix(pre) {
                            c.name = rest.to_string();
                        }
                    }
                }
                // piesele celui la mutare primele: câștigă la suprapuneri (hit-test)
                self.cells.sort_by_key(|c| c.name.contains(':'));
            }
            _ => {}
        }
        self
    }

    /// Nume în plus de evidențiat când mouse-ul stă peste celula `i` (laturi, sloturi,
    /// bețele care ar fi luate împreună cu cel de sub mouse).
    pub fn hover_names(&self, i: usize) -> Vec<String> {
        let c = &self.cells[i];
        match self.kind {
            Kind::Puncte | Kind::Coridor => vec![c.name.clone()],
            Kind::Nim => self
                .cells
                .iter()
                .filter(|o| o.row == c.row && o.col >= c.col)
                .map(|o| o.name.clone())
                .collect(),
            _ => vec![],
        }
    }

    pub fn clickable(&self, i: usize) -> bool {
        let c = &self.cells[i];
        match self.kind {
            Kind::Shogi => !c.name.contains(':') && !(c.name.ends_with('*') && c.num == 0),
            Kind::Ludo | Kind::Pachisi => !c.name.contains(':'),
            Kind::Puncte => matches!(c.name.as_bytes().first(), Some(b'o' | b'v')) && c.ch == ' ',
            Kind::Rotire => c.name.starts_with('q') || c.ch == '.',
            Kind::Strapungere
            | Kind::Izolare
            | Kind::Coridor
            | Kind::Drum
            | Kind::Tripierd
            | Kind::Nim => true,
            _ => match self.active_row {
                Some(r) => c.row == r && c.ch == 'o',
                None => true,
            },
        }
    }
}

/// Celula (indice) aflată sub `p`, dacă există.
pub fn hit(b: &Board, l: &Layout, p: Pos2) -> Option<usize> {
    if b.kind == Kind::Backgammon {
        let x = (p.x - l.origin.x) / l.cell;
        let y = (p.y - l.origin.y) / l.cell;
        if x < 0.0 || y < 0.0 || x >= BG_COLS as f32 || y >= BG_ROWS as f32 {
            return None;
        }
        let col = x as usize;
        let top = y < (BG_ROWS / 2) as f32;
        return b.cells.iter().position(|c| match col {
            BG_BAR_COL | BG_OFF_COL => c.col == col,
            _ => c.col == col && (c.row < BG_ROWS / 2) == top && c.name != "bar" && c.name != "off",
        });
    }
    let half = l.cell * 0.5;
    if b.kind == Kind::Hex || l.mode == Mode::Hex {
        // cea mai apropiată celulă, în raza cercului înscris în hexagon
        return b
            .cells
            .iter()
            .enumerate()
            .map(|(i, c)| (i, (l.center(c) - p).length()))
            .filter(|(_, d)| *d <= half)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i);
    }
    if let Mode::Gap { .. } = l.mode {
        // sloturile de zid (mici) au prioritate și o zonă mai mare decât golul
        let slot = |c: &Cell| c.name.starts_with("w-");
        let zone = |c: &Cell| {
            if slot(c) {
                Rect::from_center_size(l.center(c), Vec2::splat(l.cell * 0.7))
            } else {
                l.rect(c)
            }
        };
        return (0..b.cells.len())
            .find(|&i| slot(&b.cells[i]) && zone(&b.cells[i]).contains(p))
            .or_else(|| {
                (0..b.cells.len()).find(|&i| !slot(&b.cells[i]) && zone(&b.cells[i]).contains(p))
            });
    }
    b.cells
        .iter()
        .enumerate()
        .filter(|(_, c)| {
            let q = l.center(c);
            (p.x - q.x).abs() <= half && (p.y - q.y).abs() <= half
        })
        .map(|(i, _)| i)
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_games::playable;

    fn board_of(id: &str) -> Board {
        let p = playable().into_iter().find(|p| p.id == id).unwrap();
        let g = (p.make)();
        parse(kind_of(id).unwrap(), &g.render()).unwrap_or_else(|| panic!("parse {id}"))
    }

    #[test]
    fn sizes() {
        for (id, n) in [
            ("chess", 64),
            ("draughts", 64),
            ("reversi", 64),
            ("connect4", 42),
            ("gomoku", 225),
            ("go", 81),
            ("morris", 24),
            ("xiangqi", 90),
            ("makruk", 64),
            ("janggi", 90),
            ("dara", 30),
            ("morabaraba", 24),
            ("mancala", 14),
            ("sungka", 16),
            ("ayo", 14),
            ("pallanguzhi", 16),
            ("shogi", 95),
            ("backgammon", 26),
            ("ludo", 8),
            ("pachisi", 8),
            ("hex", 121),
            ("amazoane", 100),
            ("linii", 64),
            ("konane", 64),
            ("fanorona", 45),
            ("tigri", 25),
            ("surakarta", 36),
            ("hasami", 81),
            ("puncte", 121),
            ("strapungere", 64),
            ("izolare", 49),
            ("rotire", 44),
            ("coridor", 145),
            ("drum", 25),
            ("tripierd", 61),
            ("nim", 18),
        ] {
            assert_eq!(board_of(id).cells.len(), n, "{id}");
        }
    }

    #[test]
    fn chess_names_and_pieces() {
        let b = board_of("chess");
        let e1 = &b.cells[b.find("e1").unwrap()];
        assert_eq!(e1.ch, 'K');
        assert_eq!(b.cells[b.find("e8").unwrap()].ch, 'k');
        assert_eq!(b.cells[b.find("a8").unwrap()].row, 0);
    }

    #[test]
    fn go_skips_i() {
        let b = board_of("go");
        assert!(b.find("j9").is_some());
        assert!(b.find("i9").is_none());
    }

    #[test]
    fn xiangqi_ranks() {
        let b = board_of("xiangqi");
        assert_eq!(b.cells[b.find("e0").unwrap()].ch, 'K');
        assert_eq!(b.cells[b.find("e9").unwrap()].ch, 'k');
        assert!(b.find("a0").is_some());
    }

    #[test]
    fn morris_links() {
        let b = board_of("morris");
        assert_eq!(b.links.len(), 32);
        assert!(b.find("d4").is_none());
    }

    #[test]
    fn morabaraba_links_and_pits() {
        let b = board_of("morabaraba");
        assert_eq!(b.links.len(), 40);
        let m = board_of("mancala");
        assert_eq!(m.cells[m.find("depozit1").unwrap()].num, 0);
        assert_eq!(m.cells[m.find("3").unwrap()].num, 4);
        let j = board_of("janggi");
        assert_eq!(j.cells[j.find("e1").unwrap()].ch, 'G');
        assert!(j.find("a0").is_some());
    }

    #[test]
    fn hit_roundtrip() {
        for id in [
            "chess",
            "go",
            "morris",
            "connect4",
            "janggi",
            "mancala",
            "shogi",
            "backgammon",
            "hex",
            "amazoane",
            "fanorona",
            "tigri",
            "surakarta",
            "hasami",
            "linii",
            "konane",
            "puncte",
            "rotire",
            "coridor",
            "tripierd",
            "nim",
        ] {
            let b = board_of(id);
            let l = layout(
                &b,
                Rect::from_min_size(Pos2::new(10.0, 20.0), Vec2::new(640.0, 500.0)),
            );
            for (i, c) in b.cells.iter().enumerate() {
                assert_eq!(hit(&b, &l, l.center(c)), Some(i), "{id} {}", c.name);
            }
        }
    }

    #[test]
    fn new_boards_names_and_geometry() {
        let h = board_of("hex");
        assert_eq!(h.cells[h.find("k11").unwrap()].row, 10);
        assert_eq!(h.cells[h.find("a1").unwrap()].col, 0);
        let l = layout(
            &h,
            Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(800.0, 700.0)),
        );
        // rândul următor e deplasat cu ½ celulă spre dreapta
        let a1 = l.center(&h.cells[h.find("a1").unwrap()]);
        let a2 = l.center(&h.cells[h.find("a2").unwrap()]);
        assert!((a2.x - a1.x - l.cell * 0.5).abs() < 0.01);
        let a = board_of("amazoane");
        assert_eq!(a.cells[a.find("a4").unwrap()].ch, 'X');
        assert!(a.find("j10").is_some());
        let f = board_of("fanorona");
        assert_eq!((f.cols, f.rows), (9, 5));
        assert_eq!(f.cells[f.find("e3").unwrap()].ch, '+');
        let t = board_of("tigri");
        assert_eq!(t.cells.iter().filter(|c| c.ch == 'T').count(), 4);
        let s = board_of("surakarta");
        assert_eq!(s.cells.iter().filter(|c| c.ch == 'X').count(), 12);
        let ha = board_of("hasami");
        assert_eq!(ha.cells.iter().filter(|c| c.ch == 'O').count(), 9);
    }

    #[test]
    fn shogi_names_hands_and_mover() {
        let b = board_of("shogi").with_mover(polytabletop_core::Player::One);
        assert_eq!(b.cells[b.find("5i").unwrap()].ch, 'K');
        assert_eq!(b.cells[b.find("1a").unwrap()].ch, 'l');
        assert_eq!(b.cells[b.find("2h").unwrap()].ch, 'R');
        // cel la mutare (sente) are numele fără prefix; gote păstrează `G:`
        assert!(b.find("P*").is_some());
        assert!(b.find("G:P*").is_some());
        assert!(!b.clickable(b.find("P*").unwrap()));
    }

    #[test]
    fn shogi_hand_parse() {
        assert_eq!(hand_counts("sente în mână: R B2 P3"), [1, 2, 0, 0, 0, 0, 3]);
        assert_eq!(hand_counts("gote în mână: -"), [0; 7]);
    }

    #[test]
    fn backgammon_layout() {
        let b = board_of("backgammon");
        let n = |s: &str| b.cells[b.find(s).unwrap()].clone();
        assert_eq!(n("24").num, 2);
        assert!(matches!(n("24").ch, 'X' | 'O'));
        assert_eq!(n("24").row, 2);
        assert_eq!(n("6").row, 5);
        assert_eq!(n("13").col, 0);
        assert_eq!(n("12").col, 0);
        assert_eq!(n("19").col, 7);
        assert_eq!(n("1").col, 12);
    }

    #[test]
    fn ring_is_a_closed_perimeter() {
        for ring in [40usize, 48] {
            let w = race_w(ring);
            let mut seen = std::collections::BTreeSet::new();
            for sq in 0..ring {
                let (c, r) = ring_xy(ring, sq);
                assert!(c == 0 || r == 0 || c == w - 1 || r == w - 1);
                assert!(seen.insert((c, r)));
                let (c2, r2) = ring_xy(ring, (sq + 1) % ring);
                assert_eq!(c.abs_diff(c2) + r.abs_diff(r2), 1, "{ring} {sq}");
            }
            assert_eq!(seen.len(), ring);
        }
    }

    #[test]
    fn race_lanes_do_not_collide() {
        for (ring, home, goal) in [(40usize, 3usize, 44u32), (48, 6, 54)] {
            let mut all = std::collections::BTreeSet::new();
            for sq in 0..ring {
                all.insert(ring_xy(ring, sq));
            }
            for pl in 0..2 {
                let tm = goal - home as u32 - 1;
                for q in tm + 1..=goal {
                    assert!(
                        all.insert(race_pos(ring, home, goal, pl, q, 0)),
                        "{ring} {pl} {q}"
                    );
                }
                for slot in 0..4 {
                    assert!(all.insert(race_pos(ring, home, goal, pl, 0, slot)));
                }
            }
        }
    }

    #[test]
    fn race_mover_names() {
        let p = playable().into_iter().find(|p| p.id == "ludo").unwrap();
        let g = (p.make)();
        let b = parse(Kind::Ludo, &g.render())
            .unwrap()
            .with_mover(g.to_move());
        // toate în bază: nume cu prefix, nimic apăsabil
        assert!((0..b.cells.len()).all(|i| !b.clickable(i)));
    }
}
