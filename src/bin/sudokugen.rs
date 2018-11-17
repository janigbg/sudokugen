extern crate rand;
extern crate sudokugen;
use std::io;
use sudokugen::generator::Generator;
use sudokugen::generator::random_gen::*;
use sudokugen::solver::least_options::LeastOptionsSolver;
use sudokugen::solver::Solver;

fn main() -> () {
    
    let solver = LeastOptionsSolver::new();
    let mut gen = RandGenerator::new(Box::new(solver));
    let puzzle = gen.generate().unwrap();
    
    LeastOptionsSolver::new().solve(&mut puzzle.board.clone()).unwrap();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
