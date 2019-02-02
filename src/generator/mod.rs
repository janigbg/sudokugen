use super::board::SudokuBoard;
use std::convert::From;
use std::fmt;

pub mod random_gen;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Evil,
}

impl From<u32> for Difficulty {
    fn from(x: u32) -> Self {
        match x {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Hard,
            3 => Difficulty::Evil,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
            Difficulty::Evil => write!(f, "Evil"),
        }
    }
}

pub struct Puzzle {
    pub board: SudokuBoard,
    pub difficulty: Difficulty,
}

/// Sudoku puzzle generator.
pub trait Generator {
    /// Generates sudoku puzzle or an error if generation fails.
    fn run(&mut self) -> Result<Puzzle, String>;
}
