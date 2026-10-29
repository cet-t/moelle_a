use crate::splitmix::SplitMix64;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Squares32 {
    c: u64,
    k: u64,
}

impl From<u32> for Squares32 {
    fn from(value: u32) -> Self {
        let mut sg = SplitMix64::new(value as _);
        Self { c: 0, k: sg.next() }
    }
}

impl trng::Rng for Squares32 {
    type Word = u32;

    fn nextu(&mut self) -> Self::Word {
        let y = self.c.wrapping_mul(self.k);
        let z = y.wrapping_add(self.k);
        let x = {
            let mut x = y.wrapping_mul(y).wrapping_add(y);
            x = x.wrapping_mul(x).wrapping_add(z);
            x = x.rotate_left(32);

            x = x.wrapping_mul(x).wrapping_add(y);
            x = x.rotate_left(32);
            x
        };
        let out = (x.wrapping_mul(x).wrapping_add(z) >> 32) as _;

        self.c = self.c.wrapping_add(1);

        out
    }
}
