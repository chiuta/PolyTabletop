//! Gul bara (Orientul Mijlociu, Golful Persic) — variantă de table fără lovituri, cu mișcare paralelă.
//! Motorul este `tavli` cu varianta `Gul bara`; vezi regulile acolo.
//!
//! Ambii jucători au 15 piese pornind din colțuri opuse și merg în același sens; un punct ocupat de
//! adversar e blocat (nu se lovește). Cu dubluri: primele trei aruncări ale partidei dau 4 mutări;
//! apoi un dublu `d-d` dă câte 4 mutări cu d, d+1, …, 6. Scoaterea începe când toate piesele sunt acasă.

use crate::tavli::{Tavli, Variant};

pub struct GulBara;

impl GulBara {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Tavli {
        Tavli::with_variant(
            Variant::GulBara,
            polytabletop_core::rng::Rng::from_clock().next_u64(),
        )
    }

    pub fn with_seed(seed: u64) -> Tavli {
        Tavli::with_variant(Variant::GulBara, seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    #[test]
    fn gul_bara_starts_with_a_roll_and_moves() {
        let g = GulBara::with_seed(9);
        assert_eq!(g.variant(), Some(Variant::GulBara));
        assert!(!GameObj::legal_moves(&g).is_empty());
    }
}
