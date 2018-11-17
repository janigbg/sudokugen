use super::super::board::{add, sub, Group, Placement, SudokuBoard};
use super::super::solver::{Solution, Solver};
use super::{Difficulty, Generator, Puzzle};
use rand::prelude::*;

pub struct RandGenerator {
    solver: Box<Solver>,
    random_seed: bool,
    seed: [u8; 16],
    difficulty: Difficulty,
}

impl Generator for RandGenerator {
    fn generate(&mut self) -> Result<Puzzle, String> {

        if self.random_seed {
            thread_rng().fill_bytes(&mut self.seed);
        }
        let mut rng = SmallRng::from_seed(self.seed);

        let mut board: SudokuBoard;

        loop {
            board = SudokuBoard::with_clues(&[]);

            let mut add_sequence: Vec<usize> = (0..81).collect();
            rng.shuffle(&mut add_sequence);
            let mut index = 0;
            for _iteration in 0..25 {
                let mut row: usize;
                let mut col: usize;
                while index < 81 {
                    row = add_sequence[index] / 9;
                    col = add_sequence[index] % 9;
                    let placements = board.get_available_placements(row, col);
                    index += 1;
                    match (0u8..9u8)
                        .filter(|&val| placements[val as usize] == 1)
                        .collect::<Vec<u8>>()
                    {
                        ref v if v.len() == 0 => (),
                        values => {
                            board
                                .place((row, col, *rng.choose(&values).unwrap() + 1))
                                .unwrap();
                            break;
                        }
                    }
                }
            }

            println!("Starting board:\r\n{}", &board);

            let result = self.solver.try_solve(&mut board, Some(10000));

            match result {
                Err(_) => {
                    println!("Could not solve board!");
                }
                Ok(solution) => {
                    println!("Solved board:\r\n{}", &board);
                    println!("Solution: {:?}", solution);
                    break;
                }
            }
        }

        let mut removal_sequence: Vec<usize> = (0..81).collect();

        rng.shuffle(&mut removal_sequence);
        let mut count = 0;
        let mut removed_cells = 0;

        while count < 81 {
            let index = removal_sequence[count];
            if board.values[index] > 0 && !board.clues[index] {
                let num = board.values[index];
                let (row, col) = (index / 9, index % 9);
                board.place((row, col, 0)).unwrap();
                count += 1;
                if self.solver.verify(&board) {
                    removed_cells += 1;
                } else {
                    board.place((row, col, num)).unwrap();
                }
            }
        }

        println!("\r\n# of clues: {}\r\n", 81 - removed_cells);
        println!("Verified board:\r\n{}", &board);

        Ok(Puzzle{
            board,
            difficulty: self.difficulty
        })
    }
}

impl RandGenerator {
    pub fn new(solver: Box<Solver>) -> RandGenerator {
        RandGenerator {
            solver,
            random_seed: true,
            seed: [0; 16],
            difficulty: Difficulty::Easy,
        }
    }

    pub fn seed<'a>(&'a mut self, seed: u32) -> &'a mut RandGenerator {
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

    pub fn difficulty<'a>(&'a mut self, diff: Difficulty) -> &'a mut RandGenerator {
        self.difficulty = diff;
        self
    }
}
