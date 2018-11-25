pub mod least_options;

use super::board::{Placement, SudokuBoard};

#[derive(Debug)]
pub struct Solution {
    pub placements: Vec<Placement>,
    pub branches: u32,
}

/// Sudoku puzzle solver.
pub trait Solver {
    /// Verifies that a `SudokuBoard` represents a valid Sudoku puzzle.
    fn verify(&mut self, board: &SudokuBoard) -> (u32, bool);

    /// Solves `SudokuBoard` and returns the solution,
    /// or returns `String` if not solvable.
    fn solve(&mut self, board: &mut SudokuBoard) -> Result<Solution, String>;

    /// Tries to solve `SudokuBoard` within `max_iterations`,
    /// or returns `String` if not solvable within specified number of iterations.
    fn try_solve(
        &mut self,
        board: &mut SudokuBoard,
        max_iterations: Option<u32>,
    ) -> Result<Solution, String>;
}
