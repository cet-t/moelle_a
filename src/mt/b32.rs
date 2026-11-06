use splitmix::SplitMix32;

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908B0DF;
const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7FFFFFFF;

#[derive(Debug, Clone, Copy)]
pub struct Mt19937 {
    mt: [u32; N],
    mti: usize,
}

impl From<u32> for Mt19937 {
    fn from(value: u32) -> Self {
        let mut sg = SplitMix32::new(value);

        let mut mt = [0; N];
        mt[0] = sg.next();

        for i in 1..N {
            let p = mt[i - 1];
            mt[i] = (p ^ p.wrapping_shr(30)).wrapping_mul(1812433253) + i as u32;
        }

        Self { mt, mti: N }
    }
}

impl moelle::Rng for Mt19937 {
    type Word = u32;

    fn next(&mut self) -> Self::Word {
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
        y ^= y.wrapping_shr(11);
        y ^= y.wrapping_shl(7) & 0x9D2C5680;
        y ^= y.wrapping_shl(15) & 0xEFC60000;
        y ^= y.wrapping_shr(18);
        y
    }
}
