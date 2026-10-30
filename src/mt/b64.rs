use crate::splitmix::SplitMix64;

const N: usize = 312;
const M: usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UPPER_MASK: u64 = 0xFFFFFFFF80000000;
const LOWER_MASK: u64 = 0x7FFFFFFF;

#[derive(Debug)]
pub struct Mt1993764 {
    mt: [u64; N],
    mti: usize,
}

impl From<u64> for Mt1993764 {
    fn from(value: u64) -> Self {
        let mut sg = SplitMix64::new(value);

        let mut mt = [0; N];
        mt[0] = sg.next();

        for i in 1..N {
            let p = mt[i - 1];
            mt[i] = (p ^ p.wrapping_shr(62)).wrapping_mul(6364136223846793005) + i as u64;
        }

        Self { mt, mti: N }
    }
}

impl trng::Rng for Mt1993764 {
    type Word = u64;

    fn nextu(&mut self) -> Self::Word {
        // twist
        if self.mti >= N {
            for i in 0..(N - M) {
                let x = (self.mt[i] & UPPER_MASK) | (self.mt[i + 1] & LOWER_MASK);
                let mut x_a = x.wrapping_shr(1);
                if x & 1 != 0 {
                    x_a ^= MATRIX_A;
                }
                self.mt[i] = self.mt[i + M] ^ x_a;
            }
            for i in (N - M)..(N - 1) {
                let x = (self.mt[i] & UPPER_MASK) | (self.mt[i + 1] & LOWER_MASK);
                let mut x_a = x.wrapping_shr(1);
                if x & 1 != 0 {
                    x_a ^= MATRIX_A;
                }
                self.mt[i] = self.mt[i + M - N] ^ x_a;
            }
            let x = (self.mt[N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
            let mut x_a = x.wrapping_shr(1);
            if x & 1 != 0 {
                x_a ^= MATRIX_A;
            }
            self.mt[N - 1] = self.mt[M - 1] ^ x_a;
            self.mti = 0;
        }

        let mut y = self.mt[self.mti];
        self.mti = self.mti.wrapping_add(1);
        y ^= y.wrapping_shr(29) & 0x5555555555555555;
        y ^= y.wrapping_shl(17) & 0x71D67FFFEDA60000;
        y ^= y.wrapping_shl(37) & 0xFFF7EEE000000000;
        y ^= y.wrapping_shr(43);
        y
    }
}
