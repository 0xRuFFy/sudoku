// use std::{fs, io, io::Write as _I, sync::mpsc, thread, time::Instant};

use std::time::Instant;

mod backtracking_bf; // backtracking brute force
mod backtracking_ds; // backtracking dynamic selection
mod board;
mod solver;

fn main() {
    let mut sudoku = board::Sudoku::new();
    sudoku.load_board_from_str(
        "000000010400000000020000000000050407008000300001090000300400200050100000000806000",
    );
    println!("{}", sudoku);

    // let mut solver = backtracking_bf::BBFS::new();
    let mut solver = backtracking_ds::BDSS::new();
    let start = Instant::now();
    sudoku.solve(&mut solver);
    println!("Time elapsed: {:?}", start.elapsed());
    println!("{}", sudoku);

    // let mut sudoku = board::Sudoku::new();
    // // let mut solver = backtracking_bf::BBFS::new();
    // let mut solver = backtracking_ds::BDSS::new();
    // let file = std::fs::read_to_string("data/easy50.txt").expect("Unable to read file");
    // let lines = file.lines();
    // let count = lines.clone().count();
    // println!("Running {} benchmarks.", count);
    // let mut total = 0;
    // use std::io::{self, Write};
    // let start = Instant::now();
    // for line in lines {
    //     sudoku.load_board_from_str(line);
    //     println!("{}", sudoku);
    //     sudoku.solve(&mut solver);
    //     println!("{}", sudoku);
    //     total += 1;
    //     print!("{:<6} / {:<6}\r", total, count);
    //     io::stdout().flush().unwrap();
    // }
    // println!("Time elapsed: {:?}", start.elapsed());


    // let file = std::fs::read_to_string("data/bench.txt").expect("Unable to read file");
    // let mut lines = file.lines();
    // let count = lines.next().map(|l| l.parse::<usize>().unwrap()).unwrap();

    // let mut i = 0;
    // let (tx, rx) = std::sync::mpsc::channel();
    // // let count = 10;
    // let threads: Vec<_> = (0..count)
    //     .map(|_| tx.clone())
    //     .map(|txc| {
    //         let mut sudoku = board::Sudoku::new();
    //         sudoku.load_board_from_str(lines.next().unwrap());
    //         let mut solver = backtracking_ds::BDSS::new();
    //         i += 1;
    //         std::thread::spawn(move || {
    //             sudoku.solve(&mut solver);
    //             txc.send(i).unwrap();
    //         })
    //     })
    //     .collect();

    // let mut total = 0;

    // use std::io::{self, Write};
    // for _ in rx {
    //     total += 1;
    //     print!("{:<6} / {:<6}\r", total, count);
    //     io::stdout().flush().unwrap();
    //     if total == count {
    //         break;
    //     }
    // }

    // for thread in threads {
    //     thread.join().unwrap();
    // }
}
