extern crate bit_vec; //https://docs.rs/bit-vec/0.6.1/bit_vec/
use bit_vec::BitVec;
use crate::Library;
use crate::library::Count;
use std::fs;
use std::option::NoneError;

pub fn read_file(filename: &str) -> Result<(u32, u32, Vec<u32>, Vec<Library>), NoneError> {
    // read and parse file
    let contents = fs::read_to_string( format!("data/{}",filename))
        .unwrap_or(
            String::from(""));
    let mut lines = contents
        .split("\n")
        .map(|line| line.split(" ")
            .flat_map(str::parse::<u32>)
            .collect::<Vec<u32>>());

    // read line0 and line1
    let line = lines.next()?;
    let (nr_books, nr_libs, nr_days) = (line[0], line[1], line[2]);
    let scores = lines.next()?;

    // read libs
    let mut libs = Vec::new();
    for lib_id in 0..nr_libs {
        let lib = Library::new(nr_books as usize, lib_id as u32, lines.next()?, lines.next()?);
        libs.push(lib);
    }

    Ok((nr_books, nr_days, scores, libs))
}

pub fn write_solution(score: u64, used_libs: Vec<usize>, used_books: Vec<BitVec>, filename: &str) {
    fs::create_dir(format!("solutions/{}", filename)).ok(); // ignore if creating a directory fails
    let path = format!("solutions/{}/{}.txt", filename, score.to_string());

    let nr_books = used_libs.len().to_string();
    let rest = used_libs.iter().zip(used_books.iter()).map(| (lib, books)|
        format!("{} {}\n{}", lib, books.count_ones(), books
            .clone()
            .to_vec()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
        )).collect::<Vec<String>>()
        .join("\n");
    fs::write(path, format!("{}\n{}", nr_books, rest))
        .expect("Write failed");
}