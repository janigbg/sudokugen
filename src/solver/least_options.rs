extern crate test;

use super::super::board::{Placement, SudokuBoard};
use super::super::group::{add, sub, Group};
use super::{Solution, Solver, Verification};

#[cfg_attr(rustfmt, rustfmt_skip)]
static BOX_BY_INDEX: [usize; 81] =
    [
        0, 0, 0, 1, 1, 1, 2, 2, 2,
        0, 0, 0, 1, 1, 1, 2, 2, 2,
        0, 0, 0, 1, 1, 1, 2, 2, 2,
        3, 3, 3, 4, 4, 4, 5, 5, 5,
        3, 3, 3, 4, 4, 4, 5, 5, 5,
        3, 3, 3, 4, 4, 4, 5, 5, 5,
        6, 6, 6, 7, 7, 7, 8, 8, 8,
        6, 6, 6, 7, 7, 7, 8, 8, 8,
        6, 6, 6, 7, 7, 7, 8, 8, 8,
    ];

#[cfg_attr(rustfmt, rustfmt_skip)]
static BOX_BY_COORDS: [[usize; 9]; 9] =
    [
        [ 0, 0, 0, 1, 1, 1, 2, 2, 2, ],
        [ 0, 0, 0, 1, 1, 1, 2, 2, 2, ],
        [ 0, 0, 0, 1, 1, 1, 2, 2, 2, ],
        [ 3, 3, 3, 4, 4, 4, 5, 5, 5, ],
        [ 3, 3, 3, 4, 4, 4, 5, 5, 5, ],
        [ 3, 3, 3, 4, 4, 4, 5, 5, 5, ],
        [ 6, 6, 6, 7, 7, 7, 8, 8, 8, ],
        [ 6, 6, 6, 7, 7, 7, 8, 8, 8, ],
        [ 6, 6, 6, 7, 7, 7, 8, 8, 8, ],
    ];

#[cfg_attr(rustfmt, rustfmt_skip)]
static BOX_TO_COORDS: [[(usize, usize); 9]; 9] =
    [
        [ (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2), ],
        [ (0, 3), (0, 4), (0, 5), (1, 3), (1, 4), (0, 5), (2, 3), (2, 4), (2, 5), ],
        [ (0, 6), (0, 7), (0, 8), (1, 6), (1, 7), (1, 8), (2, 6), (2, 7), (2, 8), ],
        [ (3, 0), (3, 1), (3, 2), (4, 0), (4, 1), (4, 2), (5, 0), (5, 1), (5, 2), ],
        [ (3, 3), (3, 4), (3, 5), (4, 3), (4, 4), (4, 5), (5, 3), (5, 4), (5, 5), ],
        [ (3, 6), (3, 7), (3, 8), (4, 6), (4, 7), (4, 8), (5, 6), (5, 7), (5, 8), ],
        [ (6, 0), (6, 1), (6, 2), (7, 0), (7, 1), (7, 2), (8, 0), (8, 1), (8, 2), ],
        [ (6, 3), (6, 4), (6, 5), (7, 3), (7, 4), (7, 5), (8, 3), (8, 4), (8, 5), ],
        [ (6, 6), (6, 7), (6, 8), (7, 6), (7, 7), (7, 8), (8, 6), (8, 7), (8, 8), ],
    ];

struct SolutionStep {
    placement: Placement,
    alts: Vec<Placement>,
    branches: u32,
}

/// Sudoku solver using backtracking least options strategy.
///
/// # Remarks
///
/// Least options strategy means that the solver will find all
/// placement options that cannot be placed anywhere else on
/// a row, column or box. When no such options exist, the solver
/// will find placements options that can be placed in either
/// of two locations on a row, column or box. Then three, then
/// four and so on.
///
/// When no placement options can be found the solver backtracks
/// to the last branch (more than one option was available) with
/// any remaining untried options and chooses the next one.
#[derive(Default)]
pub struct LeastOptionsSolver {
    solution: Vec<SolutionStep>,
    max_iterations: Option<u32>,
    iterations: u32,
}

impl Solver for LeastOptionsSolver {
    fn verify(&mut self, board: &SudokuBoard) -> Verification {
        let mut clone = board.clone();

        self.solution.clear();
        self.max_iterations = None;

        match self.find_solution(&mut clone) {
            Ok(_) => {
                let branches = self.branches();
                if self.find_solution(&mut clone).is_err() && self.solution.is_empty() {
                    Verification::ValidWithBranches(branches)
                } else {
                    Verification::NotValid
                }
            }
            Err(_) => Verification::NotValid,
        }
    }

    fn solve(&mut self, board: &SudokuBoard) -> Result<Solution, String> {
        self.try_solve(board, None)
    }

