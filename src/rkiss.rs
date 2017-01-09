pub struct RKISS {
    a: u64,
    b: u64,
    c: u64,
    d: u64
}

impl RKISS {
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

    pub fn rand(&mut self) -> u64 {
        let e = self.a - rotate_l(self.b, 7);
        self.a = self.b ^ rotate_l(self.c, 13);
        self.b = self.c + rotate_l(self.d, 37);
        self.c = self.d + e;
        self.d = e + self.a;
        self.d
    }

    pub fn magic_rand(&mut self, s: usize) -> u64 {
        rotate_l(
            rotate_l(
                self.rand(),
                (s >> 0) & 0x3F
            ) & self.rand(),
            (s >> 6) & 0x3F
        ) & self.rand()
    }
}

fn rotate_l(x: u64, k: usize) -> u64 {
    (x << k) | (x >> (64 - k))
}