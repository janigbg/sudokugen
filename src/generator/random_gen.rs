use super::super::board::{SudokuBoard};
use super::super::solver::Solver;
use super::{Difficulty, Generator, Puzzle};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;

pub struct RandGenSudoku {
    solver: Box<Solver>,
    random_seed: bool,
    seed: [u8; 16],
    difficulty: Difficulty,
    max_iterations: u32
}

impl Generator for RandGenSudoku {
    fn generate(&mut self) -> Result<Puzzle, String> {

        let mut rng = self.random_generator();
        let mut board = self.get_solved_board(&mut rng);

        self.get_valid_puzzle(&mut board, &mut rng);

        Ok(Puzzle{
            board,
            difficulty: self.difficulty
        })
    }
}

impl RandGenSudoku {
    pub fn new(solver: Box<Solver>) -> RandGenSudoku {
        RandGenSudoku {
            solver,
            random_seed: true,
            seed: [0; 16],
            difficulty: Difficulty::Easy,
            max_iterations: 1000,
        }
    }

    fn random_generator(&mut self) -> Pcg64Mcg {
        if self.random_seed {
            Pcg64Mcg::from_entropy().fill_bytes(&mut self.seed);
        }
        Pcg64Mcg::from_seed(self.seed)
    }

    fn get_solved_board(&mut self, rng: &mut Pcg64Mcg) -> SudokuBoard {
        
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

    fn get_valid_puzzle(&mut self, board: &mut SudokuBoard, rng: &mut Pcg64Mcg) -> () {
        
        let mut removal_sequence: Vec<usize> = (0..81).collect();
        removal_sequence.shuffle(rng);

        let mut count = 0;
        let mut _removed_cells = 0;

        while count < 81 {
            let index = removal_sequence[count];
            if board.values[index] > 0 && !board.clues[index] {
                let num = board.values[index];
                let (row, col) = (index / 9, index % 9);
                board.place((row, col, 0)).unwrap();
                count += 1;
                if self.solver.verify(&board) {
                    _removed_cells += 1;
                } else {
                    board.place((row, col, num)).unwrap();
                }
            }
        }

        convert_to_clues(board);
    }

    pub fn seed(mut self, seed: u32) -> RandGenSudoku {
        let seed_bytes: [u8; 4] = [
            (seed >> 24) as u8,
            (seed >> 16) as u8,
            (seed >> 8) as u8,
            seed as u8,
        ];
        self.random_seed = false;
        for (i, pos) in self.seed.iter_mut().enumerate() {
            *pos = seed_bytes[i % 4];
        }
        self
    }

    pub fn difficulty(mut self, diff: Difficulty) -> RandGenSudoku {
        self.difficulty = diff;
        self
    }
}

fn convert_to_clues(board: &mut SudokuBoard) -> () {
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
                ref v if v.len() == 0 => (),
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
