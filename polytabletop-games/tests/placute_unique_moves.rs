use polytabletop_core::rng::Rng;
use polytabletop_core::GameObj;
use polytabletop_games::placute::Placute;
#[test]
fn placute_unique_moves() {
    for seed in 1..=400u64 {
        let mut g: Box<dyn GameObj> = Box::new(Placute::with_seed(seed));
        let mut rng = Rng::new(seed * 7 + 1);
        for _ in 0..3000 {
            if g.outcome().is_some() {
                break;
            }
            let mv = g.legal_moves();
            let mut s = mv.clone();
            s.sort();
            s.dedup();
            if s.len() != mv.len() {
                let mut d = mv.clone();
                d.sort();
                let dup: Vec<_> = d
                    .windows(2)
                    .filter(|w| w[0] == w[1])
                    .map(|w| w[0].clone())
                    .collect();
                panic!("seed {seed}: dup {:?}\n{}", dup, g.render());
            }
            g.play(&mv[rng.below(mv.len())]).unwrap();
        }
    }
}
