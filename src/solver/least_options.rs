use super::super::board::{add, Group, SudokuBoard};

pub struct LeastOptionsSolver {}

impl LeastOptionsSolver {
    pub fn new() -> LeastOptionsSolver {
        LeastOptionsSolver {}
    }

    pub fn solve(&self, board: &mut SudokuBoard) -> Result<(), String> {
        if !board.is_valid() {
            return Err(String::from("Cannot solve invalid board"));
        }

        // Pre-calculate available placements for all positions
        // Pre-calculate number of options for all groups and values
        let mut available_options = AvailableOptions::calculate_options(&board);

        while !board.is_filled() {
            return Err(String::from("Not implemented"));
        }
        Ok(())
    }
}

struct AvailableOptions {
    pub placements: [Group; 81],
    pub row_options: [Group; 9],
    pub col_options: [Group; 9],
    pub box_options: [Group; 9],
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
                result.placements[row * 9 + col] = board.get_available_placements(row, col);

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
}

#[cfg(test)]
mod tests {

    use super::super::super::board::SudokuBoard;
    use super::LeastOptionsSolver;

    static SUPER_HARD: [(usize, usize, u8); 25] = [
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

    #[test]
    fn solve_super_hard() {
        let mut board = SudokuBoard::with_clues(&SUPER_HARD);

        LeastOptionsSolver::new()
            .solve(&mut board)
            .expect("Expected success");
    }

    #[test]
    fn solve_fails_for_board_with_invalid_clues() {
        let mut board = SudokuBoard::with_clues(&SUPER_HARD);
        board.values[4] = 4;

        LeastOptionsSolver::new()
            .solve(&mut board)
            .expect_err("Expected error");
    }
}
