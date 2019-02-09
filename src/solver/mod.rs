pub mod least_options;

use super::board::{Placement, SudokuBoard};

/// Solution for Sudoku puzzle.
#[derive(Debug)]
pub struct Solution {
    pub placements: Vec<Placement>,
    pub branches: u32,
}

/// Represents Sudoku puzzle solver verification result.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Verification {
    /// Sudoku puzzle that is not valid, i.e. puzzle
    /// that has no solution or many solutions.
    NotValid,
    /// Valid sudoku puzzle with specified number of
    /// branches in solution.
    ValidWithBranches(u32),
}

/// Sudoku puzzle solver.
pub trait Solver {
    /// Verifies that a `SudokuBoard` represents a valid Sudoku puzzle.
    /// Returns number of branches if valid.
    /// 
    /// A valid puzzle is a puzzle that has one and only one solution.
    fn verify(&mut self, board: &SudokuBoard) -> Verification;

    /// Solves `SudokuBoard` and returns the solution,
    /// or returns `Err(String)` if not solvable.
    fn solve(&mut self, board: &mut SudokuBoard) -> Result<Solution, String>;

    /// Tries to solve `SudokuBoard` within `max_iterations`,
    /// or returns `Err(String)` if not solvable within specified number of iterations.
    fn try_solve(
        &mut self,
        board: &mut SudokuBoard,
        max_iterations: Option<u32>,
    ) -> Result<Solution, String>;
}
