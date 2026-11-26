//! Integrarea cărților / zarurilor / dominourilor desenate în sesiunea GUI.
//!
//! [`analyze`] decide (pe baza [`cards::spec_for`] și a jetoanelor recunoscute) dacă
//! jocul curent poate fi afișat cu cărți desenate; în caz contrar întoarce `None` și GUI-ul
//! păstrează butoanele text de până acum. Textul `render()` rămâne mereu vizibil sub imagine.

use crate::cards::{
    self, dice_rows, draw_back, draw_card, draw_die, draw_domino, draw_suit, CardState, DiceRow,
    Part, Role, Row, Slot, Spec, BACK_BROWN, GOLD, INK,
};
use eframe::egui::{
    self, Align2, Color32, CursorIcon, FontId, Id, Pos2, Rect, RichText, Sense, Stroke, StrokeKind,
    Ui, Vec2,
};

/// Ce se poate desena pentru starea curentă.
pub struct View {
    spec: Spec,
    rows: Vec<Row>,
    moves: Vec<(String, Option<Vec<Part>>)>,
    dice: Vec<DiceRow>,
    legal: Vec<String>,
}

/// Analizează o stare. `None` = nimic de desenat (rămân butoanele / textul).
pub fn analyze(id: &str, render: &str, legal: &[String]) -> Option<View> {
    let dice = if cards::has_dice(id) {
        dice_rows(render)
    } else {
        vec![]
    };
    let spec = cards::spec_for(id);
    if spec.is_none() && dice.is_empty() {
        return None;
    }
    let spec = spec.unwrap_or(Spec {
        kind: cards::SpecKind::Cards(cards::Deck::French),
        jokers: false,
    });
    let is_cards = cards::spec_for(id).is_some();
    let rows = if is_cards {
        cards::rows(spec, render)
    } else {
        vec![]
    };
    // bridge: în licitație jetoanele „2c”, „7h” sunt oferte, nu cărți
    let bidding = id == "bridge" && !render.contains("Contract:");
    let moves: Vec<(String, Option<Vec<Part>>)> = legal
        .iter()
        .map(|m| {
            let parts = if is_cards && !bidding {
                cards::parse_move(m, spec)
            } else {
                None
            };
            (m.clone(), parts)
        })
        .collect();
    if rows.is_empty() && dice.is_empty() && moves.iter().all(|(_, p)| p.is_none()) {
        return None;
    }
    Some(View {
        spec,
        rows,
        moves,
        dice,
        legal: legal.to_vec(),
    })
}

impl View {
    /// Acțiunile (mutările legale) sunt desenate de această vedere?
    pub fn handles_actions(&self) -> bool {
        self.moves.iter().any(|(_, p)| p.is_some()) || self.holdable().is_some()
    }

    fn holdable(&self) -> Option<&DiceRow> {
        self.dice
            .iter()
            .find(|d| d.holdable)
            .filter(|_| self.legal.iter().any(|m| m.starts_with("tine ")))
    }
}

// ───────────────────────── geometrie ─────────────────────────

struct Geo {
    step: f32,
    per_row: usize,
    rows: usize,
}

fn geo(aw: f32, n: usize, w: f32, min_frac: f32) -> Geo {
    let full = w * 1.1;
    let max_one = (((aw - w) / full).floor().max(0.0) as usize) + 1;
    if n <= max_one {
        return Geo {
            step: full,
            per_row: n.max(1),
            rows: 1,
        };
    }
    let min_step = w * min_frac;
    if n > 1 {
        let step = (aw - w) / (n as f32 - 1.0);
        if step >= min_step {
            return Geo {
                step: step.min(full),
                per_row: n,
                rows: 1,
            };
        }
    }
    let per_row = (((aw - w) / min_step).floor().max(1.0) as usize) + 1;
    Geo {
        step: min_step,
        per_row,
        rows: n.div_ceil(per_row),
    }
}

fn hand_width(n: usize) -> f32 {
    match n {
        0..=10 => 58.0,
        11..=20 => 50.0,
        _ => 44.0,
    }
}

