use core::array;

use splitmix::SplitMix32;

const N: usize = 25;
const M: usize = 7;
const MAG01: [u32; 2] = [0x0, 0x8ebf_d028];

#[derive(Debug, Clone, Copy)]
pub struct TwistedGfsr {
    seed: [u32; N],
    index: usize,
}

impl From<u64> for TwistedGfsr {
    fn from(value: u64) -> Self {
        let mut sg = SplitMix32::new(value as _);

        Self {
            seed: array::from_fn(|_| sg.next()),
            index: N,
        }
    }
}

impl fil::Rng for TwistedGfsr {
    type Word = u64;

    fn nextu(&mut self) -> Self::Word {
        // twist
        if self.index >= N {
            for k in 0..(N - M) {
                self.seed[k] = self.seed[k + M]
                    ^ self.seed[k].wrapping_shr(1)
                    ^ MAG01[(self.seed[k] & 1) as usize];
            }

            for k in (N - M)..N {
                self.seed[k] = self.seed[k + M - N]
                    ^ self.seed[k].wrapping_shr(1)
                    ^ MAG01[(self.seed[k] & 1) as usize];
            }

            self.index = 0;
        }

        let mut y = self.seed[self.index];
        y ^= y.wrapping_shl(7) & 0x2b5b_2500;
        y ^= y.wrapping_shl(15) & 0xdb8b_0000;
        y ^= y.wrapping_shr(16);

        self.index = self.index.wrapping_add(1);
        u64::from(y)
    }
}
