pub struct SudokuBoard {
    pub values: [u8; 81],
}

static NUMBERS: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

impl SudokuBoard {
    pub fn with_clues(clues: &[(usize, usize, u8)]) -> SudokuBoard {
        let mut result = SudokuBoard { values: [0; 81] };

        clues
            .iter()
            .for_each(|(row, col, val)| result.values[row * 9 + col] = *val);

        result
    }

    pub fn is_filled(&self) -> bool {
        self.values.iter().all(|&val| val > 0)
    }

    pub fn is_valid(&self) -> bool {
        (0..9).into_iter().all(|group| {
            NUMBERS.iter().all(|&n| {
                match (0..9).into_iter().fold((0, 0, 0), |(r, c, b), pos| {
                    (
                        // Sum values in rows
                        r + if self.values[group * 9 + pos] == n {
                            1
                        } else {
                            0
                        },
                        // Sum values in columns
                        c + if self.values[pos * 9 + group] == n {
                            1
                        } else {
                            0
                        },
                        // Sum values in boxes
                        b + if self.values
                            [((group / 3) * 3 + pos / 3) * 9 + (group % 3) * 3 + pos % 3]
                            == n
                        {
                            1
                        } else {
                            0
                        },
                    )
                }) {
                    (x, y, z) if x > 1 || y > 1 || z > 1 => false,
                    _ => true,
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn when_creating_board_with_clue_then_place_clue_correctly() {
        let clues = [(0, 0, 1)];
        let board = SudokuBoard::with_clues(&clues);
        assert_eq!(board.values[0], 1);
    }

    #[test]
    fn when_creating_board_with_many_clues_then_place_clues_correctly() {
        let clues = [(1, 1, 1), (2, 7, 9), (7, 7, 5)];
        let board = SudokuBoard::with_clues(&clues);
        assert_eq!(board.values[10], 1);
        assert_eq!(board.values[25], 9);
        assert_eq!(board.values[70], 5);
    }

    #[test]
    fn when_board_is_empty_then_is_filled_returns_false() {
        let board = SudokuBoard::with_clues(&[]);
        assert_eq!(board.is_filled(), false);
    }

    #[test]
    fn when_board_not_full_then_is_filled_returns_false() {
        let clues = [(1, 1, 1), (2, 7, 9), (7, 7, 5)];
        let board = SudokuBoard::with_clues(&clues);
        assert_eq!(board.is_filled(), false);
    }

    #[test]
    fn when_board_is_full_then_is_filled_returns_true() {
        let mut board = SudokuBoard::with_clues(&[]);
        board.values = [1; 81];
        assert_eq!(board.is_filled(), true);
    }

    #[test]
    fn when_board_is_empty_then_it_is_valid() {
        check_is_valid(&[], true);
    }

    #[test]
    fn when_board_has_no_duplicates_on_row_col_or_box_then_it_is_valid() {
        check_is_valid(
            &[
                (0, 4, 2),
                (1, 4, 1),
                (2, 4, 7),
                (3, 3, 3),
                (3, 4, 6),
                (3, 5, 2),
                (4, 0, 3),
                (4, 1, 2),
                (4, 2, 7),
                (4, 3, 4),
                (4, 4, 5),
                (4, 5, 9),
                (4, 6, 1),
                (4, 7, 8),
                (4, 8, 6),
                (5, 3, 1),
                (5, 4, 8),
                (5, 5, 7),
                (6, 4, 3),
                (7, 4, 9),
                (8, 4, 4),
            ],
            true,
        );
    }

    #[test]
    fn when_board_has_same_number_duplicated_on_row_then_it_is_not_valid() {
        check_is_valid(&[(7, 0, 2), (7, 8, 2)], false);
    }

    #[test]
    fn when_board_has_same_number_duplicated_on_column_then_it_is_not_valid() {
        check_is_valid(&[(2, 3, 7), (6, 3, 7)], false);
    }

    #[test]
    fn when_board_has_same_number_duplicated_in_box_then_it_is_not_valid() {
        check_is_valid(&[(3, 3, 1), (5, 5, 1)], false);
    }

    fn check_is_valid(clues: &[(usize, usize, u8)], expected: bool) -> () {
        let board = SudokuBoard::with_clues(clues);
        assert_eq!(board.is_valid(), expected);
    }
}
