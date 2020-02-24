extern crate bit_vec; //https://docs.rs/bit-vec/0.6.1/bit_vec/
use bit_vec::BitVec;
use core::fmt;

#[derive(Clone)]
pub struct Library {
    pub books: BitVec,
    pub sign_up: u32,
    pub speed: u32,
    pub index: usize,
}

impl Library {
    pub fn new(nr_books: usize, index: u32, line0: Vec<u32>, line1: Vec<u32>) -> self::Library {
        // create BitVec and set bit
        let mut books = BitVec::from_elem(nr_books, false);
        for bit in line1 { books.set(bit as usize, true) };

        self::Library{books, index: index as usize, sign_up: line0[1], speed: line0[2],}
    }
}

impl fmt::Debug for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lib#{} {{ sign_up: {}, speed: {}, size: {} }}", self.index, self.sign_up, self.speed, self.books.count_ones())
    }
}

pub trait Count {
    fn count_ones(&self) -> u64;
    fn to_vec(self) -> Vec<usize>;
}

impl Count for BitVec {
    fn count_ones(&self) -> u64 {
        self.iter()
            .filter(|x| *x)
            .count() as u64
    }

    fn to_vec(self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter(|(_, bit)| *bit)
            .map(|(pos, _)| pos)
            .collect::<Vec<usize>>()
    }
}