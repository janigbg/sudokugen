# sudokugen
sudokugen is a Sudoku (9x9) puzzle generator and solver crate.
It is designed to generate, solve and verify Sudoku puzzles and allows for extension
with custom solvers and generators.

## Example

```rust
extern crate rand;
extern crate sudokugen;

use sudokugen::generator::random_gen::*;
use sudokugen::generator::{Difficulty, Generator};
use sudokugen::solver::least_options::LeastOptionsSolver;
use sudokugen::solver::Solver;

fn generate_puzzle() -> Result<(), Box<std::error::Error>> {
    // Provide a Solver to generate puzzle
    let solver = LeastOptionsSolver::new();
    // Create puzzle generator of specified difficulty
    let mut generator = RandomSudoku::new(solver).difficulty(Difficulty::Evil);
    // Create a puzzle
    let puzzle = generator.run()?;
    // Print out the board
    println!("Verified board:\n{}", puzzle.board);
    Ok(())
}

fn main() {
    if let Err(e) = generate_puzzle() {
        error!("Error during puzzle generation: {}", e);
    }
}
```

## License

This project is licensed under [The Unlicense](UNLICENSE)