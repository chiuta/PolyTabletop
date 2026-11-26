//! Tema vizuală „Fără Frontiere” (aceeași limbă grafică ca aplicațiile single-file
//! HTML de pe chiuta.ro) și fonturile încorporate.
//!
//! * Fonturi (toate încorporate cu `include_bytes!`, fără rețea la rulare):
//!   Cormorant Garamond (titluri), Lora (text), JetBrains Mono (monospace) — SIL OFL 1.1 —
//!   plus DejaVu Sans (licență Bitstream Vera) ca rezervă pentru simboluri
//!   (♠♥♦♣, piese de șah, zaruri, săgeți, forme geometrice).
//! * Paleta: întunecat = fundal #0d0a08 / panouri maronii / text crem / accent auriu;
//!   luminos = pergament (#efe9dc) cu accent maro-auriu #7a5a10.
//!
//! Agenții de desen folosesc [`palette`] pentru culori coerente cu tema.

// API partajat cu agenții de desen (cărți, table): nu toate câmpurile sunt folosite încă.
#![allow(dead_code)]

use eframe::egui::{
    self, epaint::Shadow, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId,
    Stroke, TextStyle,
};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

pub const FONT_CORMORANT: &[u8] = include_bytes!("../assets/fonts/CormorantGaramond-SemiBold.ttf");
pub const FONT_CORMORANT_REG: &[u8] =
    include_bytes!("../assets/fonts/CormorantGaramond-Regular.ttf");
pub const FONT_LORA: &[u8] = include_bytes!("../assets/fonts/Lora-Regular.ttf");
pub const FONT_LORA_MEDIUM: &[u8] = include_bytes!("../assets/fonts/Lora-Medium.ttf");
pub const FONT_MONO: &[u8] = include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf");
pub const FONT_DEJAVU: &[u8] = include_bytes!("../assets/fonts/DejaVuSans.ttf");

/// Toate fonturile încorporate, în ordinea lanțului de rezervă (nume, octeți).
pub const FONTS: [(&str, &[u8]); 6] = [
    ("Lora", FONT_LORA),
    ("LoraMedium", FONT_LORA_MEDIUM),
    ("CormorantSemiBold", FONT_CORMORANT),
    ("CormorantRegular", FONT_CORMORANT_REG),
    ("JetBrainsMono", FONT_MONO),
    ("DejaVuSans", FONT_DEJAVU),
];

/// Familie pentru titluri (Cormorant Garamond SemiBold).
pub fn heading_family() -> FontFamily {
    FontFamily::Name("heading".into())
}
/// Familie pentru butoane și accente (Lora Medium).
pub fn button_family() -> FontFamily {
    FontFamily::Name("button".into())
}
/// Familie serif display, subțire (Cormorant Garamond Regular) — subtitluri.
pub fn display_family() -> FontFamily {
    FontFamily::Name("display".into())
}

/// Instalează fonturile; se apelează o dată la crearea contextului.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fd = FontDefinitions::default();
    for (name, bytes) in FONTS {
        fd.font_data
            .insert(name.to_owned(), Arc::new(FontData::from_static(bytes)));
    }
    // pictogramele/emoji implicite din egui rămân la coadă (nu au litere)
    let keep = |old: &[String]| -> Vec<String> {
        old.iter()
            .filter(|n| n.to_lowercase().contains("emoji") || n.to_lowercase().contains("icon"))
            .cloned()
            .collect()
    };
    let tail_prop = keep(
        fd.families
            .get(&FontFamily::Proportional)
            .map_or(&[][..], |v| v),
    );
    let tail_mono = keep(
        fd.families
            .get(&FontFamily::Monospace)
            .map_or(&[][..], |v| v),
    );
    let chain = |first: &[&str], tail: &[String]| -> Vec<String> {
        let mut v: Vec<String> = first.iter().map(|s| (*s).to_owned()).collect();
        v.extend(tail.iter().cloned());
        v
    };
    fd.families.insert(
        FontFamily::Proportional,
        chain(&["Lora", "DejaVuSans"], &tail_prop),
    );
    fd.families.insert(
        FontFamily::Monospace,
        chain(&["JetBrainsMono", "DejaVuSans"], &tail_mono),
    );
    fd.families.insert(
        heading_family(),
        chain(&["CormorantSemiBold", "Lora", "DejaVuSans"], &tail_prop),
    );
    fd.families.insert(
        button_family(),
        chain(&["LoraMedium", "Lora", "DejaVuSans"], &tail_prop),
    );
    fd.families.insert(
        display_family(),
        chain(&["CormorantRegular", "Lora", "DejaVuSans"], &tail_prop),
    );
    ctx.set_fonts(fd);
}

