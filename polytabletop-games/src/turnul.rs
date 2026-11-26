//! Turnul Șovăitor — turn de blocuri cu simulare discretă a stabilității (variantă originală).
//!
//! # Model fizic documentat
//! * Turnul are straturi de câte 3 blocuri (sloturile `a`, `b`, `c`); stratul 1 e jos. Blocul are
//!   lățimea 1 și lungimea 3, iar straturile se rotesc alternativ cu 90°: într-un strat impar blocurile
//!   sunt lungi pe axa X (sloturile se așază pe Y: a = −1, b = 0, c = +1); într-un strat par
//!   blocurile sunt lungi pe axa Y (sloturile pe X). Toate au aceeași masă.
//! * Suprafața de sprijin a unui strat este învelișul convex al blocurilor lui rămase
//!   (dreptunghiul care cuprinde blocurile; golurile dintre ele contează ca sprijin).
//! * Turnul **stă** dacă pentru fiecare strat `i` centrul de masă al tuturor blocurilor aflate deasupra
//!   lui `i` cade în suprafața de sprijin a lui `i` (marginea inclusă). Un strat gol nu sprijină nimic.
//!   Exemple: poți scoate două blocuri dintr-un strat doar dacă rămâne cel din mijloc sau dacă
//!   centrul de masă de deasupra e încă pe blocul rămas; un singur bloc lateral nu ține un
//!   turn simetric.
//! * Mutare: scoți un bloc dintr-un strat aflat sub cel mai înalt strat **complet** (stratul complet
//!   cel mai de sus și cele de deasupra nu se ating) și îl așezi sus: într-un slot liber al
//!   stratului de sus dacă nu e complet, altfel începi un strat nou. Text: `ia 4b pune a`
//!   (stratul 4, slotul b → pe slotul a al stratului de sus).
//! * Dacă turnul nu mai stă după scoatere sau după așezare, cel care a mutat pierde. Nu există
//!   noroc: jocul este determinist; AI-ul caută 2 plies și minimizează mutările sigure ale adversarului.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const START_LAYERS: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub layer: usize,
    pub slot: usize,
    pub put: usize,
}

#[derive(Clone)]
pub struct Turnul {
    /// `layers[i][s]` = bloc prezent.
    layers: Vec<[bool; 3]>,
    side: Player,
    fallen: Option<Player>,
    last: String,
}

impl Default for Turnul {
    fn default() -> Self {
        Self::new()
    }
}

/// Centrul unui bloc în (x, y). Strat cu indice par (0, 2, …) = blocuri lungi pe X.
fn center(layer: usize, slot: usize) -> (f32, f32) {
    let off = slot as f32 - 1.0;
    if layer.is_multiple_of(2) {
        (0.0, off)
    } else {
        (off, 0.0)
    }
}

impl Turnul {
    pub fn new() -> Self {
        Turnul {
            layers: vec![[true; 3]; START_LAYERS],
            side: Player::One,
            fallen: None,
            last: String::new(),
        }
    }

    pub fn with_layers(n: usize) -> Self {
        let mut t = Self::new();
        t.layers = vec![[true; 3]; n.max(4)];
        t
    }

