//! Small xorshift64* generator. No external dependencies.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static SEEDED: AtomicBool = AtomicBool::new(false);
static SEED: AtomicU64 = AtomicU64::new(0);
static COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Never let the state be zero.
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    /// Process-wide seed override. With `Some(n)`, every `from_clock()` call
    /// returns a deterministic stream derived from `n` and an incrementing
    /// counter (reset to 0 here), so a whole game (deals, dice) is reproducible.
    /// `None` restores clock seeding.
    pub fn set_global_seed(seed: Option<u64>) {
        COUNTER.store(0, Ordering::SeqCst);
        match seed {
            Some(n) => {
                SEED.store(n, Ordering::SeqCst);
                SEEDED.store(true, Ordering::SeqCst);
            }
            None => SEEDED.store(false, Ordering::SeqCst),
        }
    }

    /// Seed taken from the system clock (for interactive play), or from the
    /// global seed override when set.
    pub fn from_clock() -> Self {
        if SEEDED.load(Ordering::SeqCst) {
            let c = COUNTER.fetch_add(1, Ordering::SeqCst);
            let mut z = SEED
                .load(Ordering::SeqCst)
                .wrapping_add(c.wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            return Rng::new(z ^ (z >> 31));
        }
        let n = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x1234_5678);
        Rng::new(n)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in `0..n`. `n` must be > 0.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }

    /// Uniform in `1..=6`.
    pub fn d6(&mut self) -> u8 {
        (self.below(6) + 1) as u8
    }
}