    fn try_solve(
        &mut self,
        board: &SudokuBoard,
        max_iterations: Option<u32>,
    ) -> Result<Solution, String> {
        self.solution.clear();
        self.max_iterations = max_iterations;
        let mut solve_board = board.clone();
        self.find_solution(&mut solve_board)?;

        let (result, branches): (Vec<Placement>, Vec<u32>) = self
            .solution
            .drain(..)
            .map(|step| (step.placement, step.branches))
            .unzip();
        Ok(Solution {
            board: solve_board,
            placements: result,
            branches: branches.iter().sum(),
        })
    }
}

impl LeastOptionsSolver {
    pub fn new() -> LeastOptionsSolver {
        LeastOptionsSolver {
            solution: Vec::with_capacity(81),
            max_iterations: None,
            iterations: 0,
        }
    }

    fn branches(&self) -> u32 {
        self.solution.iter().map(|step| step.branches).sum()
    }

    fn find_solution(&mut self, board: &mut SudokuBoard) -> Result<(), String> {
        self.iterations = 0;
        let backtrack = !self.solution.is_empty();

        if !backtrack && !board.is_valid() {
            return Err(String::from("Cannot solve invalid board"));
        }

        // Pre-calculate available placements for all positions
        // Pre-calculate number of options for all groups and values
        let mut opts = AvailableOptions::calculate_options(&board);

        while !board.is_filled() || (backtrack && self.iterations == 0) {
            let mut found_placements = false;

            for options in 1..10 {
                for index in 0..81 {
                    // Already contains value, ignore
                    if board.values[index] > 0 {
                        continue;
                    }

                    let row = index / 9;
                    let col = index % 9;

                    // -----
                    // ROWS
                    // -----

                    let found_rows = LeastOptionsSolver::find_option(
                        index,
                        opts.row_options[row],
                        options,
                        &opts.placements,
                    );

                    if let Some(val) = found_rows {
                        self.place_value(
                            board,
                            (row, col, (val + 1) as u8),
                            LeastOptionsSolver::find_row_alts(row, col, val, &opts.placements),
                            u32::from(options - 1),
                            &mut opts,
                        )?;

                        found_placements = true;
                        match options {
                            // Perform all trivial placements
                            1 => continue,
                            // For non-trivial placements (branching),
                            // redo available options calculations
                            _ => break,
                        };
                    }

                    // -----
                    // COLS
                    // -----

                    let found_cols = LeastOptionsSolver::find_option(
                        index,
                        opts.col_options[col],
                        options,
                        &opts.placements,
                    );

                    if let Some(val) = found_cols {
                        self.place_value(
                            board,
                            (row, col, (val + 1) as u8),
                            LeastOptionsSolver::find_col_alts(row, col, val, &opts.placements),
                            u32::from(options - 1),
                            &mut opts,
                        )?;

                        found_placements = true;
                        match options {
                            // Perform all trivial placements
                            1 => continue,
                            // For non-trivial placements (branching),
                            // redo available options calculations
                            _ => break,
                        };
                    }

                    // -----
                    // BOXES
                    // -----

                    let found_boxes = LeastOptionsSolver::find_option(
                        index,
                        opts.box_options[BOX_BY_INDEX[index]],
                        options,
                        &opts.placements,
                    );

                    if let Some(val) = found_boxes {
                        self.place_value(
                            board,
                            (row, col, (val + 1) as u8),
                            LeastOptionsSolver::find_box_alts(
                                row,
                                col,
                                BOX_BY_INDEX[index],
                                val,
                                &opts.placements,
                            ),
                            u32::from(options - 1),
                            &mut opts,
                        )?;

                        found_placements = true;
                        match options {
                            // Perform all trivial placements
                            1 => continue,
                            // For non-trivial placements (branching),
                            // redo available options calculations
                            _ => break,
                        };
                    }
                }

                if found_placements {
                    break;
                }
            }

            if !board.is_valid() || !found_placements {
                loop {
                    let mut found_alt = false;
                    match self.solution.pop() {
                        None => return Err(String::from("No solution found")),
                        Some(mut step) => {
                            let (row, col, _) = step.placement;
                            board.place((row, col, 0))?;
                            opts.on_value_changed(board, row, col);

                            if !step.alts.is_empty() {
                                let new_val = step.alts.pop().unwrap();
                                self.place_value(
                                    board,
                                    new_val,
                                    step.alts,
                                    step.branches,
                                    &mut opts,
                                )?;
                                found_alt = true;
                            }
                        }
                    };

                    if found_alt && board.is_valid() {
                        break;
                    }
                }
            }
        }

        trace!(
            "find_solution: Valid: {}, Iterations: {}, branches: {}",
            board.is_valid(),
            self.iterations,
            self.branches()
        );

        Ok(())
    }

