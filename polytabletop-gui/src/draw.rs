//! Desenarea tablelor cu `egui::Painter`.

use crate::board::{self, Board, Cell, Extra, Kind, Layout};
use eframe::egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, Vec2};
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
pub struct Theme {
    pub dark: bool,
}

impl Theme {
    pub fn text(&self) -> Color32 {
        if self.dark {
            Color32::from_gray(225)
        } else {
            Color32::from_gray(30)
        }
    }
    fn sq_light(&self) -> Color32 {
        if self.dark {
            Color32::from_rgb(150, 140, 120)
        } else {
            Color32::from_rgb(240, 217, 181)
        }
    }
    fn sq_dark(&self) -> Color32 {
        if self.dark {
            Color32::from_rgb(85, 100, 80)
        } else {
            Color32::from_rgb(181, 136, 99)
        }
    }
    fn wood(&self) -> Color32 {
        if self.dark {
            Color32::from_rgb(120, 92, 55)
        } else {
            Color32::from_rgb(222, 184, 120)
        }
    }
    fn line(&self) -> Color32 {
        if self.dark {
            Color32::from_gray(25)
        } else {
            Color32::from_gray(50)
        }
    }
}

const SEL: Color32 = Color32::from_rgb(250, 210, 40);
const NEXT: Color32 = Color32::from_rgb(60, 190, 90);
const LAST: Color32 = Color32::from_rgb(235, 120, 30);

fn disc(p: &Painter, c: Pos2, r: f32, fill: Color32, edge: Color32) {
    p.circle_filled(c, r, fill);
    p.circle_stroke(c, r, Stroke::new((r * 0.08).max(1.0), edge));
}

fn label(p: &Painter, c: Pos2, s: &str, size: f32, col: Color32) {
    p.text(c, Align2::CENTER_CENTER, s, FontId::proportional(size), col);
}

/// Marcaje: ultima mutare (contur portocaliu), selecția (galben), destinațiile
/// legale (puncte verzi) și, la plimbarea mouse-ului peste o piesă, destinațiile ei
/// (puncte verzi estompate).
fn marks(
    p: &Painter,
    b: &Board,
    l: &Layout,
    sel: &[String],
    next: &BTreeSet<String>,
    hover: &BTreeSet<String>,
    last: &[usize],
) {
    let r = l.cell * 0.44;
    let bg = b.kind == Kind::Backgammon;
    for &i in last {
        if let Some(c) = b.cells.get(i) {
            let rc = board::cell_rect(b, l, c).shrink(l.cell * 0.04);
            p.rect_stroke(
                rc,
                l.cell * 0.08,
                Stroke::new((l.cell * 0.07).max(2.0), LAST),
                StrokeKind::Inside,
            );
        }
    }
    for (i, c) in b.cells.iter().enumerate() {
        if !b.clickable(i) {
            continue;
        }
        // aceste table își desenează singure selecția, destinațiile și hover-ul
        if match b.kind {
            Kind::Puncte | Kind::Nim => true,
            Kind::Rotire => c.name.starts_with('q'),
            Kind::Coridor => c.name.starts_with("w-"),
            _ => false,
        } {
            continue;
        }
        let pos = l.center(c);
        let empty = matches!(c.ch, '.' | '+' | ' ');
        let is_sel = sel.contains(&c.name);
        let is_next = next.contains(&c.name);
        let is_hover = !is_next && hover.contains(&c.name);
        if bg && (is_sel || is_next || is_hover) {
            let col = if is_sel {
                SEL
            } else if is_next {
                NEXT
            } else {
                NEXT.gamma_multiply(0.55)
            };
            let rc = board::cell_rect(b, l, c).shrink(l.cell * 0.06);
            p.rect_stroke(
                rc,
                l.cell * 0.1,
                Stroke::new(l.cell * 0.08, col),
                StrokeKind::Inside,
            );
            continue;
        }
        if is_sel {
            p.circle_stroke(pos, r, Stroke::new(l.cell * 0.09, SEL));
        } else if is_next || is_hover {
            let col = if is_next {
                NEXT
            } else {
                NEXT.gamma_multiply(0.55)
            };
            if empty {
                p.circle_filled(pos, l.cell * 0.13, col);
            } else {
                p.circle_stroke(pos, r, Stroke::new(l.cell * 0.07, col));
            }
        }
    }
}

fn square_bg(p: &Painter, l: &Layout, c: &Cell, col: Color32) {
    let tl = l.origin + Vec2::new(c.col as f32 * l.cell, c.row as f32 * l.cell);
    p.rect_filled(Rect::from_min_size(tl, Vec2::splat(l.cell)), 0.0, col);
}

