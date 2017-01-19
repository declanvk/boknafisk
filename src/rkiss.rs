pub static MAGIC_BOOSTERS: [usize; 8] = [3101, 552, 3555, 926, 834, 26, 2131, 1117];

pub struct RKISS {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl RKISS {
    #[inline]
    pub fn new(rounds: usize) -> RKISS {
        let mut rgen = RKISS {
            a: 0xF1EA5EED,
            b: 0xD4E12C77,
            c: 0xD4E12C77,
            d: 0xD4E12C77,
        };

        for _ in 0..rounds {
            rgen.rand();
        }

        rgen
    }

    #[inline]
    pub fn rand(&mut self) -> u64 {
        let e = self.a.wrapping_sub(rotate_l(self.b, 7));
        self.a = self.b ^ rotate_l(self.c, 13);
        self.b = self.c.wrapping_add(rotate_l(self.d, 37));
        self.c = self.d.wrapping_add(e);
        self.d = e.wrapping_add(self.a);
        self.d
    }

    #[inline]
    pub fn magic_rand(&mut self, s: usize) -> u64 {
        rotate_l(rotate_l(self.rand(), (s >> 0) & 0x3F) & self.rand(),
                 (s >> 6) & 0x3F) & self.rand()
    }
}

#[inline]
fn rotate_l(x: u64, k: usize) -> u64 {
    (x.wrapping_shl(k as u32)) | (x.wrapping_shr(64u32.wrapping_sub(k as u32)))
}