    /// Din fabrică nu depinde de noroc; `with_seed` doar pentru uniformitate cu celelalte jocuri.
    pub fn with_seed(_seed: u64) -> Self {
        Self::new()
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn blocks(&self) -> usize {
        self.layers
            .iter()
            .map(|l| l.iter().filter(|&&b| b).count())
            .sum()
    }

    /// Turnul stă? (`layers` poate fi orice configurație.)
    pub fn stands(layers: &[[bool; 3]]) -> bool {
        Self::margin(layers) >= 0.0
    }

    /// Cea mai mică distanță de la centrul de masă de deasupra la marginea sprijinului
    /// (negativ = se prăbușește).
    pub fn margin(layers: &[[bool; 3]]) -> f32 {
        let mut n = 0.0f32;
        let (mut sx, mut sy) = (0.0f32, 0.0f32);
        let mut worst = f32::MAX;
        for i in (0..layers.len()).rev() {
            if n > 0.0 {
                let (cx, cy) = (sx / n, sy / n);
                let present: Vec<usize> = (0..3).filter(|&s| layers[i][s]).collect();
                if present.is_empty() {
                    return -1.0;
                }
                let (lo, hi) = (
                    present[0] as f32 - 1.5,
                    present[present.len() - 1] as f32 - 0.5,
                );
                // pe axa lungă blocul acoperă [-1.5, 1.5]; pe cea scurtă [lo, hi]
                let (m_long, m_short) = if i % 2 == 0 {
                    (1.5 - cx.abs(), (cy - lo).min(hi - cy))
                } else {
                    (1.5 - cy.abs(), (cx - lo).min(hi - cx))
                };
                worst = worst.min(m_long).min(m_short);
            }
            for s in 0..3 {
                if layers[i][s] {
                    let (x, y) = center(i, s);
                    n += 1.0;
                    sx += x;
                    sy += y;
                }
            }
        }
        if worst == f32::MAX {
            1.0
        } else {
            worst
        }
    }

    /// Straturile din care se poate lua.
    fn removable_below(&self) -> usize {
        let t = self.layers.len() - 1;
        let top_full = self.layers[t].iter().all(|&b| b);
        // cel mai înalt strat complet: nu se ating nici el, nici cele de deasupra
        let hc = if top_full { t } else { t.saturating_sub(1) };
        // dacă și stratul de sub cel incomplet e de fapt incomplet (după mutări), tot îl protejăm
        hc
    }

    fn sim(&self, m: &Move) -> (Vec<[bool; 3]>, bool, bool) {
        let mut l = self.layers.clone();
        l[m.layer][m.slot] = false;
        let after_remove = Self::stands(&l);
        let t = l.len() - 1;
        let top_full = l[t].iter().all(|&b| b);
        if top_full {
            l.push([false; 3]);
        }
        let t2 = l.len() - 1;
        l[t2][m.put] = true;
        let after_put = Self::stands(&l);
        (l, after_remove, after_put)
    }

    /// Mutări care nu prăbușesc turnul.
    pub fn safe_moves(&self) -> Vec<Move> {
        self.legal_moves()
            .into_iter()
            .filter(|m| {
                let (_, a, b) = self.sim(m);
                a && b
            })
            .collect()
    }
}

impl Game for Turnul {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.fallen.is_some() {
            return Vec::new();
        }
        let n = self.layers.len();
        let t = n - 1;
        let top_full = self.layers[t].iter().all(|&b| b);
        let hc = self.removable_below();
        let mut v = Vec::new();
        for layer in 0..hc {
            for slot in 0..3 {
                if !self.layers[layer][slot] {
                    continue;
                }
                // sloturile libere pe stratul de sus (după scoaterea unui bloc din altă parte)
                let puts: Vec<usize> = if top_full {
                    vec![0, 1, 2]
                } else {
                    (0..3).filter(|&s| !self.layers[t][s]).collect()
                };
                for put in puts {
                    v.push(Move { layer, slot, put });
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let (l, a, b) = self.sim(m);
        self.last = format!(
            "Jucătorul {}: a luat blocul {}{} și l-a pus pe {}.",
            self.side.idx() + 1,
            m.layer + 1,
            (b'a' + m.slot as u8) as char,
            (b'a' + m.put as u8) as char
        );
        if !(a && b) {
            self.fallen = Some(self.side);
            self.last.push_str(" Turnul se prăbușește!");
        }
        // stratul de sub cel golit complet dispare din calcul prin prăbușire; păstrăm starea
        self.layers = l;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.fallen.map(|p| Outcome::Win(p.other()))
    }

    fn evaluate(&self, p: Player) -> i32 {
        let s = self.safe_moves().len() as i32;
        // cel la mutare vrea multe mutări sigure; celălalt vrea să-i lase puține
        if self.side == p {
            s
        } else {
            -s
        }
    }

    fn render(&self) -> String {
        let mut s = String::from("Turnul Șovăitor (stratul de sus primul; X = bloc, . = gol)\n");
        let m = Self::margin(&self.layers);
        for i in (0..self.layers.len()).rev() {
            let blocks: String = (0..3)
                .map(|k| if self.layers[i][k] { "[XXX]" } else { "[   ]" })
                .collect::<Vec<_>>()
                .join(" ");
            let orient = if i % 2 == 0 { "─" } else { "│" };
            s.push_str(&format!("{:>2} {} {}\n", i + 1, orient, blocks));
        }
        s.push_str(&format!(
            "sloturi: a b c   blocuri: {}   marja de stabilitate: {:.2}\n",
            self.blocks(),
            m
        ));
        let hc = self.removable_below();
        s.push_str(&format!("Poți lua din straturile 1..{}.\n", hc));
        if self.fallen.is_none() {
            s.push_str(&format!(
                "La mutare: jucătorul {} (ex. «ia 3b pune a»)\n",
                self.side.idx() + 1
            ));
        }
        s.push_str(self.last.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!(
            "ia {}{} pune {}",
            m.layer + 1,
            (b'a' + m.slot as u8) as char,
            (b'a' + m.put as u8) as char
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full(n: usize) -> Vec<[bool; 3]> {
        vec![[true; 3]; n]
    }

    #[test]
    fn full_tower_stands_with_positive_margin() {
        assert!(Turnul::stands(&full(12)));
        assert!(Turnul::margin(&full(12)) > 0.0);
    }

    #[test]
    fn two_side_blocks_removed_leaves_middle_stable() {
        let mut l = full(6);
        l[2] = [false, true, false];
        assert!(Turnul::stands(&l));
    }

    #[test]
    fn single_edge_block_topples_a_symmetric_tower() {
        let mut l = full(6);
        l[2] = [true, false, false];
        assert!(!Turnul::stands(&l));
        l[2] = [false, false, true];
        assert!(!Turnul::stands(&l));
    }

    #[test]
    fn empty_layer_collapses() {
        let mut l = full(6);
        l[2] = [false; 3];
        assert!(!Turnul::stands(&l));
    }

    #[test]
    fn edge_block_can_stay_if_center_of_mass_is_over_it() {
        // totul de deasupra cade pe partea „c”: stratul rămâne cu „c” și „b”
        let mut l = full(4);
        l[1] = [false, true, true];
        assert!(Turnul::stands(&l));
        // dar când deasupra e încărcat asimetric spre stânga, scoaterea lui b+c cade
        let mut l2 = full(4);
        l2[3] = [true, false, false];
        l2[1] = [false, false, true];
        assert!(!Turnul::stands(&l2));
    }

    #[test]
    fn removing_a_lower_block_can_topple_via_upper_asymmetry() {
        // sus un singur bloc pe partea „a” (y = −1): centrul de masă de deasupra stratului 3
        // este la y = −0,25, deci doar blocurile care acoperă acel punct țin turnul
        let mut l = full(5);
        l[4] = [true, false, false];
        l[2] = [true, true, false];
        assert!(Turnul::stands(&l));
        l[2] = [false, true, true];
        assert!(Turnul::stands(&l)); // y = −0,25 e încă în [−0,5; 1,5]
        l[2] = [false, false, true];
        assert!(!Turnul::stands(&l));
        l[2] = [true, false, false];
        assert!(!Turnul::stands(&l));
    }

    #[test]
    fn move_generation_respects_top_layers() {
        let g = Turnul::new();
        let m = g.legal_moves();
        // straturile 1..11 (indici 0..11), 3 sloturi, 3 puneri pe un strat nou
        assert_eq!(m.len(), 11 * 3 * 3);
        assert!(m.iter().all(|x| x.layer < 11));
        let mut g2 = Turnul::new();
        g2.apply(&Move {
            layer: 0,
            slot: 0,
            put: 1,
        });
        assert_eq!(g2.layer_count(), 13);
        assert_eq!(g2.blocks(), 36);
        // stratul de sus (13) are un bloc: cel mai înalt complet e 12; se ia din 1..11 (32 de blocuri)
        let m2 = g2.legal_moves();
        assert!(m2.iter().all(|x| x.layer < 11));
        assert!(m2.iter().all(|x| x.put != 1));
        assert_eq!(m2.len(), 32 * 2);
        assert!(!m2.iter().any(|x| x.layer == 0 && x.slot == 0));
    }

    #[test]
    fn blocks_are_conserved_and_games_end_in_collapse() {
        for seed in 1..=30u64 {
            let mut g = Turnul::new();
            let mut rng = polytabletop_core::rng::Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                let pick = m[rng.below(m.len())];
                g.apply(&pick);
                assert_eq!(g.blocks(), 36);
                n += 1;
                assert!(n < 1500);
            }
            // cel care a mutat ultimul a pierdut
            assert!(g.fallen.is_some());
        }
    }

    #[test]
    fn collapse_means_the_mover_loses() {
        let mut g = Turnul::new();
        // golim stratul 1 de două ori ca să-l prăbușim
        g.layers[0] = [true, false, false];
        g.apply(&Move {
            layer: 5,
            slot: 0,
            put: 0,
        });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn ai_avoids_immediate_collapse_when_a_safe_move_exists() {
        let mut g = Turnul::new();
        let mut rng = polytabletop_core::rng::Rng::new(3);
        // jucăm câteva mutări sigure aleatorii ca să avem o poziție mai deschisă
        for _ in 0..25 {
            let s = g.safe_moves();
            if s.is_empty() {
                break;
            }
            let pick = s[rng.below(s.len())];
            g.apply(&pick);
        }
        for step in 0..10 {
            if g.safe_moves().is_empty() {
                break;
            }
            let m = polytabletop_core::ai::best_move(&g, step).unwrap();
            assert!(
                g.safe_moves().contains(&m),
                "AI a ales o mutare care prăbușește"
            );
            g.apply(&m);
        }
    }
}