fn label(ui: &mut Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .small()
            .color(Color32::from_rgb(0xc8, 0x9a, 0x5a)),
    );
}

fn sel_id(ui: &Ui) -> Id {
    ui.scope_id().with("cards_sel")
}

// ───────────────────────── desenare ─────────────────────────

fn draw_slot(p: &egui::Painter, rect: Rect, s: &Slot, st: CardState) {
    match s {
        Slot::Card(c) => draw_card(p, rect, c, st),
        Slot::Back => draw_back(p, rect, st),
        Slot::Domino(a, b) => draw_domino(p, rect, *a, *b, st),
        Slot::Pair(def, att) => {
            draw_card(p, rect, att, st);
            let r2 = rect.translate(Vec2::new(rect.width() * 0.45, rect.height() * 0.2));
            match def {
                Some(c) => draw_card(p, r2, c, st),
                None => {
                    p.rect_stroke(
                        r2,
                        rect.width() * 0.09,
                        Stroke::new(1.5, GOLD),
                        StrokeKind::Inside,
                    );
                }
            }
        }
    }
}

fn slot_size(s: &Slot, w: f32) -> Vec2 {
    match s {
        Slot::Domino(..) => Vec2::new(w * 1.5, w * 0.75),
        Slot::Pair(..) => Vec2::new(w * 1.45, w * 1.45 * 1.2),
        _ => Vec2::new(w, w * 1.45),
    }
}

