use crate::splitmix::SplitMix64;

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn sfmt_recursion(
    a: [u32; 4],
    b: [u32; 4],
    r1: [u32; 4],
    r2: [u32; 4],
    mask: [u32; 4],
    sl1: u32,
    sl2: u32,
    sr1: u32,
    sr2: u32,
) -> [u32; 4] {
    #[inline(always)]
    fn xor4(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
    }

    #[inline(always)]
    fn and4(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        [a[0] & b[0], a[1] & b[1], a[2] & b[2], a[3] & b[3]]
    }

    #[inline(always)]
    fn shl4(a: [u32; 4], shift: u32) -> [u32; 4] {
        [a[0] << shift, a[1] << shift, a[2] << shift, a[3] << shift]
    }

    #[inline(always)]
    fn shr4(a: [u32; 4], shift: u32) -> [u32; 4] {
        [a[0] >> shift, a[1] >> shift, a[2] >> shift, a[3] >> shift]
    }

    #[inline(always)]
    fn pack128(a: [u32; 4]) -> u128 {
        // SFMT treats lane 0 as the least significant word on every target.
        u128::from(a[0])
            | (u128::from(a[1]) << 32)
            | (u128::from(a[2]) << 64)
            | (u128::from(a[3]) << 96)
    }

    #[inline(always)]
    fn unpack128(a: u128) -> [u32; 4] {
        [
            a as u32,
            (a >> 32) as u32,
            (a >> 64) as u32,
            (a >> 96) as u32,
        ]
    }

    #[inline(always)]
    fn lshift128(a: [u32; 4], bytes: u32) -> [u32; 4] {
        unpack128(pack128(a) << (bytes * 8))
    }

    #[inline(always)]
    fn rshift128(a: [u32; 4], bytes: u32) -> [u32; 4] {
        unpack128(pack128(a) >> (bytes * 8))
    }

    xor4(
        xor4(xor4(a, lshift128(a, sl2)), and4(shr4(b, sr1), mask)),
        xor4(rshift128(r1, sr2), shl4(r2, sl1)),
    )
}

#[derive(Debug, Clone, Copy)]
pub struct Sfmt1993764 {
    state: [[u32; 4]; N],
    idx: usize,
}

const N: usize = 156;
const POS1: usize = 122;
const SL1: u32 = 18;
const SR1: u32 = 11;
const MSK: [u32; 4] = [0xdfffffef, 0xddfecb7f, 0xbffaffff, 0xbffffff6];
const PARITY: [u32; 4] = [0x00000001, 0x00000000, 0x00000000, 0x13c9e684];

impl From<u64> for Sfmt1993764 {
    fn from(value: u64) -> Self {
        let mut seedgen = SplitMix64::new(value);

        let mut raw_state = [0u32; N * 4];
        for i in 0..N * 2 {
            let s = seedgen.next();
            raw_state[2 * i] = s as u32;
            raw_state[2 * i + 1] = (s >> 32) as u32;
        }

        let mut state = [[0u32; 4]; N];
        for i in 0..N {
            state[i] = [
                raw_state[4 * i],
                raw_state[4 * i + 1],
                raw_state[4 * i + 2],
                raw_state[4 * i + 3],
            ];
        }

        let mut rng = Self { state, idx: N * 2 };
        rng.period_certification();
        rng
    }
}

impl Sfmt1993764 {
    fn gen_rand_all(&mut self) {
        unsafe {
            let ptr = self.state.as_mut_ptr();
            let mut r1 = *ptr.add(N - 2);
            let mut r2 = *ptr.add(N - 1);

            for i in 0..(N - POS1) {
                let p_i = ptr.add(i);
                let a = *p_i;
                let b = *ptr.add(i + POS1);
                let r = sfmt_recursion(a, b, r1, r2, MSK, SL1, 1, SR1, 1);

                *p_i = r;
                r1 = r2;
                r2 = r;
            }

            for i in (N - POS1)..N {
                let p_i = ptr.add(i);
                let a = *p_i;
                let b = *ptr.add(i + POS1 - N);
                let r = sfmt_recursion(a, b, r1, r2, MSK, SL1, 1, SR1, 1);

                *p_i = r;
                r1 = r2;
                r2 = r;
            }
        }
    }

    fn period_certification(&mut self) {
        let mut inner = 0;
        let psfmt32 =
            unsafe { core::slice::from_raw_parts(self.state.as_ptr() as *const u32, N * 4) };

        for i in 0..4 {
            inner ^= psfmt32[i] & PARITY[i];
        }
        let mut i = 16;
        while i > 0 {
            inner ^= inner.wrapping_shr(i);
            i = i.wrapping_shr(1);
        }
        inner &= 1;

        if inner == 1 {
            return;
        }

        let psfmt32_mut =
            unsafe { core::slice::from_raw_parts_mut(self.state.as_mut_ptr() as *mut u32, N * 4) };

        for i in 0..4 {
            let mut work = 1;
            for _ in 0..32 {
                if (work & PARITY[i]) != 0 {
                    psfmt32_mut[i] ^= work;
                    return;
                }
                work = work.wrapping_shl(1);
            }
        }
    }
}

impl fil::Rng for Sfmt1993764 {
    type Word = u64;

    #[inline]
    fn nextu(&mut self) -> Self::Word {
        if self.idx >= N * 2 {
            self.gen_rand_all();
            self.idx = 0;
        }

        let s: &[u64] = bytemuck::cast_slice(&self.state);
        let val = s[self.idx];
        self.idx = self.idx.wrapping_add(1);
        val
    }
}
