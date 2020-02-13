use std::fmt::{ Display, Formatter, Result as FmtResult };

#[derive(Clone, Debug)]
pub struct BitVec {
    internal: Vec<usize>,
    index: usize,
}

impl BitVec {
    const fn size_of_ptr() -> usize {
        std::mem::size_of::<usize>() * 8
    }

    pub fn new() -> Self {
        Self {
            internal: Vec::new(),
            index: Self::size_of_ptr(),
        }
    }

    pub fn len(&self) -> usize {
        (self.internal.len() - 1) * Self::size_of_ptr() + self.index
    }

    pub fn capacity(&self) -> usize {
        self.internal.capacity() * Self::size_of_ptr()
    }

    pub fn with_capacity(size: usize) -> Self {
        let size = (size + Self::size_of_ptr() - 1) / Self::size_of_ptr();
        Self {
            internal: Vec::with_capacity(size),
            index: Self::size_of_ptr(),
        }
    }

    pub fn push(&mut self, value: bool) {
        if self.index == Self::size_of_ptr() {
            self.internal.push(value as usize);
            self.index = 1;
        } else {
            let last = self.internal.last_mut().unwrap();
            *last |= (value as usize) << self.index;
            self.index += 1;
        }
    }

    pub fn pop(&mut self) -> Option<bool> {
        if self.index == 1 {
            let pop = self.internal.pop()?;
            Some(pop == 1)
        } else {
            self.index -= 1;
            let mask = 1 << self.index;
            let result = (*(self.internal.last().unwrap()) & mask) > 0;
            let mask = (!0usize) >> (Self::size_of_ptr() - self.index);
            *self.internal.last_mut().unwrap() &= mask;
            Some(result)
        }
    }

    pub fn insert(&mut self, index: usize, value: bool) {
        let internal_idx = index / Self::size_of_ptr();
        let usize_idx = index % Self::size_of_ptr();
        
        if index > self.len() {
            panic!("index is larger than len")
        };

        let mask = (!0usize) >> (Self::size_of_ptr() - usize_idx);
        
        let remain = self.internal[internal_idx] & mask;
        let (mut shl, mut leftover) = 
            (self.internal[internal_idx] & !mask).overflowing_shl(1);
        shl |= (value as usize) << usize_idx;
        self.internal[internal_idx] = remain | shl;

        for iidx in (internal_idx + 1)..self.internal.len() {
            let curr = self.internal[iidx];
            let first = leftover as usize;
            let (s, l) = curr.overflowing_shl(1);
            shl = s; // Blame the lack of destructing assignment for this!
            leftover = l;
            self.internal[iidx] = shl | first;
        }
        
        self.index += 1;
    }

    pub fn remove(&mut self, index: usize) -> bool {
        let internal_idx = index / Self::size_of_ptr();
        let usize_idx = index % Self::size_of_ptr();

        if index > self.len() {
            panic!("index is larger than len")
        };

        let mask = (!0usize) >> (Self::size_of_ptr() - usize_idx);
        let remain = self.internal[internal_idx] & mask;
        let result = (self.internal[internal_idx] & !mask) >> usize_idx;
        let shifted = (self.internal[internal_idx] & !mask & !1) >> 1;
        let mut leftover = 0;

        for iidx in ((internal_idx + 1)..self.internal.len()).rev() {
            let (mut shr, left) = self.internal[iidx].overflowing_shr(1);
            shr |= leftover << (Self::size_of_ptr() - 1);
            leftover = left as usize;
            self.internal[iidx] = shr;
        }

        let first = remain | shifted | (leftover << (Self::size_of_ptr()));
        self.internal[internal_idx] = first;
        self.index -= 1;

        return (result & 1) == 1
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        let internal_idx = index / Self::size_of_ptr();
        let usize_idx = index % Self::size_of_ptr();
        
        if index > self.len() {
            return None
        };

        Some(((self.internal[internal_idx] >> usize_idx) & 1) == 1)
    }

    pub fn internal(&self) -> &Vec<usize> {
        &self.internal
    }

    pub fn internal_mut(&mut self) -> &mut Vec<usize> {
        &mut self.internal
    }
}

impl Display for BitVec {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "[")?;
        for x in 0..self.len() - 1 {
            write!(fmt, "{}, ", self.get(x).unwrap())?
        }
        write!(fmt, "{}]\n", self.get(self.len() - 1).unwrap())?;

        Ok(())
    }
}