/// Un rând de piese. `states`: `Some` = interactiv (stare per piesă, indice-click întors).
fn draw_row(
    ui: &mut Ui,
    salt: usize,
    slots: &[Slot],
    w: f32,
    states: Option<&[CardState]>,
) -> Option<usize> {
    let n = slots.len();
    let aw = ui.available_width().max(100.0);
    let sz = slot_size(&slots[0], w);
    let domino = matches!(slots[0], Slot::Domino(..));
    let g = geo(
        aw,
        n,
        sz.x,
        if domino {
            1.04
        } else if states.is_some() {
            0.5
        } else {
            0.38
        },
    );
    let row_h = sz.y + 6.0;
    let total = Vec2::new(aw, g.rows as f32 * row_h + 8.0);
    let (area, _) = ui.allocate_exact_size(total, Sense::hover());
    let mut clicked = None;
    let mut hovered: Option<usize> = None;
    let base = ui.scope_id().with(("row", salt));
    let rect_of = |i: usize| -> Rect {
        let (r, c) = (i / g.per_row, i % g.per_row);
        Rect::from_min_size(
            Pos2::new(
                area.min.x + c as f32 * g.step,
                area.min.y + 6.0 + r as f32 * row_h,
            ),
            sz,
        )
    };
    if let Some(st) = states {
        for (i, state) in st.iter().enumerate().take(n) {
            if *state == CardState::Disabled {
                continue;
            }
            let resp = ui.interact(rect_of(i), base.with(i), Sense::click());
            if resp.hovered() {
                hovered = Some(i);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            if resp.clicked() {
                clicked = Some(i);
            }
        }
    }
    let p = ui.painter().clone();
    for i in 0..n {
        if Some(i) == hovered {
            continue;
        }
        let st = states.map_or(CardState::Normal, |s| s[i]);
        draw_slot(&p, rect_of(i), &slots[i], st);
    }
    if let Some(i) = hovered {
        let st = match states.map(|s| s[i]) {
            Some(CardState::Selected) => CardState::Selected,
            _ => CardState::Hover,
        };
        draw_slot(
            &p,
            rect_of(i).translate(Vec2::new(0.0, -sz.y * 0.10)),
            &slots[i],
            st,
        );
    }
    clicked
}

fn text_w(ui: &Ui, s: &str, size: f32) -> f32 {
    ui.painter()
        .layout_no_wrap(s.to_string(), FontId::proportional(size), INK)
        .size()
        .x
}

/// O mutare compusă (cărți + separatori) ca un buton cu imagini. Întoarce `true` la click.
fn move_chip(ui: &mut Ui, parts: &[Part], w: f32) -> bool {
    let h = w * 1.45;
    let pad = 6.0;
    let gap = 4.0;
    // măsurare
    let mut width = pad;
    for pt in parts {
        width += match pt {
            Part::Card(_) => w + gap,
            Part::Domino(..) => w * 1.5 + gap,
            Part::Suit(_) => w * 0.4 + gap,
            Part::Arrow(_) => w * 0.35 + gap,
            Part::Text(t) => text_w(ui, t, 13.0) + gap,
        };
    }
    width += pad;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, h + 2.0 * pad), Sense::click());
    let p = ui.painter();
    let hov = resp.hovered();
    let bg = if hov {
        Color32::from_rgb(0x4a, 0x38, 0x1a)
    } else {
        BACK_BROWN
    };
    p.rect_filled(rect, 8.0, bg);
    p.rect_stroke(
        rect,
        8.0,
        Stroke::new(if hov { 2.0 } else { 1.0 }, GOLD),
        StrokeKind::Inside,
    );
    let mut x = rect.min.x + pad;
    let cy = rect.center().y;
    for pt in parts {
        match pt {
            Part::Card(c) => {
                draw_card(
                    p,
                    Rect::from_min_size(Pos2::new(x, cy - h / 2.0), Vec2::new(w, h)),
                    c,
                    CardState::Normal,
                );
                x += w + gap;
            }
            Part::Domino(a, b) => {
                draw_domino(
                    p,
                    Rect::from_min_size(Pos2::new(x, cy - w * 0.375), Vec2::new(w * 1.5, w * 0.75)),
                    *a,
                    *b,
                    CardState::Normal,
                );
                x += w * 1.5 + gap;
            }
            Part::Suit(s) => {
                // pe fundal brun: simbolul pe un disc crem
                let r = w * 0.16;
                p.circle_filled(Pos2::new(x + w * 0.2, cy), r * 1.5, cards::CREAM);
                draw_suit(p, Pos2::new(x + w * 0.2, cy), r, *s, false);
                x += w * 0.4 + gap;
            }
            Part::Arrow(left) => {
                let (cx, d) = (x + w * 0.17, w * 0.14);
                let pts = if *left {
                    vec![
                        Pos2::new(cx - d, cy),
                        Pos2::new(cx + d, cy - d),
                        Pos2::new(cx + d, cy + d),
                    ]
                } else {
                    vec![
                        Pos2::new(cx + d, cy),
                        Pos2::new(cx - d, cy - d),
                        Pos2::new(cx - d, cy + d),
                    ]
                };
                p.add(egui::Shape::convex_polygon(pts, GOLD, Stroke::NONE));
                x += w * 0.35 + gap;
            }
            Part::Text(t) => {
                p.text(
                    Pos2::new(x, cy),
                    Align2::LEFT_CENTER,
                    t,
                    FontId::proportional(13.0),
                    Color32::from_rgb(0xf5, 0xed, 0xe0),
                );
                x += text_w(ui, t, 13.0) + gap;
            }
        }
    }
    if hov {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }
    resp.clicked()
}

fn text_button(ui: &mut Ui, text: &str) -> bool {
    let b =
        egui::Button::new(RichText::new(text).size(17.0).strong()).min_size(Vec2::new(90.0, 40.0));
    ui.add(b).clicked()
}

fn first_piece(parts: &[Part]) -> Option<&Part> {
    parts.iter().find(|p| p.is_piece())
}

fn same_piece(a: &Part, b: &Part) -> bool {
    match (a, b) {
        (Part::Card(x), Part::Card(y)) => x == y,
        (Part::Domino(a1, b1), Part::Domino(a2, b2)) => {
            (a1, b1) == (a2, b2) || (a1, b1) == (b2, a2)
        }
        _ => false,
    }
}

fn slot_part(s: &Slot) -> Option<Part> {
    match s {
        Slot::Card(c) => Some(Part::Card(*c)),
        Slot::Domino(a, b) => Some(Part::Domino(*a, *b)),
        _ => None,
    }
}

