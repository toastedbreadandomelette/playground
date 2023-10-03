use crate::sha::sha256::SHADigest;

pub struct Parser<'a> {
    /// Some
    byte: &'a [u8],
    /// Residue
    residue: Vec<u8>,
    /// Buffer
    buffer: Vec<u32>,
    /// Len
    len: usize,
    /// Current offset
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(byte: &'a [u8]) -> Self {
        Self {
            byte,
            residue: Vec::new(),
            buffer: vec![0; 16],
            len: byte.len(),
            offset: 0,
        }
    }

    #[inline(always)]
    pub fn fetch_transmute_u32_from_residue(
        &mut self,
        size: usize,
    ) -> Option<()> {
        if self.offset + size > self.residue.len() {
            None
        } else {
            let slice = &self.residue[self.offset..self.offset + size]; // let slice_ptr = slice.as_ptr().cast::<u32>();
            self.offset += size;
            self.buffer
                .iter_mut()
                .zip(slice.chunks(4))
                .for_each(|(b, c)| {
                    *b = u32::from_be_bytes(c.try_into().unwrap());
                });
            Some(())
        }
    }

    #[inline]
    pub fn fetch_transmute_u32_from_arr(&mut self, size: usize) -> Option<()> {
        if self.offset + size > self.len {
            // None
            self.residue.extend_from_slice(&self.byte[self.offset..]);
            self.residue.push(0x80);

            let remainder_len = if (self.len & 63) >= 56 {
                128 - (self.len & 63) - 1
            } else {
                64 - (self.len & 63) - 1
            };
            self.residue.extend_from_slice(&vec![0; remainder_len]);

            let (rlen, transmuted_slice) =
                (self.residue.len(), (self.len << 3).to_be_bytes());
            self.residue[rlen - 8..].copy_from_slice(&transmuted_slice);
            self.offset = 0;

            let slice = &self.residue[self.offset..self.offset + size];
            self.offset += size;

            self.buffer
                .iter_mut()
                .zip(slice.chunks(4))
                .for_each(|(b, c)| {
                    *b = u32::from_be_bytes(c.try_into().unwrap());
                });

            Some(())
        } else {
            let slice = &self.byte[self.offset..self.offset + size];
            self.offset += size;
            self.buffer
                .iter_mut()
                .zip(slice.chunks(4))
                .for_each(|(b, c)| {
                    *b = u32::from_be_bytes(c.try_into().unwrap());
                });
            Some(())
        }
    }

    /// Size for SHA224/SHA256
    #[inline]
    pub fn fetch_transmute_u32_and_skip(&mut self, size: usize) -> Option<()> {
        if self.residue.is_empty() {
            self.fetch_transmute_u32_from_arr(size)
        } else {
            self.fetch_transmute_u32_from_residue(size)
        }
    }

    #[inline(always)]
    pub fn fetch_transmute_u64_and_skip(
        &mut self,
        size: usize,
    ) -> Option<&'a [u64]> {
        if self.offset + size > self.len {
            None
        } else {
            unsafe {
                let slice = &self.byte[self.offset..self.offset + size];
                let slice_ptr = slice.as_ptr().cast::<u64>();
                self.offset += size;
                Some(core::slice::from_raw_parts(slice_ptr, size / 8))
            }
        }
    }
}

pub fn parse_file(file: &str) {
    let fd = std::fs::OpenOptions::new().read(true).open(file).unwrap();

    let mmaped = unsafe {
        memmap2::MmapOptions::new()
            .populate()
            .stack()
            .map(&fd)
            .unwrap()
    };

    let mut parser = Parser::new(&mmaped);
    let mut shadigest = SHADigest::new(crate::sha::sha256::SHAType::SHA256);

    while parser.fetch_transmute_u32_and_skip(64).is_some() {
        shadigest.digest_sha_multi(&parser.buffer);
    }
    let ans = shadigest.hash_to_string();
    println!("{}", ans);
}