fn coords(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    // etichete de fișier/rang pe margini (nume din celulele din colț)
    let col = th.text().gamma_multiply(0.7);
    let sz = (l.cell * 0.22).clamp(8.0, 16.0);
    for c in &b.cells {
        if c.row == b.rows - 1 {
            let pos = l.center(c) + Vec2::new(0.0, l.cell * 0.5 + sz * 0.7);
            let t: String = c.name.chars().filter(|x| x.is_alphabetic()).collect();
            label(p, pos, &t, sz, col);
        }
        if c.col == 0 {
            let pos = l.center(c) - Vec2::new(l.cell * 0.5 + sz * 0.8, 0.0);
            let t: String = c.name.chars().filter(|x| x.is_ascii_digit()).collect();
            label(p, pos, &t, sz, col);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw(
    p: &Painter,
    b: &Board,
    l: &Layout,
    th: &Theme,
    sel: &[String],
    next: &BTreeSet<String>,
    hover: &BTreeSet<String>,
    last: &[usize],
) {
    match b.kind {
        Kind::Chess | Kind::Makruk => draw_chess(p, b, l, th),
        Kind::Draughts => draw_draughts(p, b, l, th),
        Kind::Reversi => draw_reversi(p, b, l, th),
        Kind::Connect4 => draw_c4(p, b, l, th),
        Kind::Gomoku | Kind::Go => draw_stones(p, b, l, th),
        Kind::Morris | Kind::Morabaraba => draw_morris(p, b, l, th),
        Kind::Xiangqi | Kind::Janggi => draw_xiangqi(p, b, l, th),
        Kind::Dara => draw_dara(p, b, l, th),
        Kind::Pits => draw_pits(p, b, l, th),
        Kind::Shogi => draw_shogi(p, b, l, th),
        Kind::Backgammon => draw_backgammon(p, b, l, th),
        Kind::Ludo | Kind::Pachisi => draw_race(p, b, l, th),
        Kind::Hex => draw_hex(p, b, l, th),
        Kind::Amazons => draw_amazons(p, b, l, th),
        Kind::Loa => draw_loa(p, b, l, th),
        Kind::Konane => draw_konane(p, b, l, th),
        Kind::Fanorona | Kind::Tigers => draw_lines_diag(p, b, l, th),
        Kind::Surakarta => draw_surakarta(p, b, l, th),
        Kind::Hasami => draw_hasami(p, b, l, th),
        Kind::Puncte => draw_puncte(p, b, l, th, hover),
        Kind::Strapungere => draw_strap(p, b, l, th),
        Kind::Izolare => draw_izolare(p, b, l, th),
        Kind::Rotire => draw_rotire(p, b, l, th, sel, next),
        Kind::Coridor => draw_coridor(p, b, l, th, sel, next, hover),
        Kind::Drum => draw_drum(p, b, l, th),
        Kind::Tripierd => draw_tripierd(p, b, l, th),
        Kind::Nim => draw_nim(p, b, l, th, hover),
    }
    if !matches!(
        b.kind,
        Kind::Connect4
            | Kind::Morris
            | Kind::Morabaraba
            | Kind::Pits
            | Kind::Shogi
            | Kind::Backgammon
            | Kind::Ludo
            | Kind::Pachisi
            | Kind::Hex
            | Kind::Puncte
            | Kind::Rotire
            | Kind::Tripierd
            | Kind::Nim
    ) {
        coords(p, b, l, th);
    }
    marks(p, b, l, sel, next, hover, last);
}

fn draw_chess(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    for c in &b.cells {
        square_bg(
            p,
            l,
            c,
            if (c.col + c.row) % 2 == 0 {
                th.sq_light()
            } else {
                th.sq_dark()
            },
        );
    }
    for c in &b.cells {
        if c.ch.is_ascii_alphabetic() {
            let white = c.ch.is_ascii_uppercase();
            let pos = l.center(c);
            let (fill, edge, tx) = if white {
                (
                    Color32::from_gray(250),
                    Color32::from_gray(40),
                    Color32::from_gray(20),
                )
            } else {
                (
                    Color32::from_gray(35),
                    Color32::from_gray(230),
                    Color32::from_gray(245),
                )
            };
            disc(p, pos, l.cell * 0.38, fill, edge);
            label(
                p,
                pos,
                &c.ch.to_ascii_uppercase().to_string(),
                l.cell * 0.42,
                tx,
            );
        }
    }
}

fn draw_draughts(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    for c in &b.cells {
        let dark = c.ch != ' ';
        square_bg(p, l, c, if dark { th.sq_dark() } else { th.sq_light() });
    }
    for c in &b.cells {
        let low = c.ch.to_ascii_lowercase();
        if low == 'x' || low == 'o' {
            let pos = l.center(c);
            let (fill, edge) = if low == 'x' {
                (Color32::from_rgb(245, 240, 225), Color32::from_gray(60))
            } else {
                (Color32::from_rgb(150, 30, 30), Color32::from_gray(20))
            };
            disc(p, pos, l.cell * 0.38, fill, edge);
            p.circle_stroke(
                pos,
                l.cell * 0.26,
                Stroke::new(1.0, edge.gamma_multiply(0.6)),
            );
            if c.ch.is_ascii_uppercase() {
                label(
                    p,
                    pos,
                    "R",
                    l.cell * 0.34,
                    if low == 'x' {
                        Color32::from_gray(30)
                    } else {
                        Color32::from_gray(245)
                    },
                );
            }
        }
    }
}

fn draw_reversi(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let green = if th.dark {
        Color32::from_rgb(25, 90, 50)
    } else {
        Color32::from_rgb(40, 140, 75)
    };
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(r, 0.0, green);
    for i in 0..=b.cols {
        let x = l.origin.x + i as f32 * l.cell;
        p.line_segment(
            [Pos2::new(x, r.top()), Pos2::new(x, r.bottom())],
            Stroke::new(1.0, th.line()),
        );
    }
    for j in 0..=b.rows {
        let y = l.origin.y + j as f32 * l.cell;
        p.line_segment(
            [Pos2::new(r.left(), y), Pos2::new(r.right(), y)],
            Stroke::new(1.0, th.line()),
        );
    }
    for c in &b.cells {
        match c.ch {
            'X' => disc(
                p,
                l.center(c),
                l.cell * 0.4,
                Color32::from_gray(20),
                Color32::from_gray(90),
            ),
            'O' => disc(
                p,
                l.center(c),
                l.cell * 0.4,
                Color32::from_gray(245),
                Color32::from_gray(120),
            ),
            _ => {}
        }
    }
}

fn draw_c4(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(
        r,
        l.cell * 0.15,
        if th.dark {
            Color32::from_rgb(25, 55, 130)
        } else {
            Color32::from_rgb(40, 90, 190)
        },
    );
    for c in &b.cells {
        let (fill, edge) = match c.ch {
            'X' => (
                Color32::from_rgb(230, 190, 20),
                Color32::from_rgb(160, 120, 0),
            ),
            'O' => (
                Color32::from_rgb(215, 40, 40),
                Color32::from_rgb(130, 15, 15),
            ),
            _ => (
                if th.dark {
                    Color32::from_gray(15)
                } else {
                    Color32::from_gray(235)
                },
                Color32::from_gray(60),
            ),
        };
        disc(p, l.center(c), l.cell * 0.4, fill, edge);
    }
    for c in b.cells.iter().filter(|c| c.row == b.rows - 1) {
        label(
            p,
            l.center(c) + Vec2::new(0.0, l.cell * 0.75),
            &c.name,
            (l.cell * 0.3).clamp(9.0, 18.0),
            th.text(),
        );
    }
}

fn stone(p: &Painter, pos: Pos2, ch: char, r: f32) {
    match ch {
        'X' => disc(p, pos, r, Color32::from_gray(15), Color32::from_gray(90)),
        'O' => disc(p, pos, r, Color32::from_gray(248), Color32::from_gray(110)),
        _ => {}
    }
}

fn draw_stones(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(r, 4.0, th.wood());
    let st = Stroke::new(1.0, th.line());
    let half = l.cell * 0.5;
    for i in 0..b.cols {
        let x = l.origin.x + (i as f32 + 0.5) * l.cell;
        p.line_segment(
            [
                Pos2::new(x, r.top() + half),
                Pos2::new(x, r.bottom() - half),
            ],
            st,
        );
    }
    for j in 0..b.rows {
        let y = l.origin.y + (j as f32 + 0.5) * l.cell;
        p.line_segment(
            [
                Pos2::new(r.left() + half, y),
                Pos2::new(r.right() - half, y),
            ],
            st,
        );
    }
    for c in &b.cells {
        let pos = l.center(c);
        if c.ch == '+' {
            p.circle_filled(pos, l.cell * 0.09, th.line());
        }
        stone(p, pos, c.ch, l.cell * 0.44);
    }
    if b.kind == Kind::Gomoku {
        // punctul central
        if let Some(i) = b.find("h8") {
            let c = &b.cells[i];
            if c.ch == '.' {
                p.circle_filled(l.center(c), l.cell * 0.09, th.line());
            }
        }
    }
}

fn draw_morris(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(r, 6.0, th.wood());
    let st = Stroke::new(2.0, th.line());
    for &(i, j) in &b.links {
        p.line_segment([l.center(&b.cells[i]), l.center(&b.cells[j])], st);
    }
    for c in &b.cells {
        let pos = l.center(c);
        p.circle_filled(pos, l.cell * 0.1, th.line());
        stone(p, pos, c.ch, l.cell * 0.36);
    }
}

fn draw_xiangqi(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(r, 4.0, th.wood());
    let st = Stroke::new(1.0, th.line());
    let x = |i: usize| l.origin.x + (i as f32 + 0.5) * l.cell;
    let y = |j: usize| l.origin.y + (j as f32 + 0.5) * l.cell;
    for j in 0..b.rows {
        p.line_segment([Pos2::new(x(0), y(j)), Pos2::new(x(b.cols - 1), y(j))], st);
    }
    let janggi = b.kind == Kind::Janggi;
    for i in 0..b.cols {
        if janggi || i == 0 || i == b.cols - 1 {
            p.line_segment([Pos2::new(x(i), y(0)), Pos2::new(x(i), y(b.rows - 1))], st);
        } else {
            p.line_segment([Pos2::new(x(i), y(0)), Pos2::new(x(i), y(4))], st);
            p.line_segment([Pos2::new(x(i), y(5)), Pos2::new(x(i), y(b.rows - 1))], st);
        }
    }
    // palate
    let (c0, c2) = (3, 5);
    for (r0, r1) in [(0usize, 2usize), (b.rows - 3, b.rows - 1)] {
        p.line_segment([Pos2::new(x(c0), y(r0)), Pos2::new(x(c2), y(r1))], st);
        p.line_segment([Pos2::new(x(c2), y(r0)), Pos2::new(x(c0), y(r1))], st);
    }
    if !janggi {
        label(
            p,
            Pos2::new(x(b.cols / 2), (y(4) + y(5)) / 2.0),
            "~  R Â U L  ~",
            (l.cell * 0.34).clamp(9.0, 20.0),
            th.line(),
        );
    }
    for c in &b.cells {
        if c.ch.is_ascii_alphabetic() {
            let red = c.ch.is_ascii_uppercase();
            let pos = l.center(c);
            let (fill, edge, tx) = if red && janggi {
                // Cho (majuscule, începe): albastru
                (
                    Color32::from_rgb(225, 235, 250),
                    Color32::from_rgb(30, 70, 190),
                    Color32::from_rgb(30, 70, 190),
                )
            } else if red {
                (
                    Color32::from_rgb(250, 235, 205),
                    Color32::from_rgb(190, 30, 30),
                    Color32::from_rgb(190, 30, 30),
                )
            } else if janggi {
                (
                    Color32::from_rgb(250, 225, 220),
                    Color32::from_rgb(190, 30, 30),
                    Color32::from_rgb(170, 25, 25),
                )
            } else {
                (
                    Color32::from_rgb(235, 225, 200),
                    Color32::from_gray(20),
                    Color32::from_gray(15),
                )
            };
            disc(p, pos, l.cell * 0.4, fill, edge);
            label(
                p,
                pos,
                &c.ch.to_ascii_uppercase().to_string(),
                l.cell * 0.42,
                tx,
            );
        }
    }
}

fn draw_dara(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(r, 4.0, th.wood());
    for i in 0..=b.cols {
        let x = l.origin.x + i as f32 * l.cell;
        p.line_segment(
            [Pos2::new(x, r.top()), Pos2::new(x, r.bottom())],
            Stroke::new(1.0, th.line()),
        );
    }
    for j in 0..=b.rows {
        let y = l.origin.y + j as f32 * l.cell;
        p.line_segment(
            [Pos2::new(r.left(), y), Pos2::new(r.right(), y)],
            Stroke::new(1.0, th.line()),
        );
    }
    for c in &b.cells {
        match c.ch {
            'X' => disc(
                p,
                l.center(c),
                l.cell * 0.36,
                Color32::from_rgb(40, 40, 120),
                Color32::from_gray(200),
            ),
            'O' => disc(
                p,
                l.center(c),
                l.cell * 0.36,
                Color32::from_rgb(235, 150, 40),
                Color32::from_gray(40),
            ),
            _ => {}
        }
    }
}

fn draw_pits(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    );
    p.rect_filled(r, l.cell * 0.2, th.wood());
    let hole = if th.dark {
        Color32::from_rgb(55, 40, 25)
    } else {
        Color32::from_rgb(120, 80, 45)
    };
    for (i, c) in b.cells.iter().enumerate() {
        let pos = l.center(c);
        let n = c.num.to_string();
        if c.ch == '[' {
            // depozit: o capsulă înaltă cât ambele rânduri
            let half = Vec2::new(l.cell * 0.4, l.cell * 0.92);
            let ctr = Pos2::new(pos.x, l.origin.y + l.cell);
            p.rect_filled(Rect::from_center_size(ctr, half * 2.0), l.cell * 0.4, hole);
            label(p, ctr, &n, l.cell * 0.4, Color32::from_gray(245));
        } else {
            p.circle_filled(pos, l.cell * 0.4, hole);
            if b.clickable(i) {
                p.circle_stroke(pos, l.cell * 0.4, Stroke::new(2.0, th.line()));
            }
            // semințe (până la 12 puncte) + numărul
            let k = c.num.clamp(0, 12) as usize;
            for s in 0..k {
                let ang = s as f32 * 2.399;
                let rad = l.cell * 0.05 * (s as f32 + 1.0).sqrt() * 2.6;
                let q = pos + Vec2::new(ang.cos(), ang.sin()) * rad;
                disc(
                    p,
                    q,
                    l.cell * 0.05,
                    Color32::from_rgb(235, 225, 200),
                    Color32::from_gray(90),
                );
            }
            label(
                p,
                pos + Vec2::new(0.0, l.cell * 0.3),
                &n,
                l.cell * 0.22,
                Color32::from_gray(245),
            );
        }
    }
}

// ------------------------------------------------------------------ Shogi

fn shogi_piece(p: &Painter, pos: Pos2, cell: f32, gote: bool, letter: char, promoted: bool) {
    let sgn = if gote { -1.0 } else { 1.0 };
    let pts: Vec<Pos2> = [
        (0.0, -0.44),
        (0.31, -0.2),
        (0.38, 0.42),
        (-0.38, 0.42),
        (-0.31, -0.2),
    ]
    .iter()
    .map(|(x, y)| pos + Vec2::new(x * cell * 0.92, y * sgn * cell * 0.92))
    .collect();
    p.add(Shape::convex_polygon(
        pts,
        Color32::from_rgb(244, 220, 160),
        Stroke::new(1.2, Color32::from_gray(40)),
    ));
    let col = if promoted {
        Color32::from_rgb(190, 30, 30)
    } else {
        Color32::from_gray(20)
    };
    let t = if promoted {
        format!("+{letter}")
    } else {
        letter.to_string()
    };
    let at = pos + Vec2::new(0.0, 0.06 * sgn * cell);
    // piesa lui gote este întoarsă: litera rămâne lizibilă
    label(p, at, &t, cell * if promoted { 0.34 } else { 0.42 }, col);
}

fn draw_shogi(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = Rect::from_min_size(
        l.origin + Vec2::new(0.0, l.cell),
        Vec2::new(l.cell * 9.0, l.cell * 9.0),
    );
    p.rect_filled(r, 3.0, th.wood());
    let st = Stroke::new(1.0, th.line());
    for i in 0..=9 {
        let x = r.left() + i as f32 * l.cell;
        let y = r.top() + i as f32 * l.cell;
        p.line_segment([Pos2::new(x, r.top()), Pos2::new(x, r.bottom())], st);
        p.line_segment([Pos2::new(r.left(), y), Pos2::new(r.right(), y)], st);
    }
    for (cx, cy) in [(3, 3), (6, 3), (3, 6), (6, 6)] {
        p.circle_filled(
            Pos2::new(r.left() + cx as f32 * l.cell, r.top() + cy as f32 * l.cell),
            l.cell * 0.07,
            th.line(),
        );
    }
    let sz = (l.cell * 0.2).clamp(8.0, 14.0);
    let tc = th.text().gamma_multiply(0.75);
    for c in 0..9usize {
        label(
            p,
            Pos2::new(r.left() + (c as f32 + 0.5) * l.cell, r.top() - sz * 0.7),
            &(9 - c).to_string(),
            sz,
            tc,
        );
    }
    for rr in 0..9usize {
        label(
            p,
            Pos2::new(r.right() + sz * 0.9, r.top() + (rr as f32 + 0.5) * l.cell),
            &((b'a' + rr as u8) as char).to_string(),
            sz,
            tc,
        );
    }
    for c in &b.cells {
        let is_hand = c.row == 0 || c.row == 10;
        if c.ch == '.' {
            continue;
        }
        let gote = c.ch.is_ascii_lowercase();
        let up = c.ch.to_ascii_uppercase();
        let pos = l.center(c);
        if is_hand {
            if c.num == 0 {
                p.circle_stroke(
                    pos,
                    l.cell * 0.3,
                    Stroke::new(1.0, th.text().gamma_multiply(0.25)),
                );
                continue;
            }
            shogi_piece(p, pos, l.cell, gote, up, false);
            if c.num > 1 {
                label(
                    p,
                    pos + Vec2::new(l.cell * 0.36, l.cell * 0.3),
                    &format!("×{}", c.num),
                    l.cell * 0.28,
                    th.text(),
                );
            }
        } else {
            shogi_piece(p, pos, l.cell, gote, up, c.num == 1);
        }
    }
}

// ------------------------------------------------------------- Table

fn bg_pip_layout(n: u8) -> &'static [(f32, f32)] {
    match n {
        1 => &[(0.0, 0.0)],
        2 => &[(-1.0, -1.0), (1.0, 1.0)],
        3 => &[(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
        4 => &[(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)],
        5 => &[
            (-1.0, -1.0),
            (1.0, -1.0),
            (0.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
        _ => &[
            (-1.0, -1.0),
            (1.0, -1.0),
            (-1.0, 0.0),
            (1.0, 0.0),
            (-1.0, 1.0),
            (1.0, 1.0),
        ],
    }
}

fn draw_die(p: &Painter, c: Pos2, size: f32, n: u8) {
    let rc = Rect::from_center_size(c, Vec2::splat(size));
    p.rect_filled(rc, size * 0.18, Color32::from_gray(248));
    p.rect_stroke(
        rc,
        size * 0.18,
        Stroke::new(1.5, Color32::from_gray(40)),
        StrokeKind::Inside,
    );
    for (dx, dy) in bg_pip_layout(n) {
        p.circle_filled(
            c + Vec2::new(dx * size * 0.24, dy * size * 0.24),
            size * 0.075,
            Color32::from_gray(20),
        );
    }
}

fn bg_checker(p: &Painter, pos: Pos2, r: f32, ch: char) {
    if ch == 'X' {
        disc(
            p,
            pos,
            r,
            Color32::from_rgb(245, 238, 215),
            Color32::from_gray(70),
        );
    } else {
        disc(
            p,
            pos,
            r,
            Color32::from_rgb(45, 40, 42),
            Color32::from_gray(190),
        );
    }
}

fn draw_backgammon(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let cs = l.cell;
    let at = |col: f32, row: f32| l.origin + Vec2::new(col * cs, row * cs);
    let full = Rect::from_min_size(
        l.origin,
        Vec2::new(cs * board::BG_COLS as f32, cs * board::BG_ROWS as f32),
    );
    p.rect_filled(full, 6.0, th.wood());
    let felt = if th.dark {
        Color32::from_rgb(40, 70, 50)
    } else {
        Color32::from_rgb(70, 120, 85)
    };
    for (c0, c1) in [(0.0, 6.0), (7.0, 13.0)] {
        p.rect_filled(Rect::from_min_max(at(c0, 0.0), at(c1, 8.0)), 2.0, felt);
    }
    p.rect_filled(
        Rect::from_min_max(at(6.0, 0.0), at(7.0, 8.0)),
        0.0,
        th.wood().gamma_multiply(0.7),
    );
    p.rect_filled(
        Rect::from_min_max(at(13.0, 0.0), at(14.0, 8.0)),
        0.0,
        th.wood().gamma_multiply(0.7),
    );
    let tri_a = Color32::from_rgb(205, 170, 110);
    let tri_b = Color32::from_rgb(140, 60, 50);
    let h = 3.8;
    let nlabel = (cs * 0.24).clamp(8.0, 14.0);
    for c in &b.cells {
        if c.name == "bar" || c.name == "off" {
            continue;
        }
        let top = c.row < board::BG_ROWS / 2;
        let k = if c.col < 6 { c.col } else { c.col - 1 };
        let col = if k % 2 == 0 { tri_a } else { tri_b };
        let x0 = c.col as f32;
        let (base_y, tip_y) = if top { (0.0, h) } else { (8.0, 8.0 - h) };
        p.add(Shape::convex_polygon(
            vec![
                at(x0 + 0.04, base_y),
                at(x0 + 0.96, base_y),
                at(x0 + 0.5, tip_y),
            ],
            col,
            Stroke::NONE,
        ));
        // numărul punctului în afara tablei
        let ly = if top { -0.28 } else { 8.28 };
        label(
            p,
            at(x0 + 0.5, ly),
            &c.name,
            nlabel,
            th.text().gamma_multiply(0.8),
        );
        // piese
        if c.num > 0 {
            let shown = c.num.min(4) as usize;
            for s in 0..shown {
                let y = if top {
                    0.5 + s as f32 * 0.88
                } else {
                    7.5 - s as f32 * 0.88
                };
                bg_checker(p, at(x0 + 0.5, y), cs * 0.42, c.ch);
            }
            if c.num > 4 {
                let y = if top {
                    0.5 + 3.0 * 0.88
                } else {
                    7.5 - 3.0 * 0.88
                };
                label(
                    p,
                    at(x0 + 0.5, y),
                    &c.num.to_string(),
                    cs * 0.4,
                    if c.ch == 'X' {
                        Color32::from_gray(20)
                    } else {
                        Color32::from_gray(245)
                    },
                );
            }
        }
    }
    if let Extra::Bg { bar, off, dice } = &b.extra {
        // bara: O sus, X jos (de la centru în afară)
        for (pl, ch, top) in [(1usize, 'O', true), (0usize, 'X', false)] {
            let n = bar[pl] as usize;
            for s in 0..n.min(3) {
                let y = if top { 3.5 - s as f32 } else { 4.5 + s as f32 };
                bg_checker(p, at(6.5, y), cs * 0.4, ch);
            }
            if n > 3 {
                let y = if top { 0.4 } else { 7.6 };
                label(p, at(6.5, y), &format!("×{n}"), cs * 0.3, th.text());
            }
        }
        // tava de scoatere: X jos, O sus
        for (pl, ch, top) in [(1usize, 'O', true), (0usize, 'X', false)] {
            let n = off[pl];
            let y = if top { 1.0 } else { 7.0 };
            bg_checker(p, at(13.5, y), cs * 0.38, ch);
            label(
                p,
                at(13.5, y),
                &n.to_string(),
                cs * 0.36,
                if ch == 'X' {
                    Color32::from_gray(20)
                } else {
                    Color32::from_gray(245)
                },
            );
        }
        label(
            p,
            at(13.5, 3.5),
            crate::gui_i18n::tr("borne_off"),
            cs * 0.22,
            th.text().gamma_multiply(0.9),
        );
        // zaruri rămase, în centrul jumătății din dreapta
        let n = dice.len() as f32;
        for (i, &d) in dice.iter().enumerate() {
            let x = 10.0 - (n - 1.0) * 0.55 + i as f32 * 1.1;
            draw_die(p, at(x, 4.0), cs * 0.9, d.clamp(1, 6));
        }
    }
}

// ------------------------------------------------------- Ludo / Pachisi

const P1_COL: Color32 = Color32::from_rgb(70, 120, 225);
const P2_COL: Color32 = Color32::from_rgb(235, 145, 40);

fn draw_race(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let Extra::Race {
        ring,
        home,
        goal,
        safe,
        roll,
    } = b.extra.clone()
    else {
        return;
    };
    let w = board::race_w(ring);
    let cs = l.cell;
    let sq_rect = |(c, r): (usize, usize)| {
        Rect::from_min_size(
            l.origin + Vec2::new(c as f32 * cs, r as f32 * cs),
            Vec2::splat(cs),
        )
        .shrink(cs * 0.05)
    };
    let full = Rect::from_min_size(l.origin, Vec2::splat(cs * w as f32));
    p.rect_filled(full, 6.0, th.wood());
    let tile = if th.dark {
        Color32::from_rgb(190, 175, 140)
    } else {
        Color32::from_rgb(250, 240, 215)
    };
    let tsz = (cs * 0.2).clamp(7.0, 12.0);
    for sq in 0..ring {
        let rc = sq_rect(board::ring_xy(ring, sq));
        let mut col = tile;
        if sq == 0 {
            col = P1_COL.gamma_multiply(0.7);
        } else if sq == ring / 2 {
            col = P2_COL.gamma_multiply(0.7);
        }
        p.rect_filled(rc, cs * 0.08, col);
        if safe > 0 && sq % safe == 0 {
            let c = rc.center();
            let d = cs * 0.2;
            p.add(Shape::convex_polygon(
                vec![
                    c + Vec2::new(0.0, -d),
                    c + Vec2::new(d, 0.0),
                    c + Vec2::new(0.0, d),
                    c + Vec2::new(-d, 0.0),
                ],
                Color32::from_rgb(210, 150, 30),
                Stroke::new(1.0, Color32::from_gray(60)),
            ));
        }
        label(
            p,
            rc.left_top() + Vec2::new(tsz * 0.7, tsz * 0.6),
            &sq.to_string(),
            tsz,
            Color32::from_gray(90),
        );
    }
    for (pl, colr) in [(0usize, P1_COL), (1, P2_COL)] {
        for k in 1..=home as u32 + 1 {
            let q = (goal as usize - home - 1) as u32 + k;
            let rc = sq_rect(board::race_pos(ring, home, goal, pl, q, 0));
            p.rect_filled(
                rc,
                cs * 0.08,
                colr.gamma_multiply(if q == goal { 0.9 } else { 0.45 }),
            );
            if q == goal {
                p.circle_stroke(
                    rc.center(),
                    cs * 0.3,
                    Stroke::new(2.0, Color32::from_gray(250)),
                );
                p.circle_stroke(
                    rc.center(),
                    cs * 0.15,
                    Stroke::new(2.0, Color32::from_gray(250)),
                );
            }
        }
        // baza
        let r0 = sq_rect(board::race_pos(ring, home, goal, pl, 0, 0));
        let r3 = sq_rect(board::race_pos(ring, home, goal, pl, 0, 3));
        let base = Rect::from_min_max(r0.min.min(r3.min), r0.max.max(r3.max)).expand(cs * 0.1);
        p.rect_filled(base, cs * 0.2, colr.gamma_multiply(0.3));
    }
    if let Some(r) = roll {
        let c = l.origin + Vec2::splat(cs * w as f32 / 2.0);
        if b.kind == Kind::Ludo && (1..=6).contains(&r) {
            draw_die(p, c, cs * 1.2, r as u8);
        } else {
            label(p, c, &r.to_string(), cs * 1.0, th.text());
            label(
                p,
                c + Vec2::new(0.0, cs * 0.8),
                if b.kind == Kind::Ludo {
                    crate::gui_i18n::tr("die")
                } else {
                    crate::gui_i18n::tr("roll")
                },
                cs * 0.28,
                th.text().gamma_multiply(0.8),
            );
        }
    }
    // piese; cele de pe același pătrat sunt ușor decalate
    let mut seen: Vec<(usize, usize, usize)> = Vec::new();
    for c in &b.cells {
        let k = seen.iter().filter(|s| s.0 == c.col && s.1 == c.row).count();
        seen.push((c.col, c.row, k));
        let pos = l.center(c) + Vec2::new(k as f32 * cs * 0.16, -(k as f32) * cs * 0.1);
        let (fill, edge) = if c.ch == 'X' {
            (P1_COL, Color32::from_gray(250))
        } else {
            (P2_COL, Color32::from_gray(30))
        };
        disc(p, pos, cs * 0.3, fill, edge);
        if c.num > 0 && (c.num as u32) < goal {
            label(
                p,
                pos,
                &c.num.to_string(),
                cs * 0.26,
                Color32::from_gray(250),
            );
        }
    }
}

// ------------------------------------------- Hex, Amazoane, Linii, Konane, ...

fn board_rect(b: &Board, l: &Layout) -> Rect {
    Rect::from_min_size(
        l.origin,
        Vec2::new(l.cell * b.cols as f32, l.cell * b.rows as f32),
    )
}

fn checker(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    for c in &b.cells {
        let col = if (c.col + c.row) % 2 == 0 {
            th.sq_light()
        } else {
            th.sq_dark()
        };
        square_bg(p, l, c, col);
    }
}

/// Hexagon cu vârful în sus, centrat în `c`, cu raza circumscrisă `r`.
fn hex_points(c: Pos2, r: f32) -> Vec<Pos2> {
    (0..6)
        .map(|k| {
            let a = std::f32::consts::FRAC_PI_3 * k as f32 - std::f32::consts::FRAC_PI_2;
            c + Vec2::new(a.cos(), a.sin()) * r
        })
        .collect()
}

fn draw_hex(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = l.cell / 3f32.sqrt();
    let edge_x = P1_COL;
    let edge_o = P2_COL;
    // marginile colorate: X sus–jos, O stânga–dreapta
    let first = |row: usize, col: usize| b.cells.iter().find(|c| c.row == row && c.col == col);
    let (n, m) = (b.rows - 1, b.cols - 1);
    let (tl, tr, bl, br) = (first(0, 0), first(0, m), first(n, 0), first(n, m));
    if let (Some(tl), Some(tr), Some(bl), Some(br)) = (tl, tr, bl, br) {
        let (tl, tr, bl, br) = (l.center(tl), l.center(tr), l.center(bl), l.center(br));
        let up = Vec2::new(0.0, -r * 1.25);
        let side = Vec2::new(l.cell * 0.62, 0.0);
        p.line_segment([tl + up, tr + up], Stroke::new(l.cell * 0.14, edge_x));
        p.line_segment([bl - up, br - up], Stroke::new(l.cell * 0.14, edge_x));
        p.line_segment([tl - side, bl - side], Stroke::new(l.cell * 0.14, edge_o));
        p.line_segment([tr + side, br + side], Stroke::new(l.cell * 0.14, edge_o));
    }
    for c in &b.cells {
        let pos = l.center(c);
        p.add(Shape::convex_polygon(
            hex_points(pos, r * 0.97),
            th.sq_light(),
            Stroke::new(1.0, th.line()),
        ));
        match c.ch {
            'X' => disc(p, pos, l.cell * 0.36, P1_COL, Color32::from_gray(250)),
            'O' => disc(p, pos, l.cell * 0.36, P2_COL, Color32::from_gray(30)),
            _ => {}
        }
    }
}

fn draw_loa(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    checker(p, b, l, th);
    for c in &b.cells {
        stone(p, l.center(c), c.ch, l.cell * 0.4);
    }
}

fn draw_amazons(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    checker(p, b, l, th);
    for c in &b.cells {
        let pos = l.center(c);
        match c.ch {
            '#' => {
                let rc = Rect::from_center_size(pos, Vec2::splat(l.cell * 0.78));
                p.rect_filled(rc, l.cell * 0.1, Color32::from_gray(40));
                let d = l.cell * 0.22;
                let st = Stroke::new(l.cell * 0.07, Color32::from_rgb(230, 90, 40));
                p.line_segment([pos + Vec2::new(-d, -d), pos + Vec2::new(d, d)], st);
                p.line_segment([pos + Vec2::new(-d, d), pos + Vec2::new(d, -d)], st);
            }
            'X' | 'O' => {
                let (fill, edge, tx) = if c.ch == 'X' {
                    (
                        Color32::from_gray(250),
                        Color32::from_gray(40),
                        Color32::from_gray(20),
                    )
                } else {
                    (
                        Color32::from_gray(35),
                        Color32::from_gray(230),
                        Color32::from_gray(245),
                    )
                };
                disc(p, pos, l.cell * 0.38, fill, edge);
                label(p, pos, "A", l.cell * 0.4, tx);
            }
            _ => {}
        }
    }
}

fn draw_konane(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = board_rect(b, l);
    p.rect_filled(r, 4.0, th.wood());
    for c in &b.cells {
        if (c.col + c.row) % 2 == 0 {
            square_bg(p, l, c, th.wood().gamma_multiply(0.85));
        }
    }
    for i in 0..=b.cols {
        let x = l.origin.x + i as f32 * l.cell;
        p.line_segment(
            [Pos2::new(x, r.top()), Pos2::new(x, r.bottom())],
            Stroke::new(1.0, th.line()),
        );
    }
    for j in 0..=b.rows {
        let y = l.origin.y + j as f32 * l.cell;
        p.line_segment(
            [Pos2::new(r.left(), y), Pos2::new(r.right(), y)],
            Stroke::new(1.0, th.line()),
        );
    }
    for c in &b.cells {
        stone(p, l.center(c), c.ch, l.cell * 0.4);
    }
}

fn draw_hasami(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = board_rect(b, l);
    p.rect_filled(r, 4.0, th.wood());
    for i in 0..=b.cols {
        let x = l.origin.x + i as f32 * l.cell;
        p.line_segment(
            [Pos2::new(x, r.top()), Pos2::new(x, r.bottom())],
            Stroke::new(1.0, th.line()),
        );
    }
    for j in 0..=b.rows {
        let y = l.origin.y + j as f32 * l.cell;
        p.line_segment(
            [Pos2::new(r.left(), y), Pos2::new(r.right(), y)],
            Stroke::new(1.0, th.line()),
        );
    }
    for c in &b.cells {
        let (fill, edge) = match c.ch {
            'X' => (Color32::from_rgb(45, 40, 50), Color32::from_gray(210)),
            'O' => (Color32::from_rgb(245, 238, 215), Color32::from_gray(50)),
            _ => continue,
        };
        let rc = Rect::from_center_size(l.center(c), Vec2::splat(l.cell * 0.72));
        p.rect_filled(rc, l.cell * 0.12, fill);
        p.rect_stroke(
            rc,
            l.cell * 0.12,
            Stroke::new(1.5, edge),
            StrokeKind::Inside,
        );
    }
}

/// Grilă de linii cu diagonale prin punctele cu `(rang + coloană)` par
/// (rangul numărat de jos, ca în regulile jocului).
fn grid_lines(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let st = Stroke::new(1.2, th.line());
    let at = |col: usize, row: usize| {
        l.center(&Cell {
            name: String::new(),
            col,
            row,
            ch: ' ',
            num: 0,
        })
    };
    for r in 0..b.rows {
        p.line_segment([at(0, r), at(b.cols - 1, r)], st);
    }
    for c in 0..b.cols {
        p.line_segment([at(c, 0), at(c, b.rows - 1)], st);
    }
    for r in 0..b.rows - 1 {
        for c in 0..b.cols {
            if (b.rows - 1 - r + c).is_multiple_of(2) {
                if c + 1 < b.cols {
                    p.line_segment([at(c, r), at(c + 1, r + 1)], st);
                }
                if c > 0 {
                    p.line_segment([at(c, r), at(c - 1, r + 1)], st);
                }
            }
        }
    }
}

fn draw_lines_diag(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let pad = l.cell * 0.5;
    let r = board_rect(b, l);
    p.rect_filled(r.expand(pad * 0.2), 6.0, th.wood());
    grid_lines(p, b, l, th);
    let tigers = b.kind == Kind::Tigers;
    for c in &b.cells {
        let pos = l.center(c);
        if c.ch == '+' {
            p.circle_filled(pos, l.cell * 0.06, th.line());
            continue;
        }
        let (fill, edge, tx, tag) = match (tigers, c.ch) {
            (true, 'T') => (
                Color32::from_rgb(235, 145, 40),
                Color32::from_gray(30),
                Color32::from_gray(20),
                "T",
            ),
            (true, _) => (
                Color32::from_rgb(235, 240, 225),
                Color32::from_gray(60),
                Color32::from_gray(20),
                "G",
            ),
            (false, 'X') => (
                Color32::from_rgb(245, 240, 225),
                Color32::from_gray(60),
                Color32::from_gray(20),
                "",
            ),
            _ => (
                Color32::from_rgb(40, 35, 45),
                Color32::from_gray(210),
                Color32::from_gray(240),
                "",
            ),
        };
        disc(p, pos, l.cell * 0.34, fill, edge);
        if !tag.is_empty() {
            label(p, pos, tag, l.cell * 0.36, tx);
        }
    }
}

/// Arc de cerc (unghiuri în radiani, sensul ecranului: y în jos) ca linie frântă.
fn arc(p: &Painter, c: Pos2, r: f32, a0: f32, a1: f32, st: Stroke) {
    let n = 40;
    let pts: Vec<Pos2> = (0..=n)
        .map(|k| {
            let a = a0 + (a1 - a0) * k as f32 / n as f32;
            c + Vec2::new(a.cos(), a.sin()) * r
        })
        .collect();
    p.add(Shape::line(pts, st));
}

fn draw_surakarta(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let cs = l.cell;
    let pad = cs * l.pad;
    let full = Rect::from_min_size(
        l.origin,
        Vec2::new(
            cs * b.cols as f32 + 2.0 * pad,
            cs * b.rows as f32 + 2.0 * pad,
        ),
    );
    p.rect_filled(full, 8.0, th.wood());
    let at = |col: usize, row: usize| {
        l.origin + Vec2::new((col as f32 + 0.5) * cs + pad, (row as f32 + 0.5) * cs + pad)
    };
    let (n, m) = (b.rows - 1, b.cols - 1);
    let thin = Stroke::new(1.0, th.line());
    let small = Stroke::new(
        (cs * 0.05).max(2.0),
        Color32::from_rgb(200, 50, 50).gamma_multiply(0.85),
    );
    let big = Stroke::new(
        (cs * 0.05).max(2.0),
        Color32::from_rgb(50, 90, 200).gamma_multiply(0.85),
    );
    for r in 0..b.rows {
        let st = match r {
            1 | 4 => small,
            2 | 3 => big,
            _ => thin,
        };
        p.line_segment([at(0, r), at(m, r)], st);
    }
    for c in 0..b.cols {
        let st = match c {
            1 | 4 => small,
            2 | 3 => big,
            _ => thin,
        };
        p.line_segment([at(c, 0), at(c, n)], st);
    }
    for a in surakarta_arcs(b, l) {
        arc(p, a.ctr, a.r, a.a0, a.a1, if a.big { big } else { small });
    }
    for c in &b.cells {
        let pos = l.center(c);
        match c.ch {
            'X' => disc(p, pos, cs * 0.34, P1_COL, Color32::from_gray(250)),
            'O' => disc(p, pos, cs * 0.34, P2_COL, Color32::from_gray(30)),
            _ => {
                p.circle_filled(pos, cs * 0.06, th.line());
            }
        }
    }
}

/// Un arc de circuit: 270° în jurul unui colț al grilei, de la capătul unui rând la
/// capătul coloanei cu același număr (circuitul mic: linia 2, cel mare: linia 3).
#[derive(Clone, Copy, Debug)]
pub struct SArc {
    pub ctr: Pos2,
    pub r: f32,
    pub a0: f32,
    pub a1: f32,
    pub big: bool,
}

impl SArc {
    pub fn start(&self) -> Pos2 {
        self.ctr + Vec2::new(self.a0.cos(), self.a0.sin()) * self.r
    }
    pub fn end(&self) -> Pos2 {
        self.ctr + Vec2::new(self.a1.cos(), self.a1.sin()) * self.r
    }
}

pub fn surakarta_arcs(b: &Board, l: &Layout) -> Vec<SArc> {
    use std::f32::consts::{FRAC_PI_2, PI};
    let cs = l.cell;
    let pad = cs * l.pad;
    let at = |col: usize, row: usize| {
        l.origin + Vec2::new((col as f32 + 0.5) * cs + pad, (row as f32 + 0.5) * cs + pad)
    };
    let (n, m) = (b.rows - 1, b.cols - 1);
    // (colț, unghi de start, sens): de sub/de deasupra colțului, în jurul lui
    let corners = [
        (at(0, 0), PI / 2.0, 1.0f32),
        (at(m, 0), PI / 2.0, -1.0),
        (at(0, n), -PI / 2.0, -1.0),
        (at(m, n), -PI / 2.0, 1.0),
    ];
    let mut out = Vec::new();
    for (ctr, a0, dir) in corners {
        for (rad, big) in [(1.0f32, false), (2.0, true)] {
            out.push(SArc {
                ctr,
                r: cs * rad,
                a0,
                a1: a0 + dir * 3.0 * FRAC_PI_2,
                big,
            });
        }
    }
    out
}

// ------------------------------------------------ Puncte, Strapungere, Izolare, Rotire,
// ------------------------------------------------ Coridor, Drum, Trei-pierde, Nim

const P1: Color32 = Color32::from_rgb(60, 110, 200);
const P2: Color32 = Color32::from_rgb(225, 105, 55);

fn board_union(l: &Layout, b: &Board) -> Rect {
    Rect::from_min_max(l.rect_cr(0, 0).min, l.rect_cr(b.cols - 1, b.rows - 1).max)
}

fn draw_puncte(p: &Painter, b: &Board, l: &Layout, th: &Theme, hover: &BTreeSet<String>) {
    p.rect_filled(board_union(l, b).expand(l.cell * 0.25), 6.0, th.wood());
    let g = l.cell * board::GAP;
    // căsuțe: colorate după proprietar
    for c in b.cells.iter().filter(|c| c.name.starts_with("~b")) {
        let r = l.rect(c).shrink(l.cell * 0.05);
        let (fill, tx) = match c.ch {
            'X' => (P1, Color32::WHITE),
            'O' => (P2, Color32::WHITE),
            _ => (th.wood().gamma_multiply(0.85), th.text()),
        };
        p.rect_filled(r, l.cell * 0.08, fill);
        if c.ch != ' ' {
            label(p, r.center(), &c.ch.to_string(), l.cell * 0.5, tx);
        }
    }
    // laturi: trase = groase; libere = estompate; sub mouse = verde
    for c in &b.cells {
        let horiz = c.name.starts_with('o');
        if !(horiz || c.name.starts_with('v')) {
            continue;
        }
        let r = l.rect(c);
        let (a, z) = if horiz {
            (
                Pos2::new(r.left() - g / 2.0, r.center().y),
                Pos2::new(r.right() + g / 2.0, r.center().y),
            )
        } else {
            (
                Pos2::new(r.center().x, r.top() - g / 2.0),
                Pos2::new(r.center().x, r.bottom() + g / 2.0),
            )
        };
        let st = if c.ch != ' ' {
            Stroke::new(l.cell * 0.11, th.line())
        } else if hover.contains(&c.name) {
            Stroke::new(l.cell * 0.11, NEXT)
        } else {
            Stroke::new(l.cell * 0.035, th.line().gamma_multiply(0.3))
        };
        p.line_segment([a, z], st);
    }
    for c in b.cells.iter().filter(|c| c.name.starts_with("~d")) {
        p.circle_filled(l.center(c), l.cell * 0.1, th.line());
    }
}

fn draw_strap(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    for c in &b.cells {
        let base = if (c.col + c.row) % 2 == 0 {
            th.sq_light()
        } else {
            th.sq_dark()
        };
        square_bg(p, l, c, base);
        // rândurile-țintă: sus pentru X (urcă), jos pentru O (coboară)
        if c.row == 0 {
            p.rect_filled(l.rect(c), 0.0, P1.gamma_multiply(0.22));
        } else if c.row == b.rows - 1 {
            p.rect_filled(l.rect(c), 0.0, P2.gamma_multiply(0.22));
        }
    }
    for c in &b.cells {
        let pos = l.center(c);
        match c.ch {
            'X' => disc(
                p,
                pos,
                l.cell * 0.36,
                Color32::from_gray(245),
                Color32::from_gray(40),
            ),
            'O' => disc(
                p,
                pos,
                l.cell * 0.36,
                Color32::from_gray(40),
                Color32::from_gray(225),
            ),
            _ => {}
        }
    }
}

fn draw_izolare(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    for c in &b.cells {
        if c.ch == '#' {
            let r = l.rect(c);
            p.rect_filled(r, 0.0, Color32::from_rgb(14, 14, 20));
            let st = Stroke::new(l.cell * 0.03, Color32::from_gray(60));
            p.line_segment(
                [
                    r.left_top() + Vec2::splat(l.cell * 0.2),
                    r.right_bottom() - Vec2::splat(l.cell * 0.2),
                ],
                st,
            );
            p.line_segment(
                [
                    r.right_top() + Vec2::new(-l.cell * 0.2, l.cell * 0.2),
                    r.left_bottom() + Vec2::new(l.cell * 0.2, -l.cell * 0.2),
                ],
                st,
            );
        } else {
            square_bg(
                p,
                l,
                c,
                if (c.col + c.row) % 2 == 0 {
                    th.sq_light()
                } else {
                    th.sq_dark()
                },
            );
        }
    }
    for c in &b.cells {
        let pos = l.center(c);
        match c.ch {
            'X' => disc(p, pos, l.cell * 0.34, P1, Color32::WHITE),
            'O' => disc(p, pos, l.cell * 0.34, P2, Color32::WHITE),
            _ => {}
        }
    }
}

/// Săgeată circulară (orară sau antiorară) în jurul lui `c`.
fn arc_arrow(p: &Painter, c: Pos2, r: f32, cw: bool, col: Color32, w: f32) {
    let sign = if cw { 1.0f32 } else { -1.0 };
    let a0: f32 = if cw { -2.2 } else { -0.94 };
    let sweep = 4.2 * sign;
    let n = 24;
    let pt = |a: f32| c + Vec2::new(a.cos(), a.sin()) * r;
    let pts: Vec<Pos2> = (0..=n)
        .map(|k| pt(a0 + sweep * k as f32 / n as f32))
        .collect();
    p.add(Shape::line(pts, Stroke::new(w, col)));
    let a1 = a0 + sweep;
    let e = pt(a1);
    let t = Vec2::new(-a1.sin(), a1.cos()) * sign;
    let nrm = Vec2::new(t.y, -t.x);
    let h = r * 0.55;
    p.add(Shape::convex_polygon(
        vec![
            e + t * h,
            e - t * h * 0.2 + nrm * h * 0.7,
            e - t * h * 0.2 - nrm * h * 0.7,
        ],
        col,
        Stroke::NONE,
    ));
}

fn draw_rotire(
    p: &Painter,
    b: &Board,
    l: &Layout,
    th: &Theme,
    sel: &[String],
    next: &BTreeSet<String>,
) {
    let full = Rect::from_min_max(l.rect_cr(1, 1).min, l.rect_cr(6, 6).max);
    p.rect_filled(full.expand(l.cell * 0.08), 6.0, th.line());
    for (k, (qc, qr)) in [(1usize, 1usize), (4, 1), (1, 4), (4, 4)]
        .into_iter()
        .enumerate()
    {
        let q = Rect::from_min_max(l.rect_cr(qc, qr).min, l.rect_cr(qc + 2, qr + 2).max)
            .shrink(l.cell * 0.025);
        p.rect_filled(
            q,
            4.0,
            if k == 0 || k == 3 {
                th.sq_light()
            } else {
                th.sq_dark()
            },
        );
        // numărul cadranului, estompat, în colț
        label(
            p,
            q.left_top() + Vec2::splat(l.cell * 0.2),
            &(k + 1).to_string(),
            l.cell * 0.22,
            th.text().gamma_multiply(0.45),
        );
    }
    let faint = th.text().gamma_multiply(0.4);
    for c in b.cells.iter().filter(|c| !c.name.starts_with('q')) {
        let pos = l.center(c);
        match c.ch {
            'X' | 'O' => stone(p, pos, c.ch, l.cell * 0.4),
            _ => {
                p.circle_stroke(pos, l.cell * 0.08, Stroke::new(1.0, faint));
                label(
                    p,
                    pos + Vec2::new(0.0, l.cell * 0.3),
                    &c.name,
                    l.cell * 0.17,
                    faint.gamma_multiply(0.7),
                );
            }
        }
    }
    let any_next = b
        .cells
        .iter()
        .any(|c| c.name.starts_with('q') && next.contains(&c.name));
    for c in b.cells.iter().filter(|c| c.name.starts_with('q')) {
        let pos = l.center(c);
        let on = next.contains(&c.name);
        let fill = if on {
            NEXT.gamma_multiply(0.35)
        } else {
            th.wood().gamma_multiply(0.5)
        };
        p.circle_filled(pos, l.cell * 0.36, fill);
        let col = if on {
            th.text()
        } else if any_next {
            faint
        } else {
            th.text().gamma_multiply(0.75)
        };
        arc_arrow(
            p,
            pos,
            l.cell * 0.2,
            c.ch == 'd',
            col,
            (l.cell * 0.06).max(1.5),
        );
        if sel.contains(&c.name) {
            p.circle_stroke(pos, l.cell * 0.38, Stroke::new(l.cell * 0.06, SEL));
        }
    }
}

/// Dreptunghiul unui zid: slotul `(x, y)` (de la 0), orizontal sau vertical.
fn wall_rect(l: &Layout, x: usize, y: usize, horizontal: bool) -> Rect {
    let w = l.cell * 0.22;
    let slot = l.rect_cr(2 * x + 1, 15 - 2 * y);
    if horizontal {
        let (a, z) = (
            l.rect_cr(2 * x, 16 - 2 * y),
            l.rect_cr(2 * x + 2, 16 - 2 * y),
        );
        Rect::from_min_max(
            Pos2::new(a.left(), slot.center().y - w / 2.0),
            Pos2::new(z.right(), slot.center().y + w / 2.0),
        )
    } else {
        let (top, bot) = (l.rect_cr(2 * x, 14 - 2 * y), l.rect_cr(2 * x, 16 - 2 * y));
        Rect::from_min_max(
            Pos2::new(slot.center().x - w / 2.0, top.top()),
            Pos2::new(slot.center().x + w / 2.0, bot.bottom()),
        )
    }
}

fn draw_coridor(
    p: &Painter,
    b: &Board,
    l: &Layout,
    th: &Theme,
    sel: &[String],
    next: &BTreeSet<String>,
    hover: &BTreeSet<String>,
) {
    p.rect_filled(board_union(l, b).expand(l.cell * 0.25), 6.0, th.wood());
    for c in b.cells.iter().filter(|c| !c.name.starts_with("w-")) {
        let r = l.rect(c);
        p.rect_filled(r, l.cell * 0.06, th.sq_light());
        if c.row == 0 {
            p.rect_filled(r, l.cell * 0.06, P1.gamma_multiply(0.25));
        } else if c.row == b.rows - 1 {
            p.rect_filled(r, l.cell * 0.06, P2.gamma_multiply(0.25));
        }
    }
    let wall = if th.dark {
        Color32::from_rgb(230, 170, 60)
    } else {
        Color32::from_rgb(110, 50, 15)
    };
    if let Extra::Walls { h, v, .. } = &b.extra {
        for &(x, y) in h {
            p.rect_filled(wall_rect(l, x, y, true), l.cell * 0.06, wall);
        }
        for &(x, y) in v {
            p.rect_filled(wall_rect(l, x, y, false), l.cell * 0.06, wall);
        }
    }
    for c in b.cells.iter().filter(|c| !c.name.starts_with("w-")) {
        let pos = l.center(c);
        match c.ch {
            'X' => disc(p, pos, l.cell * 0.34, P1, Color32::WHITE),
            'O' => disc(p, pos, l.cell * 0.34, P2, Color32::WHITE),
            _ => {}
        }
    }
    for c in b.cells.iter().filter(|c| c.name.starts_with("w-")) {
        let pos = l.center(c);
        let (x, y) = ((c.col - 1) / 2, (15 - c.row) / 2);
        if sel.contains(&c.name) {
            p.rect_filled(
                wall_rect(l, x, y, true),
                l.cell * 0.06,
                SEL.gamma_multiply(0.6),
            );
            p.rect_filled(
                wall_rect(l, x, y, false),
                l.cell * 0.06,
                SEL.gamma_multiply(0.6),
            );
        } else if hover.contains(&c.name) {
            p.rect_filled(
                wall_rect(l, x, y, true),
                l.cell * 0.06,
                NEXT.gamma_multiply(0.45),
            );
            p.rect_filled(
                wall_rect(l, x, y, false),
                l.cell * 0.06,
                NEXT.gamma_multiply(0.45),
            );
        } else if next.contains(&c.name) {
            p.circle_filled(pos, l.cell * 0.045, NEXT.gamma_multiply(0.8));
        }
    }
}

fn piece_colors(owner_one: bool) -> (Color32, Color32) {
    if owner_one {
        (Color32::from_rgb(245, 240, 225), Color32::from_gray(40))
    } else {
        (Color32::from_rgb(40, 40, 52), Color32::from_gray(215))
    }
}

fn draw_drum(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let empty = Vec::new();
    let stacks = match &b.extra {
        Extra::Stacks(s) => s,
        _ => &empty,
    };
    for (i, c) in b.cells.iter().enumerate() {
        let r = l.rect(c);
        p.rect_filled(
            r,
            0.0,
            if (c.col + c.row) % 2 == 0 {
                th.sq_light()
            } else {
                th.sq_dark()
            },
        );
        let Some(st) = stacks.get(i) else { continue };
        let chars: Vec<char> = st.chars().collect();
        let n = chars.len();
        if n == 0 {
            continue;
        }
        let thick = (l.cell * 0.7 / n.max(4) as f32).min(l.cell * 0.16);
        let mut y = r.bottom() - l.cell * 0.1;
        for (k, &ch) in chars.iter().enumerate() {
            let one = matches!(ch, 'x' | 'X' | '@');
            let (fill, edge) = piece_colors(one);
            let top = k + 1 == n;
            let cx = r.center().x;
            match ch {
                'X' | 'O' if top => {
                    let h = l.cell * 0.46;
                    let rc = Rect::from_min_max(
                        Pos2::new(cx - l.cell * 0.09, y - h),
                        Pos2::new(cx + l.cell * 0.09, y),
                    );
                    p.rect_filled(rc, l.cell * 0.03, fill);
                    p.rect_stroke(
                        rc,
                        l.cell * 0.03,
                        Stroke::new(1.0, edge),
                        StrokeKind::Inside,
                    );
                }
                '@' | '#' => {
                    disc(
                        p,
                        Pos2::new(cx, y - l.cell * 0.18),
                        l.cell * 0.18,
                        fill,
                        edge,
                    );
                }
                _ => {
                    let rc = Rect::from_min_max(
                        Pos2::new(cx - l.cell * 0.34, y - thick * 0.92),
                        Pos2::new(cx + l.cell * 0.34, y),
                    );
                    p.rect_filled(rc, l.cell * 0.03, fill);
                    p.rect_stroke(
                        rc,
                        l.cell * 0.03,
                        Stroke::new(1.0, edge),
                        StrokeKind::Inside,
                    );
                    y -= thick;
                }
            }
        }
        if n >= 2 {
            label(
                p,
                r.right_top() + Vec2::new(-l.cell * 0.13, l.cell * 0.13),
                &n.to_string(),
                l.cell * 0.2,
                th.text(),
            );
        }
    }
}

fn draw_tripierd(p: &Painter, b: &Board, l: &Layout, th: &Theme) {
    let r = l.cell / 3f32.sqrt();
    for c in &b.cells {
        p.add(Shape::convex_polygon(
            hex_points(l.center(c), r * 0.97),
            th.wood(),
            Stroke::new((l.cell * 0.03).max(1.0), th.line()),
        ));
    }
    for c in &b.cells {
        stone(p, l.center(c), c.ch, l.cell * 0.36);
        // numele celulei, estompat, pe cele goale (mutarea se poate și scrie)
        if c.ch == '.' {
            label(
                p,
                l.center(c) + Vec2::new(0.0, l.cell * 0.27),
                &c.name,
                l.cell * 0.16,
                th.text().gamma_multiply(0.4),
            );
        }
    }
}

fn draw_nim(p: &Painter, b: &Board, l: &Layout, th: &Theme, hover: &BTreeSet<String>) {
    let w = Rect::from_min_max(l.rect_cr(0, 0).min, l.rect_cr(b.cols - 1, b.rows - 1).max);
    p.rect_filled(w.expand(l.cell * 0.2), 6.0, th.wood().gamma_multiply(0.6));
    let sz = (l.cell * 0.3).clamp(10.0, 22.0);
    for row in 0..b.rows {
        let y = l.rect_cr(0, row).center().y;
        let name = b
            .cells
            .iter()
            .find(|c| c.row == row)
            .map(|c| c.name.chars().next().unwrap_or('?'));
        let let_ = name.unwrap_or((b'a' + row as u8) as char);
        label(
            p,
            Pos2::new(w.left() - l.cell * 0.2 - sz * 0.5, y),
            &let_.to_string(),
            sz,
            th.text(),
        );
        let n = b.cells.iter().filter(|c| c.row == row).count();
        label(
            p,
            Pos2::new(w.right() + l.cell * 0.2 + sz * 0.6, y),
            &format!("({n})"),
            sz * 0.8,
            th.text().gamma_multiply(0.7),
        );
    }
    for c in &b.cells {
        let r = l.rect(c);
        let hot = hover.contains(&c.name);
        let rc = Rect::from_center_size(r.center(), Vec2::new(l.cell * 0.2, l.cell * 0.8));
        let fill = if hot {
            NEXT
        } else if th.dark {
            Color32::from_rgb(235, 215, 170)
        } else {
            Color32::from_rgb(250, 240, 210)
        };
        p.rect_filled(rc, l.cell * 0.1, fill);
        p.rect_stroke(
            rc,
            l.cell * 0.1,
            Stroke::new(1.0, th.line()),
            StrokeKind::Inside,
        );
    }
}