/// Paleta „Fără Frontiere”, pentru cod de desen (cărți, table etc.).
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    /// fundalul zonei centrale
    pub bg: Color32,
    /// panouri laterale
    pub panel: Color32,
    /// ferestre, câmpuri, carduri de panou
    pub panel2: Color32,
    /// butoane în repaus
    pub panel3: Color32,
    /// butoane la hover
    pub panel4: Color32,
    pub text: Color32,
    /// text secundar
    pub muted: Color32,
    /// chenare subțiri
    pub border: Color32,
    /// fața cărților de joc
    pub card: Color32,
    /// bordura cărții
    pub card_edge: Color32,
    /// accent auriu (principal)
    pub gold: Color32,
    /// accent auriu deschis (hover, titluri)
    pub gold2: Color32,
    /// bordura „pill” aurie (accent cu transparență 35%)
    pub gold_line: Color32,
    pub red_suit: Color32,
    pub black_suit: Color32,
    pub ok: Color32,
    pub bad: Color32,
}

pub fn palette(dark: bool) -> Palette {
    if dark {
        Palette {
            bg: rgb(0x0d0a08),
            panel: rgb(0x14100a).linear_multiply(1.0),
            panel2: rgb(0x1c1508),
            panel3: rgb(0x221a0a),
            panel4: rgb(0x2a2010),
            text: rgb(0xe8e0d0),
            muted: rgb(0x948775),
            border: rgb(0x403830),
            card: rgb(0xf5ede0),
            card_edge: rgb(0xc8832a),
            gold: rgb(0xc8832a),
            gold2: rgb(0xe8c87a),
            gold_line: Color32::from_rgb(0x74, 0x50, 0x1e),
            red_suit: rgb(0xb02a22),
            black_suit: rgb(0x1a1510),
            ok: rgb(0x8fd9a0),
            bad: rgb(0xe4695c),
        }
    } else {
        Palette {
            bg: rgb(0xefe9dc),
            panel: rgb(0xf6efe0),
            panel2: rgb(0xfffaf0),
            panel3: rgb(0xe9dcc2),
            panel4: rgb(0xdfd0b3),
            text: rgb(0x2a2014),
            muted: rgb(0x5a4a33),
            border: rgb(0xcdb98c),
            card: rgb(0xfffaf0),
            card_edge: rgb(0x7a5a10),
            gold: rgb(0x7a5a10),
            gold2: rgb(0x9a6b18),
            gold_line: rgb(0xcdb98c),
            red_suit: rgb(0xa8281b),
            black_suit: rgb(0x1a1510),
            ok: rgb(0x1b6b37),
            bad: rgb(0xa8281b),
        }
    }
}

const fn rgb(c: u32) -> Color32 {
    Color32::from_rgb((c >> 16) as u8, (c >> 8) as u8, c as u8)
}

/// 0 = neaplicat, 1 = întunecat, 2 = luminos (ca să nu reconstruim stilul la fiecare cadru).
static APPLIED: AtomicU8 = AtomicU8::new(0);

/// Aplică tema (vizuale, stiluri de text, spațiere). Ieftină dacă tema nu s-a schimbat.
pub fn apply(ctx: &egui::Context, dark: bool) {
    let want = if dark { 1 } else { 2 };
    if APPLIED.swap(want, Ordering::Relaxed) == want {
        return;
    }
    let p = palette(dark);
    let mut v = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    let r6 = CornerRadius::same(6);
    let r8 = CornerRadius::same(8);
    v.override_text_color = None;
    v.panel_fill = p.panel;
    v.window_fill = p.panel2;
    v.window_stroke = Stroke::new(1.0, p.border);
    v.window_corner_radius = CornerRadius::same(12);
    v.menu_corner_radius = r8;
    v.window_shadow = Shadow {
        offset: [0, 6],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(if dark { 140 } else { 50 }),
    };
    v.popup_shadow = v.window_shadow;
    v.extreme_bg_color = if dark { p.bg } else { p.panel2 };
    v.faint_bg_color = p.panel2;
    v.code_bg_color = p.panel3;
    v.hyperlink_color = p.gold2;
    v.warn_fg_color = p.gold2;
    v.error_fg_color = p.bad;
    v.selection.bg_fill = if dark {
        Color32::from_rgb(0x5a, 0x3d, 0x14)
    } else {
        Color32::from_rgb(0xe3, 0xcb, 0x93)
    };
    v.selection.stroke = Stroke::new(1.0, p.gold2);
    v.weak_text_alpha = 0.7;

    let w = &mut v.widgets;
    w.noninteractive.bg_fill = p.panel;
    w.noninteractive.weak_bg_fill = p.panel;
    w.noninteractive.bg_stroke = Stroke::new(1.0, p.border);
    w.noninteractive.fg_stroke = Stroke::new(1.0, p.text);
    w.noninteractive.corner_radius = r6;
    w.inactive.bg_fill = p.panel3;
    w.inactive.weak_bg_fill = p.panel3;
    w.inactive.bg_stroke = Stroke::new(1.0, p.gold_line);
    w.inactive.fg_stroke = Stroke::new(1.0, p.text);
    w.inactive.corner_radius = r6;
    w.hovered.bg_fill = p.panel4;
    w.hovered.weak_bg_fill = p.panel4;
    w.hovered.bg_stroke = Stroke::new(1.0, p.gold);
    w.hovered.fg_stroke = Stroke::new(1.5, p.gold2);
    w.hovered.corner_radius = r6;
    w.hovered.expansion = 0.0;
    w.active.bg_fill = if dark { rgb(0x3a2a10) } else { rgb(0xcdbb98) };
    w.active.weak_bg_fill = w.active.bg_fill;
    w.active.bg_stroke = Stroke::new(1.0, p.gold2);
    w.active.fg_stroke = Stroke::new(2.0, p.gold2);
    w.active.corner_radius = r6;
    w.open.bg_fill = p.panel4;
    w.open.weak_bg_fill = p.panel4;
    w.open.bg_stroke = Stroke::new(1.0, p.gold);
    w.open.fg_stroke = Stroke::new(1.0, p.gold2);
    w.open.corner_radius = r6;
    let th = if dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    ctx.set_theme(th);
    ctx.set_visuals_of(th, v);

    ctx.style_mut_of(th, |s| {
        s.text_styles
            .insert(TextStyle::Heading, FontId::new(25.0, heading_family()));
        s.text_styles
            .insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
        s.text_styles
            .insert(TextStyle::Button, FontId::new(15.0, button_family()));
        s.text_styles.insert(
            TextStyle::Small,
            FontId::new(12.5, FontFamily::Proportional),
        );
        s.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Monospace),
        );
        s.spacing.item_spacing = egui::vec2(8.0, 6.0);
        s.spacing.button_padding = egui::vec2(10.0, 4.0);
        s.spacing.interact_size.y = 24.0;
        s.spacing.window_margin = egui::Margin::same(12);
        s.spacing.menu_margin = egui::Margin::same(8);
        s.spacing.indent = 18.0;
    });
}

