extern crate bit_vec; //https://docs.rs/bit-vec/0.6.1/bit_vec/
extern crate rand;
use crate::Library;
use crate::library::Count;
use bit_vec::BitVec;
use rand::Rng;
use std::borrow::Borrow;


#[derive(Debug, Clone)]
pub struct Gamestate {
    pub time: u32,
    pub books: BitVec,
    pub libs: Vec<Library>,
    pub used_libs: Vec<usize>,
    pub used_books: Vec<BitVec>,
}

impl Gamestate {
    pub fn new(time: u32, nr_books: u32, libs: Vec<Library>) -> self::Gamestate {
        Gamestate{time, books: BitVec::from_elem(nr_books as usize, false), libs,
            used_libs: Vec::new(), used_books: Vec::new(), }
    }

    fn select_lib(&self, libs: &mut Vec<Library>) -> Library {
        let mut rng = rand::thread_rng();
        let idx = rng.gen::<usize>() % self.libs.len();
        libs.remove(idx)
    }

    fn select_books(&self, lib: &mut Library) -> BitVec {
        let mut rng = rand::thread_rng();
        let mut lib_book_ids = lib.books.clone().to_vec();
        let time = self.time - lib.sign_up;

        rng.shuffle(&mut lib_book_ids);

        let lib_book_ids = &lib_book_ids[..(time * lib.speed) as usize];
        //println!("ids {:?}", lib_book_ids); // get trimmed ids
        let mut books = self.books.clone();
        lib_book_ids
            .iter()
            .for_each(|i|
                books.set(*i as usize, true));
        books
    }

    pub fn random(&self) -> Gamestate {
        let mut lib = self.select_lib(&mut self.libs.clone());

        if self.time > lib.sign_up {
            let time = self.time - lib.sign_up;

            let mut books = lib.books.clone();
            books.difference(&self.books);

            let remtime: u64 = time as u64 * lib.speed as u64;
            if remtime >= books.count_ones() {
                books.union(&self.books);
            } else {
                self.select_books(&mut lib);
            }

            let libs = self.libs.clone();
            let mut used_books = self.used_books.clone();
            used_books.push(books.clone());
            let mut used_libs = self.used_libs.clone();
            used_libs.push(lib.index);

            Gamestate{time, books, libs, used_libs, used_books,}
        } else {
            Gamestate{time: 0, books: self.books.clone(), libs: self.libs.clone(), used_libs: self.used_libs.clone(), used_books: self.used_books.clone(),}
        }



    }

    pub fn score(&self, score: &Vec<u32>) -> u64 {
        let books = self.books.borrow();
        books.iter().zip(score.iter()).map(|(x, y)| x as u64 * *y as u64).sum()
    }
}
