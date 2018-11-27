extern crate rand;
extern crate sudokugen;
use std::io;
use sudokugen::generator::{Difficulty, Generator};
use sudokugen::generator::random_gen::*;
use sudokugen::solver::least_options::LeastOptionsSolver;
use sudokugen::solver::Solver;

fn main() -> () {
    // Provide a Solver to generate puzzle
    let solver = LeastOptionsSolver::new();
    // Create puzzle generator of specified difficulty
    let mut generator = RandomSudoku::new(solver)
        .difficulty(Difficulty::Evil);
    // Create a puzzle
    let puzzle = generator.run().unwrap();
    // Calculate number of clues
    let num_clues = puzzle.board.clues
        .iter()
        .filter(|&c| *c == true)
        .count();
    // Print out number of clues and the board
    println!("\r\n# of clues: {}\r\n", num_clues);
    println!("Verified board:\r\n{}", puzzle.board);
    // Solve the board, why??
    LeastOptionsSolver::new()
        .solve(&mut puzzle.board.clone())
        .unwrap();
    // Read Enter from keyboard input
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
