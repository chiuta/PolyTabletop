//! Standard Algebraic Notation (SAN) for the PolyTabletop chess game.
//!
//! Works purely through the object-safe [`GameObj`] interface: the board is read
//! from `render()`, the legal moves (coordinate text such as `e2e4`, `e7e8q`)
//! from `legal_moves()`, and check / checkmate from the position after the move.
//! SAN is therefore always derived from the game's own legal moves.
//! Rules of the notation: public FIDE Laws of Chess, Appendix C.

use polytabletop_core::{GameObj, Outcome};

/// Board read from `render()`: index = rank * 8 + file (a1 = 0); ' ' = empty;
/// uppercase = white, lowercase = black.
pub fn board_from_render(g: &dyn GameObj) -> [char; 64] {
    let mut b = [' '; 64];
    for line in g.render().lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.len() != 10 {
            continue;
        }
        let Ok(r) = toks[0].parse::<usize>() else {
            continue;
        };
        if !(1..=8).contains(&r) {
            continue;
        }
        for f in 0..8 {
            let c = toks[1 + f].chars().next().unwrap_or('.');
            b[(r - 1) * 8 + f] = if c == '.' { ' ' } else { c };
        }
    }
    b
}

fn parse_sq(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    if b.len() < 2 || !(b'a'..=b'h').contains(&b[0]) || !(b'1'..=b'8').contains(&b[1]) {
        return None;
    }
    Some((b[1] - b'1') as usize * 8 + (b[0] - b'a') as usize)
}

fn sq_name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 8) as u8) as char, i / 8 + 1)
}

/// (from, to, promotion letter) of a coordinate move text.
fn split(m: &str) -> Option<(usize, usize, Option<char>)> {
    if m.len() < 4 || !m.is_ascii() {
        return None;
    }
    Some((
        parse_sq(&m[0..2])?,
        parse_sq(&m[2..4])?,
        m[4..].chars().next(),
    ))
}

/// SAN of the legal move `mv` (coordinate text) in position `g`.
pub fn san_of(g: &dyn GameObj, mv: &str) -> Option<String> {
    let legal = g.legal_moves();
    if !legal.iter().any(|m| m == mv) {
        return None;
    }
    let b = board_from_render(g);
    let (from, to, promo) = split(mv)?;
    let piece = b[from].to_ascii_uppercase();
    let capture = b[to] != ' ' || (piece == 'P' && from % 8 != to % 8);
    let mut s = String::new();
    if piece == 'K' && from % 8 != to % 8 && from.abs_diff(to) == 2 {
        s.push_str(if to % 8 == 6 { "O-O" } else { "O-O-O" });
    } else if piece == 'P' {
        if capture {
            s.push((b'a' + (from % 8) as u8) as char);
            s.push('x');
        }
        s.push_str(&sq_name(to));
        if let Some(p) = promo {
            s.push('=');
            s.push(p.to_ascii_uppercase());
        }
    } else {
        s.push(piece);
        let others: Vec<usize> = legal
            .iter()
            .filter(|m| m.as_str() != mv)
            .filter_map(|m| split(m))
            .filter(|&(f2, t2, _)| t2 == to && f2 != from && b[f2] == b[from])
            .map(|(f2, _, _)| f2)
            .collect();
        if !others.is_empty() {
            if others.iter().all(|&o| o % 8 != from % 8) {
                s.push((b'a' + (from % 8) as u8) as char);
            } else if others.iter().all(|&o| o / 8 != from / 8) {
                s.push((b'1' + (from / 8) as u8) as char);
            } else {
                s.push_str(&sq_name(from));
            }
        }
        if capture {
            s.push('x');
        }
        s.push_str(&sq_name(to));
    }
    let mut after = g.clone_box();
    after.play(mv).ok()?;
    match after.outcome() {
        Some(Outcome::Win(_)) => s.push('#'),
        _ if after.render().contains("ȘAH") => s.push('+'),
        _ => {}
    }
    Some(s)
}

