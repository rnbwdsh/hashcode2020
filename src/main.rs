#![feature(try_trait)]
mod library;
mod gamestate;
mod io;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use library::Library;
use gamestate::Gamestate;
//use rand::Rng;

fn main() {
    const NR_THREADS: i32 = 1;
    const NR_FORKS: i32 = 1;
    //const FILENAMES: [&str; 1] = ["e_so_many_books.txt"];
    const FILENAMES: [&str; 6] = ["a_example.txt", "b_read_on.txt", "c_incunabula.txt", "d_tough_choices.txt", "e_so_many_books.txt", "f_libraries_of_the_world.txt"];
    for filename in FILENAMES.iter() {
        println!("{}", filename);

        let (nr_books, nr_days, scores, libs) = io::read_file(filename).unwrap();
        println!("nr_books: {}, nr_libs: {}, nr_days: {}, scores: {:?}", nr_books, libs.len(), nr_days, scores);
        //for lib in &libs {println!("{:?}", lib)};

        let mut states = VecDeque::new();
        for _ in 0..NR_THREADS {
            states.push_back(Gamestate::new(nr_days, nr_books, libs.clone()));
        }

        let states = Arc::new(Mutex::new(states));
        let scores = Arc::new(scores);
        let highscore = Arc::new(Mutex::new(0u64));

        loop {
            let mut threads = Vec::new();
            for _ in 0..NR_THREADS {
                let states = states.clone();
                let scores = scores.clone();
                let highscore = highscore.clone();
                threads.push(thread::spawn(move || {
                    let ostate = states.lock().unwrap().pop_back();
                    if ostate.is_some() {
                        for _ in 0..NR_FORKS { // generate NR_FORKS sub-proccesses
                            let state = ostate.clone().unwrap();
                            if state.time > 0 && !state.libs.is_empty() {
                                let new_state = state.random();
                                states.lock().unwrap().push_back(new_state.clone());
                                //println!("{}", state.time);
                            } else {
                                let score = state.score(&scores);
                                let mut highscore = highscore.lock().unwrap();
                                if score.gt(&highscore) {
                                    println!("{}", score);
                                    *highscore = score;
                                    io::write_solution(score, state.used_libs, state.used_books, filename);
                                }
                                break;
                }}}}));
            }
            for thread in threads { thread.join().expect("Join fail"); }
            if states.lock().unwrap().is_empty() { break; }
        }
    }
}


