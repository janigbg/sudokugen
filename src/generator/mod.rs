use super::board::{SudokuBoard};
use std::convert::From;

pub mod random_gen;

#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Evil,
}

impl From<u64> for Difficulty {
    fn from(x: u64) -> Self {
        match x {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Hard,
            3 => Difficulty::Evil,
            _ => panic!(),
        }
    }
}

pub struct Puzzle {
    pub board: SudokuBoard,
    pub difficulty: Difficulty
}

/// Sudoku puzzle generator.
pub trait Generator {
    /// Generates sudoku puzzle or an error if generation fails.
    fn generate(&mut self) -> Result<Puzzle, String>;
}
