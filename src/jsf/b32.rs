use splitmix::SplitMix32;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Jsf32 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl From<u32> for Jsf32 {
    fn from(value: u32) -> Self {
        let mut sg = SplitMix32::new(value);

        Self {
            a: 0xf1ea5eed,
            b: sg.next(),
            c: sg.next(),
            d: sg.next(),
        }
    }
}

impl moelle::Rng for Jsf32 {
    type Word = u32;

    fn next(&mut self) -> Self::Word {
        let e = self.a.wrapping_sub(self.b.rotate_left(27));
        self.a = self.b ^ self.c.rotate_left(17);
        self.b = self.c.wrapping_add(self.d);
        self.c = self.d.wrapping_add(e);
        self.d = self.a.wrapping_add(e);
        self.d
    }
}
