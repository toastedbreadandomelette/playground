const K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const H256: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
    0x1f83d9ab, 0x5be0cd19,
];

const H224: [u32; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511,
    0x64f98fa7, 0xbefa4fa4,
];

#[derive(PartialEq, Clone, Copy)]
pub enum SHAType {
    SHA224,
    SHA256,
}

pub struct SHADigest {
    h: [u32; 8],
    w: [u32; 64],
    sha_type: SHAType,
}

impl SHADigest {
    #[inline]
    pub fn new(sha_type: SHAType) -> Self {
        Self {
            h: if sha_type == SHAType::SHA224 {
                H224
            } else {
                H256
            },
            w: [0; 64],
            sha_type,
        }
    }

    #[inline(always)]
    pub fn hash_to_string(&self) -> String {
        let take_amount = if let SHAType::SHA224 = self.sha_type {
            7
        } else {
            8
        };
        self.h
            .iter()
            .take(take_amount)
            .map(|h| format!("{:04x?}", h))
            .intersperse("".to_owned())
            .collect()
    }

    #[inline(always)]
    fn rotate_right(value: u32, rotate_by: usize) -> u32 {
        (value >> rotate_by) | (value << (32 - rotate_by))
    }

    pub fn digest_sha_multi(&mut self, arr: &[u32]) {
        self.w[0..16].copy_from_slice(arr);

        for j in (16..64).step_by(2) {
            let s0_first = Self::rotate_right(self.w[j - 15], 7)
                ^ Self::rotate_right(self.w[j - 15], 18)
                ^ (self.w[j - 15] >> 3);

            let s0_second = Self::rotate_right(self.w[j - 14], 7)
                ^ Self::rotate_right(self.w[j - 14], 18)
                ^ (self.w[j - 14] >> 3);

            let s1_first = Self::rotate_right(self.w[j - 2], 17)
                ^ Self::rotate_right(self.w[j - 2], 19)
                ^ (self.w[j - 2] >> 10);

            let s1_second = Self::rotate_right(self.w[j - 1], 17)
                ^ Self::rotate_right(self.w[j - 1], 19)
                ^ (self.w[j - 1] >> 10);

            self.w[j] = self.w[j - 16]
                .wrapping_add(s0_first)
                .wrapping_add(self.w[j - 7])
                .wrapping_add(s1_first);

            self.w[j + 1] = self.w[j - 15]
                .wrapping_add(s0_second)
                .wrapping_add(self.w[j - 6])
                .wrapping_add(s1_second);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.h;

        self.w.iter().zip(K256.iter()).for_each(|(w, k)| {
            let ss1 = Self::rotate_right(e, 6)
                ^ Self::rotate_right(e, 11)
                ^ Self::rotate_right(e, 25);

            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(ss1)
                .wrapping_add(ch)
                .wrapping_add(*k)
                .wrapping_add(*w);

            let temp2 = (Self::rotate_right(a, 2)
                ^ Self::rotate_right(a, 13)
                ^ Self::rotate_right(a, 22))
            .wrapping_add((a & b) ^ (a & c) ^ (b & c));

            (a, b, c, d, e, f, g, h) = (
                temp1.wrapping_add(temp2),
                a,
                b,
                c,
                d.wrapping_add(temp1),
                e,
                f,
                g,
            );
        });

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }

    #[allow(dead_code)]
    pub fn digest_sha(&mut self, arr: &[u32]) {
        self.w[0..16].copy_from_slice(arr);

        for j in 16..64 {
            let s0 = Self::rotate_right(self.w[j - 15], 7)
                ^ Self::rotate_right(self.w[j - 15], 18)
                ^ (self.w[j - 15] >> 3);
            let s1 = Self::rotate_right(self.w[j - 2], 17)
                ^ Self::rotate_right(self.w[j - 2], 19)
                ^ (self.w[j - 2] >> 10);

            self.w[j] = self.w[j - 16]
                .wrapping_add(s0)
                .wrapping_add(self.w[j - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.h;

        self.w.iter().zip(K256.iter()).for_each(|(w, k)| {
            let ss1 = Self::rotate_right(e, 6)
                ^ Self::rotate_right(e, 11)
                ^ Self::rotate_right(e, 25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(ss1)
                .wrapping_add(ch)
                .wrapping_add(*k)
                .wrapping_add(*w);
            let temp2 = (Self::rotate_right(a, 2)
                ^ Self::rotate_right(a, 13)
                ^ Self::rotate_right(a, 22))
            .wrapping_add((a & b) ^ (a & c) ^ (b & c));

            (a, b, c, d, e, f, g, h) = (
                temp1.wrapping_add(temp2),
                a,
                b,
                c,
                d.wrapping_add(temp1),
                e,
                f,
                g,
            );
        });

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }
}
