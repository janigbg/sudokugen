extern crate rand;
extern crate sudokugen;
use std::env;
use std::io;
#[macro_use]
extern crate log;
extern crate env_logger;
use sudokugen::generator::random_gen::*;
use sudokugen::generator::{Difficulty, Generator};
use sudokugen::solver::least_options::LeastOptionsSolver;
use sudokugen::solver::Solver;

fn generate_puzzle() -> Result<(), Box<std::error::Error>> {
    // Provide a Solver to generate puzzle
    let solver = LeastOptionsSolver::new();
    // Create puzzle generator of specified difficulty
    let mut generator = RandomSudoku::new(solver).difficulty(Difficulty::Evil);
    // Create a puzzle
    let puzzle = generator.run()?;
    // Calculate number of clues
    let num_clues = puzzle.board.clues.iter().filter(|&c| *c).count();
    // Print out number of clues and the board
    println!("\n# of clues: {}\n", num_clues);
    println!("Verified board:\n{}", puzzle.board);

    // Create puzzle as string of numbers, e.g. "0100400..."
    let board_values = puzzle
        .board
        .values
        .iter()
        .fold(String::new(), |s, &val| format!("{}{:01}", s, val));
    // Print out puzzle as numbers
    println!("Puzzle: {}", board_values);

    // Solve the board, why??
    LeastOptionsSolver::new().solve(&mut puzzle.board.clone())?;
    // Read Enter from keyboard input
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    Ok(())
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    if let Err(e) = generate_puzzle() {
        error!("Error during puzzle generation: {}", e);
    }
}