    /// Finds alternative placements for value on same row.
    fn find_row_alts(row: usize, col: usize, val: usize, opts: &[Group; 81]) -> Vec<Placement> {
        (0..9)
            .filter(|c| *c != col && opts[row * 9 + c][val] == 1)
            .map(|c| (row, c, (val + 1) as u8))
            .collect()
    }

    /// Finds alternative placements for value on same column.
    fn find_col_alts(row: usize, col: usize, val: usize, opts: &[Group; 81]) -> Vec<Placement> {
        (0..9)
            .map(|r| (r, opts[r * 9 + col]))
            .filter(|(i, r)| *i != row && r[val] == 1)
            .map(|(i, _)| (i, col, (val + 1) as u8))
            .collect()
    }

    /// Finds alternative placements for value in same box.
    fn find_box_alts(
        row: usize,
        col: usize,
        box_num: usize,
        val: usize,
        opts: &[Group; 81],
    ) -> Vec<Placement> {
        (0..9)
            .map(|box_index| BOX_TO_COORDS[box_num][box_index])
            .filter(|(r, c)| (*r != row || *c != col) && opts[*r * 9 + *c][val] == 1)
            .map(|(r, c)| (r, c, (val + 1) as u8))
            .collect()
    }

    /// Finds number of placement options for value in a group (row, column or box).
    ///
    /// Returns `None` if no placement options.
    fn find_option(
        index: usize,
        group: Group,
        num_opts: u8,
        available_opts: &[Group; 81],
    ) -> Option<usize> {
        group
            .iter()
            .enumerate()
            .find(|(val, &amount)| amount == num_opts && available_opts[index][*val] == 1)
            .map(|pair| pair.0)
    }

    fn place_value(
        &mut self,
        board: &mut SudokuBoard,
        val: Placement,
        alts: Vec<Placement>,
        branches: u32,
        opts: &mut AvailableOptions,
    ) -> Result<(), String> {
        self.solution.push(SolutionStep {
            placement: val,
            alts,
            branches,
        });
        board.place(val)?;
        self.inc_placement_counter()?;
        opts.on_value_changed(board, val.0, val.1);

        Ok(())
    }

    fn inc_placement_counter(&mut self) -> Result<(), String> {
        self.iterations += 1;
        match self.max_iterations {
            Some(max) if self.iterations > max => Err(format!("Max placements attempted: {}", max)),
            _ => Ok(()),
        }
    }
}

struct AvailableOptions {
    placements: [Group; 81],
    row_options: [Group; 9],
    col_options: [Group; 9],
    box_options: [Group; 9],
}

impl AvailableOptions {
    pub fn calculate_options(board: &SudokuBoard) -> AvailableOptions {
        let mut result = AvailableOptions {
            placements: [[0; 9]; 81],
            row_options: [[0; 9]; 9],
            col_options: [[0; 9]; 9],
            box_options: [[0; 9]; 9],
        };

        for row in 0..9 {
            for col in 0..9 {
                let the_box = BOX_BY_COORDS[row][col];
                result.placements[row * 9 + col] = board.get_allowed_vals(row, col);

                result.row_options[row] =
                    add(result.row_options[row], &result.placements[row * 9 + col]);

                result.col_options[col] =
                    add(result.col_options[col], &result.placements[row * 9 + col]);

                result.box_options[the_box] = add(
                    result.box_options[the_box],
                    &result.placements[row * 9 + col],
                );
            }
        }

        result
    }

