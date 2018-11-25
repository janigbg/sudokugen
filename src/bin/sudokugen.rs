extern crate rand;
extern crate sudokugen;
use std::io;
use sudokugen::generator::{Difficulty, Generator};
use sudokugen::generator::random_gen::*;
use sudokugen::solver::least_options::LeastOptionsSolver;
use sudokugen::solver::Solver;

fn main() -> () {
    
    let solver = LeastOptionsSolver::new();
    let mut gen = RandGenSudoku::new(Box::new(solver))
        .difficulty(Difficulty::Evil);
    let puzzle = gen.generate().unwrap();

    let num_clues = puzzle.board.clues.iter().filter(|&c| *c == true).count();
    println!("\r\n# of clues: {}\r\n", num_clues);
    println!("Verified board:\r\n{}", puzzle.board);
    
    LeastOptionsSolver::new().solve(&mut puzzle.board.clone()).unwrap();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