fn norm_san(s: &str) -> String {
    let mut t: String = s
        .trim()
        .replace('0', "O")
        .chars()
        .filter(|c| !"x+#!?=:-".contains(*c))
        .collect();
    // "e8q" -> "e8Q"
    if let Some(l) = t.chars().last() {
        let prev_digit = t.chars().rev().nth(1).is_some_and(|c| c.is_ascii_digit());
        if prev_digit && "qrbn".contains(l) && t.len() >= 3 {
            t.pop();
            t.push(l.to_ascii_uppercase());
        }
    }
    t
}

/// Coordinate text of the legal move that the SAN string denotes.
pub fn move_from_san(g: &dyn GameObj, san: &str) -> Option<String> {
    let legal = g.legal_moves();
    let want = norm_san(san);
    if want.is_empty() {
        return None;
    }
    if legal.iter().any(|m| m.eq_ignore_ascii_case(san.trim())) && split(san.trim()).is_some() {
        return legal
            .iter()
            .find(|m| m.eq_ignore_ascii_case(san.trim()))
            .cloned();
    }
    for m in &legal {
        if let Some(s) = san_of(g, m) {
            if norm_san(&s) == want {
                return Some(m.clone());
            }
        }
    }
    // Over-specified disambiguation (e.g. "Ngf3" when "Nf3" suffices): match by hints.
    let chars: Vec<char> = want.chars().collect();
    if chars.len() >= 4 && "NBRQK".contains(chars[0]) {
        let dest = format!("{}{}", chars[chars.len() - 2], chars[chars.len() - 1]);
        let to = parse_sq(&dest)?;
        let hint = &chars[1..chars.len() - 2];
        let b = board_from_render(g);
        let cands: Vec<&String> = legal
            .iter()
            .filter(|m| {
                split(m).is_some_and(|(f, t, _)| {
                    t == to
                        && b[f].to_ascii_uppercase() == chars[0]
                        && hint.iter().all(|&h| {
                            if h.is_ascii_digit() {
                                (b'1' + (f / 8) as u8) as char == h
                            } else {
                                (b'a' + (f % 8) as u8) as char == h
                            }
                        })
                })
            })
            .collect();
        if cands.len() == 1 {
            return Some(cands[0].clone());
        }
    }
    None
}

/// Standard PGN tokenizer: returns (tags, SAN tokens, result token).
/// Comments `{}` / `;`, variations `()`, NAGs `$n`, move numbers and `!?` marks
/// are dropped.
pub struct ParsedPgn {
    pub tags: Vec<(String, String)>,
    pub moves: Vec<String>,
    pub result: Option<String>,
}

pub fn parse_pgn(text: &str) -> ParsedPgn {
    let mut tags = Vec::new();
    let mut body = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') && t.contains('"') {
            let inner = &t[1..t.len() - 1];
            if let Some((k, v)) = inner.split_once(' ') {
                tags.push((
                    k.to_string(),
                    v.trim().trim_matches('"').replace("\\\"", "\""),
                ));
            }
        } else if t.starts_with('%') {
        } else {
            let line = match line.find(';') {
                Some(i) => &line[..i],
                None => line,
            };
            body.push_str(line);
            body.push('\n');
        }
    }
    let mut cleaned = String::new();
    let (mut brace, mut depth) = (false, 0usize);
    for c in body.chars() {
        if brace {
            if c == '}' {
                brace = false;
            }
            continue;
        }
        match c {
            '{' => brace = true,
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth > 0 => {}
            _ => cleaned.push(c),
        }
    }
    let mut moves = Vec::new();
    let mut result = None;
    for tok in cleaned.split_whitespace() {
        if matches!(tok, "1-0" | "0-1" | "1/2-1/2" | "*") {
            result = Some(tok.to_string());
            continue;
        }
        if tok.starts_with('$') {
            continue;
        }
        if tok.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        // strip a leading move number such as "12." or "12..."
        let t = tok.trim_start_matches(|c: char| c.is_ascii_digit());
        let t = if t.len() != tok.len() && t.starts_with('.') {
            t.trim_start_matches('.')
        } else {
            tok
        };
        if t.is_empty() {
            continue;
        }
        moves.push(t.to_string());
    }
    ParsedPgn {
        tags,
        moves,
        result,
    }
}