/// Insignă „pill” cu bordură aurie (ca etichetele din aplicațiile Fără Frontiere).
pub fn pill(ui: &mut egui::Ui, dark: bool, text: &str) -> egui::Response {
    let p = palette(dark);
    egui::Frame::new()
        .stroke(Stroke::new(1.0, p.gold_line))
        .fill(p.panel2)
        .corner_radius(CornerRadius::same(255))
        .inner_margin(egui::Margin::symmetric(10, 2))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).size(12.5).color(p.gold2));
        })
        .response
}

/// Subtitlu serif cu litere spațiate (versaliile mici din aplicațiile de referință).
pub fn subtitle(ui: &mut egui::Ui, dark: bool, text: &str) {
    let p = palette(dark);
    let spaced: String = text
        .to_uppercase()
        .chars()
        .flat_map(|c| [c, '\u{200A}'])
        .collect();
    ui.label(
        egui::RichText::new(spaced)
            .font(FontId::new(13.0, display_family()))
            .color(p.gold),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use read_fonts::{types::GlyphId, FontRef, TableProvider};

    /// Caracterele ne-ASCII din jocuri (ro/en/fr/it/es/pt/de/hu), din `gui.tsv` și din
    /// desenele GUI — generat de auditul de caractere.
    const USED_CHARS: &str = include_str!("../assets/used_chars.txt");

    fn covered(c: char) -> bool {
        FONTS.iter().any(|(_, b)| {
            FontRef::new(b)
                .ok()
                .and_then(|f| f.cmap().ok())
                .and_then(|m| m.map_codepoint(c as u32))
                .is_some_and(|g| g != GlyphId::NOTDEF)
        })
    }

    #[test]
    fn toate_caracterele_folosite_sunt_acoperite() {
        let miss: String = USED_CHARS
            .chars()
            .filter(|c| !c.is_whitespace() && !covered(*c))
            .collect();
        assert!(miss.is_empty(), "caractere fără glifă: {miss}");
    }

    #[test]
    fn simboluri_de_baza_si_diacritice() {
        for c in "♠♥♦♣♔♕♖♗♘♙♚♛♜♝♞♟⚀⚅●○■□▲▼◆✓✗⚑←→↑↓ăâîșțĂÂÎȘȚéèêëçàùñãõüöőűß".chars()
        {
            assert!(covered(c), "lipsește U+{:04X} {c}", c as u32);
        }
    }

    #[test]
    fn paleta_are_contrast_suficient() {
        fn lum(c: Color32) -> f32 {
            let f = |x: u8| {
                let s = f32::from(x) / 255.0;
                if s <= 0.03928 {
                    s / 12.92
                } else {
                    ((s + 0.055) / 1.055).powf(2.4)
                }
            };
            0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
        }
        for dark in [true, false] {
            let p = palette(dark);
            let (a, b) = (lum(p.text), lum(p.panel));
            let ratio = (a.max(b) + 0.05) / (a.min(b) + 0.05);
            assert!(ratio >= 7.0, "contrast text/panou {ratio} (dark={dark})");
        }
    }
}
