extern crate test;

use super::super::board::{Placement, SudokuBoard};
use super::super::group::{add, sub, Group};
use super::{Solution, Solver};

pub struct LeastOptionsSolver {
    solution: Vec<(Placement, Vec<Placement>)>,
    max_iterations: Option<u32>,
    iterations: u32,
}

impl Solver for LeastOptionsSolver {
    fn verify(&mut self, board: &SudokuBoard) -> bool {
        let mut clone = board.clone();

        self.solution.clear();
        self.max_iterations = None;

        match self.find_solution(&mut clone) {
            Ok(_) => self.find_solution(&mut clone).is_err() && self.solution.is_empty(),
            Err(_) => false,
        }
    }

    fn solve(&mut self, board: &mut SudokuBoard) -> Result<Solution, String> {
        self.try_solve(board, None)
    }

    fn try_solve(
        &mut self,
        board: &mut SudokuBoard,
        max_iterations: Option<u32>,
    ) -> Result<Solution, String> {
        self.solution.clear();
        self.max_iterations = max_iterations;

        self.find_solution(board)?;

        let (result, _): (Vec<Placement>, Vec<_>) = self.solution.drain(..).unzip();
        Ok(Solution {
            placements: result,
            iterations: self.iterations,
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
                    if board.values[index] > 0 {
                        continue;
                    }

                    let row = index / 9;
                    let col = index % 9;
                    let box_num = (index / 27) * 3 + (index % 9) / 3;

                    let found_rows =
                        opts.row_options[row]
                            .iter()
                            .enumerate()
                            .find(|(val, &amount)| {
                                amount == options && opts.placements[index][*val] == 1
                            });

                    if let Some((val, _)) = found_rows {
                        self.place_value(
                            board,
                            (row, col, (val + 1) as u8),
                            (0..9)
                                .into_iter()
                                .filter(|c| *c != col && opts.placements[row * 9 + c][val] == 1)
                                .map(|c| (row, c, (val + 1) as u8))
                                .collect(),
                            &mut opts,
                        )?;

                        // println!("Found ROW, ({}, {}) = {}, options = {}, alts.len() = {}",
                        //     row, col, (val+1), options, alts.len());

                        found_placements = true;
                        match options {
                            1 => continue,
                            _ => break,
                        };
                    }

                    let found_cols =
                        opts.col_options[col]
                            .iter()
                            .enumerate()
                            .find(|(val, &amount)| {
                                amount == options && opts.placements[index][*val] == 1
                            });

                    if let Some((val, _)) = found_cols {
                        self.place_value(
                            board,
                            (row, col, (val + 1) as u8),
                            (0..9)
                                .into_iter()
                                .map(|r| (r, opts.placements[r * 9 + col]))
                                .filter(|(i, r)| *i != row && r[val] == 1)
                                .map(|(i, _)| (i, col, (val + 1) as u8))
                                .collect(),
                            &mut opts,
                        )?;

                        // println!("Found COL, ({}, {}) = {}, options = {}, alts.len() = {}",
                        //     row, col, (val+1), options, alts.len());

                        found_placements = true;
                        match options {
                            1 => continue,
                            _ => break,
                        };
                    }

                    let found_boxes =
                        opts.box_options[box_num]
                            .iter()
                            .enumerate()
                            .find(|(val, &amount)| {
                                amount == options && opts.placements[row * 9 + col][*val] == 1
                            });

                    if let Some((val, _)) = found_boxes {
                        self.place_value(
                            board,
                            (row, col, (val + 1) as u8),
                            (0..9)
                                .into_iter()
                                .map(|box_index| {
                                    LeastOptionsSolver::from_box_coords(box_num, box_index)
                                })
                                .filter(|(r, c)| {
                                    (*r != row || *c != col)
                                        && opts.placements[*r * 9 + *c][val] == 1
                                })
                                .map(|(r, c)| (r, c, (val + 1) as u8))
                                .collect(),
                            &mut opts,
                        )?;

                        // println!("Found BOX, ({}, {}) = {}, options = {}, alts.len() = {}",
                        //     row, col, (val+1), options, alts.len());

                        found_placements = true;
                        match options {
                            1 => continue,
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
                        Some(((row, col, _), mut alts)) => {
                            board.place((row, col, 0))?;
                            opts.on_value_changed(board, row, col);

                            if alts.len() > 0 {
                                let new_val = alts.pop().unwrap();
                                self.place_value(board, new_val, alts, &mut opts)?;
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

        println!("Iterations:{}", self.iterations);

        Ok(())
    }

    fn place_value(
        &mut self,
        board: &mut SudokuBoard,
        val: Placement,
        alts: Vec<Placement>,
        opts: &mut AvailableOptions,
    ) -> Result<(), String> {
        self.solution.push((val, alts));
        board.place(val)?;
        self.inc_placement_counter()?;
        opts.on_value_changed(board, val.0, val.1);

        Ok(())
    }

    fn inc_placement_counter(&mut self) -> Result<(), String> {
        self.iterations += 1;
        match self.max_iterations {
            Some(max) if self.iterations > max => {
                return Err(String::from(format!("Max placements attempted: {}", max)))
            }
            _ => Ok(()),
        }
    }

    fn from_box_coords(the_box: usize, box_index: usize) -> (usize, usize) {
        (
            (the_box / 3) * 3 + box_index / 3,
            (the_box % 3) * 3 + box_index % 3,
        )
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
                let the_box = (row / 3) * 3 + (col / 3);
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

    pub fn on_value_changed(&mut self, board: &SudokuBoard, row: usize, col: usize) -> () {
        let the_box = (row / 3) * 3 + (col / 3);
        for (i, pos) in self.placements.iter_mut().enumerate().filter(|(index, _)| {
            index / 9 == row || index % 9 == col || ((index / 27) * 3 + (index % 9) / 3) == the_box
        }) {
            let r = i / 9;
            let c = i % 9;
            let b = (i / 27) * 3 + (i % 9) / 3;

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
    use super::Solver;

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
        assert_eq!(true, result);
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