/// Desenează tot ce poate: rânduri de cărți, mâna interactivă, mutările compuse.
/// Întoarce mutarea aleasă (textul de trimis jocului).
pub fn show(ui: &mut Ui, v: &View, my_turn: bool) -> Option<String> {
    let mut chosen: Option<String> = None;
    let sid = sel_id(ui);
    let piece_moves = v.moves.iter().any(|(_, p)| p.is_some());
    let selected: Option<Part> = ui.data(|d| d.get_temp::<Option<Part>>(sid)).flatten();
    let mut new_sel = selected.clone();
    // mutări „consumate” de mână
    let mut consumed = vec![false; v.moves.len()];
    let mut wait_sel = false;

    for (ri, row) in v.rows.iter().enumerate() {
        label(ui, &row.label);
        match row.role {
            Role::Hidden => {
                // doar dosuri, mici și suprapuse; numărul în etichetă
                let n = row.slots.len();
                if n > 0 {
                    ui.horizontal(|ui| {
                        let shown = n.min(14);
                        let w = 26.0;
                        let (area, _) = ui.allocate_exact_size(
                            Vec2::new(w + (shown as f32 - 1.0) * w * 0.35, w * 1.45),
                            Sense::hover(),
                        );
                        for i in 0..shown {
                            draw_back(
                                ui.painter(),
                                Rect::from_min_size(
                                    Pos2::new(area.min.x + i as f32 * w * 0.35, area.min.y),
                                    Vec2::new(w, w * 1.45),
                                ),
                                CardState::Normal,
                            );
                        }
                        ui.label(RichText::new(format!("×{n}")).small());
                    });
                }
            }
            Role::Table => {
                let w = if row.slots.iter().any(|s| matches!(s, Slot::Domino(..))) {
                    26.0
                } else {
                    38.0
                };
                draw_row(ui, ri, &row.slots, w, None);
            }
            Role::Hand => {
                let mut w = hand_width(row.slots.len());
                if row.slots.iter().any(|s| matches!(s, Slot::Domino(..))) {
                    w = 44.0;
                }
                let interactive = my_turn && piece_moves;
                if !interactive {
                    draw_row(ui, ri, &row.slots, w, None);
                    continue;
                }
                // legătura carte → mutări
                let mut states = Vec::new();
                let mut targets: Vec<Vec<usize>> = Vec::new();
                for s in &row.slots {
                    let part = slot_part(s);
                    let idxs: Vec<usize> = match &part {
                        Some(pt) => v
                            .moves
                            .iter()
                            .enumerate()
                            .filter(|(_, (_, ps))| {
                                ps.as_deref()
                                    .and_then(first_piece)
                                    .is_some_and(|f| same_piece(f, pt))
                            })
                            .map(|(i, _)| i)
                            .collect(),
                        None => vec![],
                    };
                    let st = if idxs.is_empty() {
                        CardState::Disabled
                    } else if idxs.len() > 1 && selected.as_ref() == part.as_ref() {
                        CardState::Selected
                    } else {
                        CardState::Normal
                    };
                    states.push(st);
                    targets.push(idxs);
                }
                for t in &targets {
                    if t.len() == 1 {
                        consumed[t[0]] = true;
                    } else {
                        for &i in t {
                            // se arată doar după selectarea cărții
                            let sel_ok = selected.as_ref().is_some_and(|sp| {
                                v.moves[i]
                                    .1
                                    .as_deref()
                                    .and_then(first_piece)
                                    .is_some_and(|f| same_piece(f, sp))
                            });
                            if !sel_ok {
                                consumed[i] = true;
                                wait_sel = true;
                            }
                        }
                    }
                }
                if let Some(i) = draw_row(ui, ri, &row.slots, w, Some(&states)) {
                    let t = &targets[i];
                    if t.len() == 1 {
                        chosen = Some(v.moves[t[0]].0.clone());
                    } else if t.len() > 1 {
                        let part = slot_part(&row.slots[i]);
                        new_sel = if new_sel == part { None } else { part };
                    }
                }
            }
        }
    }

    // zaruri (în aceeași vedere)
    if let Some(m) = show_dice(ui, v, my_turn) {
        chosen = Some(m);
    }

    // mutările rămase: cărți compuse și comenzi text
    if my_turn && v.handles_actions() {
        let rest: Vec<usize> = (0..v.moves.len()).filter(|&i| !consumed[i]).collect();
        let skip_tine = v.holdable().is_some();
        let rest: Vec<usize> = rest
            .into_iter()
            .filter(|&i| !(skip_tine && v.moves[i].0.starts_with("tine ")))
            .collect();
        if !rest.is_empty() || wait_sel {
            ui.add_space(4.0);
            if wait_sel && selected.is_none() {
                let what = if matches!(v.spec.kind, cards::SpecKind::Domino) {
                    "Apasă o piesă pentru a alege capătul"
                } else {
                    "Apasă o carte pentru a vedea variantele"
                };
                label(ui, what);
            }
            ui.horizontal_wrapped(|ui| {
                for &i in &rest {
                    let (raw, parts) = &v.moves[i];
                    let hit = match parts {
                        Some(ps) => {
                            let n_pieces = ps.iter().filter(|p| p.is_piece()).count();
                            let w = if n_pieces <= 1 { 52.0 } else { 40.0 };
                            move_chip(ui, ps, w)
                        }
                        None => text_button(ui, raw),
                    };
                    if hit {
                        chosen = Some(raw.clone());
                    }
                }
            });
        }
    }
    if new_sel != selected {
        ui.data_mut(|d| d.insert_temp(sid, new_sel));
    }
    chosen
}