    pub fn on_value_changed(&mut self, board: &SudokuBoard, row: usize, col: usize) {
        let the_box = BOX_BY_COORDS[row][col];
        for (i, pos) in self.placements.iter_mut().enumerate().filter(|(index, _)| {
            index / 9 == row || index % 9 == col || BOX_BY_INDEX[*index] == the_box
        }) {
            let r = i / 9;
            let c = i % 9;
            let b = BOX_BY_INDEX[i];

            self.row_options[r] = sub(self.row_options[r], pos);
            self.col_options[c] = sub(self.col_options[c], pos);
            self.box_options[b] = sub(self.box_options[b], pos);

            *pos = board.get_allowed_vals(r, c);

            self.row_options[r] = add(self.row_options[r], pos);
            self.col_options[c] = add(self.col_options[c], pos);
            self.box_options[b] = add(self.box_options[b], pos);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::board::{Placement, SudokuBoard};
    use super::test::Bencher;
    use super::LeastOptionsSolver;
    use super::{Solver, Verification};

    static SUPER_HARD: [Placement; 25] = [
        (0, 3, 3),
        (0, 4, 7),
        (0, 5, 6),
        (0, 8, 8),
        (1, 5, 1),
        (1, 8, 6),
        (2, 1, 2),
        (3, 0, 7),
        (3, 2, 4),
        (3, 6, 9),
        (3, 7, 3),
        (4, 0, 1),
        (4, 2, 8),
        (4, 5, 2),
        (5, 0, 2),
        (5, 1, 6),
        (5, 4, 9),
        (5, 8, 4),
        (6, 5, 3),
        (6, 7, 7),
        (6, 8, 1),
        (7, 6, 8),
        (7, 7, 6),
        (8, 3, 9),
        (8, 4, 6),
    ];

    static EVIL: [Placement; 28] = [
        (0, 0, 8),
        (0, 4, 1),
        (0, 8, 9),
        (1, 1, 4),
        (1, 4, 8),
        (1, 7, 2),
        (2, 3, 2),
        (2, 5, 5),
        (3, 2, 8),
        (3, 4, 2),
        (3, 6, 7),
        (4, 0, 5),
        (4, 1, 6),
        (4, 3, 7),
        (4, 5, 1),
        (4, 7, 9),
        (4, 8, 8),
        (5, 2, 1),
        (5, 4, 9),
        (5, 6, 5),
        (6, 3, 9),
        (6, 5, 2),
        (7, 1, 8),
        (7, 4, 4),
        (7, 7, 6),
        (8, 0, 7),
        (8, 4, 5),
        (8, 8, 4),
    ];

    static REFLECTION_SYMMETRY: [Placement; 18] = [
        (1, 1, 9),
        (1, 4, 1),
        (1, 7, 3),
        (2, 2, 6),
        (2, 4, 2),
        (2, 6, 7),
        (3, 3, 3),
        (3, 5, 4),
        (4, 0, 2),
        (4, 1, 1),
        (4, 7, 9),
        (4, 8, 8),
        (6, 2, 2),
        (6, 3, 5),
        (6, 5, 6),
        (6, 6, 4),
        (7, 1, 8),
        (7, 7, 1),
    ];

    static AGAINST_BRUTE_FORCE: [Placement; 17] = [
        (1, 5, 3),
        (1, 7, 8),
        (1, 8, 5),
        (2, 2, 1),
        (2, 4, 2),
        (3, 3, 5),
        (3, 5, 7),
        (4, 2, 4),
        (4, 6, 1),
        (5, 1, 9),
        (6, 0, 5),
        (6, 7, 7),
        (6, 8, 3),
        (7, 2, 2),
        (7, 4, 1),
        (8, 4, 4),
        (8, 8, 9),
    ];

    #[test]
    fn verify_super_hard() {
        let board = SudokuBoard::with_clues(&REFLECTION_SYMMETRY);

        let result = LeastOptionsSolver::new().verify(&board);
        assert_eq!(Verification::ValidWithBranches(7), result);
    }

    #[test]
    fn solve_super_hard() {
        let mut board = SudokuBoard::with_clues(&SUPER_HARD);

        LeastOptionsSolver::new()
            .solve(&mut board)
            .expect("Expected success");
    }

    #[test]
    fn find_solution_fails_for_board_with_invalid_clues() {
        let mut board = SudokuBoard::with_clues(&SUPER_HARD);
        board.values[4] = 3;

        LeastOptionsSolver::new()
            .find_solution(&mut board)
            .expect_err("Expected error");
    }

    #[test]
    fn solve_evil() {
        let mut board = SudokuBoard::with_clues(&EVIL);

        let _result = LeastOptionsSolver::new()
            .solve(&mut board)
            .expect("Expected success");
    }

    #[test]
    fn solve_reflection_symmetry() {
        let mut board = SudokuBoard::with_clues(&REFLECTION_SYMMETRY);

        let _result = LeastOptionsSolver::new()
            .solve(&mut board)
            .expect("Expected success");
    }

    #[test]
    fn solve_against_brute_force() {
        let mut board = SudokuBoard::with_clues(&AGAINST_BRUTE_FORCE);

        let _result = LeastOptionsSolver::new()
            .solve(&mut board)
            .expect("Expected success");
    }

    #[bench]
    fn bench_verify_super_hard(b: &mut Bencher) {
        b.iter(|| verify_super_hard());
    }

    #[bench]
    fn bench_super_hard(b: &mut Bencher) {
        b.iter(|| solve_super_hard());
    }

    #[bench]
    fn bench_evil(b: &mut Bencher) {
        b.iter(|| solve_evil());
    }

    #[bench]
    fn bench_reflection_symmetry(b: &mut Bencher) {
        b.iter(|| solve_reflection_symmetry());
    }

    #[bench]
    fn bench_against_brute_force(b: &mut Bencher) {
        b.iter(|| solve_against_brute_force());
    }
}
