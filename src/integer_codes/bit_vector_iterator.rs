use crate::introduction::bit_vector::BitVector;
use crate::introduction::util::lsb;

pub struct BitVectorIterator<'a> {
    bv: &'a BitVector,
    pos: usize,
    buf: u64,
    avail: usize,
}

impl<'a> BitVectorIterator<'a> {
    pub fn new(bv: &'a BitVector, pos: usize) -> Self {
        Self {
            bv,
            pos,
            buf: 0,
            avail: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn at(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn fill_buf(&mut self) {
        self.buf = self.bv.get_word64(self.pos);
        self.avail = 64;
    }

    /// return 1 byte assuming position is aligned to a 8-bit boundary
    pub fn take_one_byte(&mut self) -> u64 {
        assert!(self.pos % 8 == 0);
        if self.avail == 0 {
            self.fill_buf();
        }
        let val = self.buf & 255;
        self.buf >>= 8;
        self.avail -= 8;
        self.pos += 8;
        val
    }

    /// return the next l bits from the current position and advance by l bits
    pub fn take(&mut self, l: usize) -> u64 {
        assert!(l <= 64);
        if self.avail < l {
            self.fill_buf();
        }
        let mut val = self.buf;
        if l != 64 {
            val &= (1u64 << l).wrapping_sub(1);
            self.buf >>= l;
        }
        self.avail -= l;
        self.pos += l;
        val
    }

    /// skip all zeros from the current position and return the number of skipped zeros
    pub fn skip_zeros(&mut self) -> usize {
        let mut zeros = 0;
        while self.buf == 0 {
            self.pos += self.avail;
            zeros += self.avail;
            self.fill_buf();
        }

        let l = lsb(self.buf);
        self.buf >>= l;
        self.buf >>= 1;
        self.avail -= l + 1;
        self.pos += l + 1;
        zeros + l
    }
}