// ───────────────────────── zaruri ─────────────────────────

fn held_id(ui: &Ui) -> Id {
    ui.scope_id().with("dice_held")
}

/// Zarurile: rânduri de zaruri desenate; la „Zaruri: [..]” (zarpoker/cinci zaruri) se pot ține.
pub fn show_dice(ui: &mut Ui, v: &View, my_turn: bool) -> Option<String> {
    let mut chosen = None;
    for (di, d) in v.dice.iter().enumerate() {
        label(ui, &d.label);
        let hold = d.holdable && my_turn && v.holdable().is_some();
        let hid = held_id(ui);
        let mut held: Vec<bool> = vec![false; d.values.len()];
        if hold {
            if let Some((vals, flags)) = ui.data(|m| m.get_temp::<(Vec<u8>, Vec<bool>)>(hid)) {
                if vals == d.values {
                    held = flags;
                }
            }
        }
        let size = if d.values.len() > 2 { 44.0 } else { 52.0 };
        ui.horizontal(|ui| {
            let mut changed = false;
            for (i, &val) in d.values.iter().enumerate() {
                let (rect, resp) = ui.allocate_exact_size(
                    Vec2::splat(size),
                    if hold { Sense::click() } else { Sense::hover() },
                );
                let r = if held[i] {
                    rect.translate(Vec2::new(0.0, -6.0))
                } else {
                    rect
                };
                let st = if held[i] {
                    CardState::Selected
                } else if hold && resp.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    CardState::Hover
                } else {
                    CardState::Normal
                };
                draw_die(ui.painter(), r, val, st);
                if hold && resp.clicked() {
                    held[i] = !held[i];
                    changed = true;
                }
            }
            if hold {
                let mut keep: Vec<u8> = d
                    .values
                    .iter()
                    .zip(&held)
                    .filter(|(_, h)| **h)
                    .map(|(v, _)| *v)
                    .collect();
                keep.sort_unstable();
                let mv = if keep.is_empty() {
                    "tine -".to_string()
                } else {
                    format!(
                        "tine {}",
                        keep.iter().map(u8::to_string).collect::<Vec<_>>().join(" ")
                    )
                };
                let ok = v.legal.contains(&mv);
                ui.add_space(8.0);
                if ui
                    .add_enabled(
                        ok,
                        egui::Button::new(RichText::new("Ține selecția").size(16.0)),
                    )
                    .clicked()
                {
                    chosen = Some(mv);
                }
            }
            if changed {
                ui.data_mut(|m| m.insert_temp(hid, (d.values.clone(), held.clone())));
            }
        });
        let _ = di;
    }
    chosen
}
