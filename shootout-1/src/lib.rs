use std::marker::PhantomData;

pub struct BigBuf {
    buf: Vec<u8>,
}

impl BigBuf {
    pub fn new(sz: usize) -> Self {
        // Fill with data to ensure that the memory is mapped
        let mut buf = Vec::with_capacity(sz);
        let mut ctr = 128u8;
        for _ in 0..sz {
            ctr = ctr.wrapping_add(1);
            buf.push(ctr);
        }
        Self {
            buf
        }
    }
}

pub struct Bbpv1<'a> {
    sli: &'a [u8]
}

impl<'a> ProdV1 for Bbpv1<'a> {
    fn try_take(&mut self) -> Option<u8> {
        let (f, rest) = self.sli.split_first()?;
        self.sli = rest;
        Some(*f)
    }

    fn try_take_n(&mut self, ct: usize) -> Option<&[u8]> {
        if ct <= self.sli.len() {
            let (st, rest) = self.sli.split_at(ct);
            self.sli = rest;
            Some(st)
        } else {
            None
        }
    }

    fn ctry_take_n<const N: usize>(&mut self) -> Option<&[u8; N]> {
        todo!()
    }
}

pub struct Bbcv1<'a> {
    sli: &'a mut [u8],
}

pub struct Bbcv2<'a> {
    sli: &'a mut [u8],
    idx: usize,
}

pub struct Bbcv3<'a> {
    cursor: *mut u8,
    end: *mut u8,
    _lt: PhantomData<&'a u8>
}

impl BigBuf {
    pub fn as_cons<'a>(&'a mut self) -> Bbcv1<'a> {
        Bbcv1 {
            sli: self.buf.as_mut_slice(),
        }
    }
    pub fn as_cons2<'a>(&'a mut self) -> Bbcv2<'a> {
        Bbcv2 {
            sli: self.buf.as_mut_slice(),
            idx: 0,
        }
    }
    pub fn as_cons3<'a>(&'a mut self) -> Bbcv3<'a> {
        Bbcv3 {
            cursor: self.buf.as_mut_ptr(),
            end: unsafe { self.buf.as_mut_ptr().add(self.buf.len()) },
            _lt: PhantomData,
        }
    }
}

impl<'a> ConsV1 for Bbcv1<'a> {
    fn try_push(&mut self, b: u8) -> Result<(), ()> {
        if self.sli.len() == 0 {
            return Err(())
        }
        let sli = core::mem::take(&mut self.sli);
        sli[0] = b;
        self.sli = &mut sli[1..];
        Ok(())
    }

    fn try_extend(&mut self, b: &[u8]) -> Result<(), ()> {
        let blen = b.len();
        if blen > self.sli.len() {
            Err(())
        } else {
            let sli = core::mem::take(&mut self.sli);
            let (now, later) = sli.split_at_mut(blen);
            now.copy_from_slice(b);
            self.sli = later;
            Ok(())
        }
    }
}

impl<'a> ConsV1 for Bbcv2<'a> {
    fn try_push(&mut self, b: u8) -> Result<(), ()> {
        *self.sli.get_mut(self.idx).ok_or(())? = b;
        self.idx = self.idx.wrapping_add(1);
        Ok(())
    }

    fn try_extend(&mut self, b: &[u8]) -> Result<(), ()> {
        let blen = b.len();
        if (blen + self.idx) > self.sli.len() {
            Err(())
        } else {
            self.sli[self.idx..][..blen].copy_from_slice(b);
            self.idx += blen;
            Ok(())
        }
    }
}

impl<'a> ConsV1 for Bbcv3<'a> {
    #[inline(always)]
    fn try_push(&mut self, b: u8) -> Result<(), ()> {
        if self.cursor == self.end {
            Err(())
        } else {
            unsafe {
                self.cursor.write(b);
                self.cursor = self.cursor.add(1);
            }
            Ok(())
        }
    }

    #[inline(always)]
    fn try_extend(&mut self, b: &[u8]) -> Result<(), ()> {
        let remain = (self.end as usize) - (self.cursor as usize);
        let blen = b.len();
        if blen > remain {
            Err(())
        } else {
            unsafe {
                core::ptr::copy_nonoverlapping(b.as_ptr(), self.cursor, blen);
                self.cursor = self.cursor.add(blen);
            }
            Ok(())
        }
    }

    fn try_extend_with<F>(&mut self, n: usize, f: F) -> Result<(), ()>
    where
        F: FnOnce(&mut [u8])
    {
        let remain = (self.end as usize) - (self.cursor as usize);
        if n > remain {
            Err(())
        } else {
            unsafe {
                f(core::slice::from_raw_parts_mut(self.cursor, n));
                self.cursor = self.cursor.add(n);
            }
            Ok(())
        }
    }
}

pub trait ProdV1 {
    fn try_take(&mut self) -> Option<u8>;
    fn try_take_n(&mut self, ct: usize) -> Option<&[u8]>;
    fn ctry_take_n<const N: usize>(&mut self) -> Option<&[u8; N]>;
}

pub trait ConsV1 {
    fn try_push(&mut self, b: u8) -> Result<(), ()>;
    fn try_extend(&mut self, b: &'_ [u8]) -> Result<(), ()>;
    fn try_extend_with<F>(&mut self, n: usize, f: F) -> Result<(), ()>
    where
        F: FnOnce(&mut [u8])
    {
        todo!()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let mut buf = BigBuf::new(1024);
        let mut cons = buf.as_cons();
        cons.try_push(1).unwrap();
        cons.try_push(2).unwrap();
        cons.try_push(3).unwrap();
        cons.try_push(4).unwrap();

    }
}
