use super::super::board::SudokuBoard;
use super::super::solver::{Solver, Verification};
use super::{Difficulty, Generator, Puzzle};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;

/// Number of attempts of creating board with valid
/// set of clues
static CREATE_CLUE_ATTEMPTS: u32 = 5;

/// Generator for creating random sudoku puzzle.
///
/// Allows specifying random seed and `Difficulty`.
pub struct RandomSudoku {
    solver: Box<Solver>,
    random_seed: bool,
    seed: [u8; 16],
    difficulty: Difficulty,
    max_iterations: u32,
}

impl Generator for RandomSudoku {
    fn run(&mut self) -> Result<Puzzle, String> {
        // Set up random generator
        let mut rng = self.random_generator();
        // Randomize clues and solve board
        let mut board = self.solve_with_random_clues(&mut rng);
        // Find valid puzzle, if possible
        self.find_valid_puzzle(&mut board, &mut rng)?;
        // Return puzzle
        Ok(Puzzle {
            board,
            difficulty: self.difficulty,
        })
    }
}

impl RandomSudoku {
    /// Creates new `RandomSudoku` with default settings
    /// and specified `Solver`.
    ///
    /// Can be further customized with builder methods
    /// `seed` and `difficulty`.
    pub fn new<T>(solver: T) -> RandomSudoku
    where
        T: Solver + 'static,
    {
        // Default settings with specified solver
        RandomSudoku {
            solver: Box::new(solver),
            random_seed: true,
            seed: [0; 16],
            difficulty: Difficulty::Easy,
            max_iterations: 1000,
        }
    }

    /// Sets random seed to use for puzzle generation.
    ///
    /// Using same seed (and same `Difficulty`) will generate
    /// the exact same puzzle.
    pub fn seed(mut self, seed: u32) -> RandomSudoku {
        // Build array from u32
        let seed_bytes: [u8; 4] = [
            (seed >> 24) as u8,
            (seed >> 16) as u8,
            (seed >> 8) as u8,
            seed as u8,
        ];
        self.random_seed = false;
        // Duplicate u32 seed to fill up seed array
        // TODO: Improve, this might impact randomness of generator
        for (i, pos) in self.seed.iter_mut().enumerate() {
            *pos = seed_bytes[i % 4];
        }
        self
    }

    /// Sets target `Difficulty` of puzzle.
    pub fn difficulty(mut self, diff: Difficulty) -> RandomSudoku {
        self.difficulty = diff;
        self
    }
}

impl RandomSudoku {
    fn random_generator(&mut self) -> Pcg64Mcg {
        if self.random_seed {
            Pcg64Mcg::from_entropy().fill_bytes(&mut self.seed);
        }
        Pcg64Mcg::from_seed(self.seed)
    }

    fn solve_with_random_clues(&mut self, rng: &mut Pcg64Mcg) -> SudokuBoard {
        let mut board: SudokuBoard;
        loop {
            board = get_board_with_clues(rng);

            let result = self.solver.try_solve(&mut board, Some(self.max_iterations));

            if result.is_ok() {
                break;
            }
        }

        board
    }

    fn find_valid_puzzle(
        &mut self,
        board: &mut SudokuBoard,
        rng: &mut Pcg64Mcg,
    ) -> Result<(), String> {
        let orig_values = board.values;

        for _ in 0..CREATE_CLUE_ATTEMPTS {
            let mut removal_sequence: Vec<usize> = (0..81).collect();
            removal_sequence.shuffle(rng);

            let mut count = 0;
            let mut removed_cells = 0;
            let mut diff = Difficulty::Easy;

            while count < 81 {
                let index = removal_sequence[count];
                if board.values[index] > 0 && !board.clues[index] {
                    count += 1;

                    if self.try_removing_value(board, index, &mut removed_cells, &mut diff) {
                        break;
                    }
                }
            }

            if diff >= self.difficulty {
                convert_to_clues(board);
                return Ok(());
            } else {
                println!("Need to retry puzzle generation.");
                board.values = orig_values;
            }
        }

        Err(format!(
            "Could not generate puzzle of difficulty {}",
            self.difficulty
        ))
    }

    fn try_removing_value(
        &mut self,
        board: &mut SudokuBoard,
        index: usize,
        removed_cells: &mut u32,
        diff: &mut Difficulty,
    ) -> bool {
        let num = board.values[index];
        let (row, col) = (index / 9, index % 9);
        board.place((row, col, 0)).unwrap();
        if let Verification::ValidWithBranches(branches) = self.solver.verify(&board) {
            *removed_cells += 1;
            let prev_diff = *diff;
            *diff = get_difficulty(*removed_cells, branches);
            if *diff > self.difficulty {
                *diff = prev_diff;
                board.place((row, col, num)).unwrap();
                return true;
            }
        } else {
            board.place((row, col, num)).unwrap();
        }
        false
    }
}

fn get_difficulty(removed: u32, branches: u32) -> Difficulty {
    let clues = 81 - removed;
    match (clues, branches) {
        (_, b) if b > 1 => Difficulty::Evil,
        (c, b) if b > 0 || c < 28 => Difficulty::Hard,
        (c, _) if c < 35 => Difficulty::Medium,
        _ => Difficulty::Easy,
    }
}

fn convert_to_clues(board: &mut SudokuBoard) {
    for (i, clue) in board.clues.iter_mut().enumerate() {
        if board.values[i] > 0 {
            *clue = true;
        }
    }
}

fn get_board_with_clues(rng: &mut Pcg64Mcg) -> SudokuBoard {
    let mut board = SudokuBoard::with_clues(&[]);
    let mut add_sequence: Vec<usize> = (0..81).collect();

    add_sequence.shuffle(rng);

    let mut index = 0;
    for _iteration in 0..25 {
        let mut row: usize;
        let mut col: usize;
        while index < 81 {
            row = add_sequence[index] / 9;
            col = add_sequence[index] % 9;
            let placements = board.get_allowed_vals(row, col);
            index += 1;
            match (0u8..9u8)
                .filter(|&val| placements[val as usize] == 1)
                .collect::<Vec<u8>>()
            {
                ref v if !v.is_empty() => (),
                values => {
                    board
                        .place((row, col, *values.choose(rng).unwrap() + 1))
                        .unwrap();
                    break;
                }
            }
        }
    }

    board
}
