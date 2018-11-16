extern crate sudokugen;
extern crate rand;
use rand::prelude::*;
use sudokugen::solver::Solver;
use sudokugen::solver::least_options::LeastOptionsSolver;
use sudokugen::board::{SudokuBoard};
use std::io;

fn main() -> () {
    // let mut board = SudokuBoard::with_defaults(&vec![
    //     // (0,3,3),(0,4,7),(0,5,6),(0,8,8),
    //     // (1,5,1),(1,8,6),
    //     // (2,1,2),
    //     // (3,0,7),(3,2,4),(3,6,9),(3,7,3),
    //     // (4,0,1),(4,2,8),(4,5,2),
    //     // (5,0,2),(5,1,6),(5,4,9),(5,8,4),
    //     // (6,5,3),(6,7,7),(6,8,1),
    //     // (7,6,8),(7,7,6),
    //     // (8,3,9),(8,4,6)

    //     // (0,0,8),(0,4,1),(0,8,9),
    //     // (1,1,4),(1,4,8),(1,7,2),
    //     // (2,3,2),(2,5,5),
    //     // (3,2,8),(3,4,2),(3,6,7),
    //     // (4,0,5),(4,1,6),(4,3,7),(4,5,1),(4,7,9),(4,8,8),
    //     // (5,2,1),(5,4,9),(5,6,5),
    //     // (6,3,9),(6,5,2),
    //     // (7,1,8),(7,4,4),(7,7,6),
    //     // (8,0,7),(8,4,5),(8,8,4)

    //     (1,1,9),(1,4,1),(1,7,3),
    //     (2,2,6),(2,4,2),(2,6,7),
    //     (3,3,3),(3,5,4),
    //     (4,0,2),(4,1,1),(4,7,9),(4,8,8),
    //     (6,2,2),(6,3,5),(6,5,6),(6,6,4),
    //     (7,1,8),(7,7,1)
    // ]);

    //let mut rng = StdRng::from_entropy();
    let mut seed = [0;16];
    thread_rng().fill_bytes(&mut seed);
    let mut rng = SmallRng::from_seed(seed);
    let index = [0,1,2,3,4,5,6,7,8];
    let mut solver = LeastOptionsSolver::new();

    let mut board: SudokuBoard;
    
    loop {
        board = SudokuBoard::with_clues(&[]);

        for _iteration in 0..30 {
            let mut row: usize;
            let mut col: usize;
            loop {
                row = *rng.choose(&index).unwrap();
                col = *rng.choose(&index).unwrap();
                if board.values[row * 9 + col] == 0 {
                    let placements = board.get_available_placements(row, col);
                    match (0u8..9u8).filter(|&val| placements[val as usize] == 1).collect::<Vec<u8>>() {
                        ref v if v.len() == 0 => (),
                        values => {
                            board.place((row, col, *rng.choose(&values).unwrap() + 1)).unwrap();
                            break;
                        }
                    }
                }
            }
        }

        println!("Starting board:\r\n{}", &board);

        let result = solver.try_solve(&mut board, Some(10000));

        println!("Solved board:\r\n{}", &board);
        match result {
            Err(_) => {
                println!("Could not solve board!");
            },
            Ok(solution) => {
                println!("Solution: {:?}", solution);
                break;
            }
        }

    }
  
    let mut removal_sequence: Vec<usize> = 
        (0..81).collect();

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
            if solver.verify(&board) {
                removed_cells += 1;
            } else {
                board.place((row, col, num)).unwrap();                    
            }
        }
    }

    println!("\r\n# of clues: {}\r\n", 81 - removed_cells);
    println!("Verified board:\r\n{}", &board);

    solver.solve(&mut board.clone()).unwrap();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}