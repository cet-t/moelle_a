use crate::splitmix::SplitMix64;

const STRIDE: u64 = 0x60bee2bee120fc15;
const A: u128 = 0xa3b195354a39b70d;
const B: u128 = 0x1b03738712fad5c9;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WyHash64 {
    s: u64,
}

impl From<u64> for WyHash64 {
    fn from(value: u64) -> Self {
        let mut sg = SplitMix64::new(value);

        Self { s: sg.next() }
    }
}

impl fil::Rng for WyHash64 {
    type Word = u64;

    fn nextu(&mut self) -> Self::Word {
        self.s = self.s.wrapping_add(STRIDE);

        let mut t = (self.s as u128).wrapping_mul(A);
        let m1 = (t.wrapping_shr(64) ^ t) as u64;
        t = (m1 as u128).wrapping_mul(B);
        let m2 = t.wrapping_shr(64) ^ t;
        m2 as _
    }
}
