use std::fmt;

pub type Placement = (usize, usize, u8);
pub type Group = [u8; 9];

pub fn add(a: Group, b: &Group) -> Group {
    new_array_from(a.iter().zip(b).map(|(a, b)| a + b))
}

pub fn sub(a: Group, b: &Group) -> Group {
    new_array_from(a.iter().zip(b).map(|(a, b)| a - b))
}

fn new_array_from<F: Iterator<Item = u8>>(src: F) -> Group {
    let mut result = [0; 9];
    for (result_ref, val) in result.iter_mut().zip(src) {
        *result_ref = val;
    }
    result
}

static VALUES: Group = [1, 2, 3, 4, 5, 6, 7, 8, 9];

#[derive(Clone)]
pub struct SudokuBoard {
    pub values: [u8; 81],
    pub clues: [bool; 81]
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "-------------------------------------");
        for row in 0..9 {
            write!(f, "|");
            for col in 0..9 {
                write!(
                    f,
                    " {} {}",
                    self.values[row * 9 + col],
                    match col {
                        2 | 5 => "+",
                        8 => "|",
                        _ => ".",
                    }
                );
            }

            writeln!(f);
            writeln!(
                f,
                "{}",
                match row {
                    2 | 5 => "|---+---+---+---+---+---+---+---+---|",
                    8 => "-------------------------------------",
                    _ => "|--- --- ---|--- --- ---|--- --- ---|",
                }
            );
        }
        writeln!(
            f,
            "Valid: {}\tComplete: {}",
            self.is_valid(),
            self.is_filled()
        );
        Ok(())
    }
}

impl SudokuBoard {
    pub fn with_clues(clues: &[Placement]) -> SudokuBoard {
        let mut result = SudokuBoard { values: [0; 81], clues: [false; 81] };

        clues
            .iter()
            .for_each(|(row, col, val)| {
                result.values[row * 9 + col] = *val;
                result.clues[row * 9 + col] = true;
            });

        result
    }

    pub fn is_filled(&self) -> bool {
        self.values.iter().all(|&val| val > 0)
    }

    pub fn is_valid(&self) -> bool {
        (0..9).into_iter().all(|group| {
            VALUES
                .iter()
                .all(|&n| match self.get_count_of_value_in_groups(group, n) {
                    (row, col, a_box) if row > 1 || col > 1 || a_box > 1 => false,
                    _ => true,
                })
        })
    }

    fn get_count_of_value_in_groups(&self, group: usize, n: u8) -> (i32, i32, i32) {
        (0..9)
            .into_iter()
            .fold((0, 0, 0), |(row, col, a_box), pos| {
                (
                    // Count values in rows
                    row + self.check_value_in_position(group * 9 + pos, n),
                    // Count values in columns
                    col + self.check_value_in_position(pos * 9 + group, n),
                    // Count values in boxes
                    a_box + self.check_value_in_position(get_cell_for_box_pos(group, pos), n),
                )
            })
    }

    fn check_value_in_position(&self, index: usize, value: u8) -> i32 {
        if self.values[index] == value {
            1
        } else {
            0
        }
    }

    pub fn get_available_placements(&self, row: usize, col: usize) -> Group {
        if self.values[row * 9 + col] > 0 {
            return [0; 9];
        }

        let the_box = (row / 3) * 3 + (col / 3);

        let result: Vec<u8> = (1..10)
            .into_iter()
            .map(|n| {
                if (0..9).into_iter().all(|pos| {
                    self.values[row * 9 + pos] != n
                        && self.values[pos * 9 + col] != n
                        && self.values[get_cell_for_box_pos(the_box, pos)] != n
                }) {
                    1
                } else {
                    0
                }
            })
            .collect();

        [
            result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
            result[8],
        ]
    }

    pub fn place(&mut self, row: usize, col: usize, num: u8) -> Result<(), String> {
        if num > 9 {
            return Err(String::from(format!("Value out of range: {}", num)),);
        }

        if col > 8 || row > 8 {
            return Err(String::from(format!("Coordinate out of range: ({}, {})", row, col)),);
        }

        match self.clues[row * 9 + col] {
            true => Err(String::from(format!("Cannot place on clue at ({}, {})", row, col)),),
            false => {
                self.values[row * 9 + col] = num;
                Ok(())
            }
        }
    }
}

fn get_cell_for_box_pos(group: usize, pos: usize) -> usize {
    ((group / 3) * 3 + pos / 3) * 9 + (group % 3) * 3 + pos % 3
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

    fn check_is_valid(clues: &[Placement], expected: bool) -> () {
        let board = SudokuBoard::with_clues(clues);
        assert_eq!(board.is_valid(), expected);
    }

    #[test]
    fn when_cell_has_value_then_no_available_placements() {
        let board = SudokuBoard::with_clues(&[(4, 4, 1)]);
        assert_eq!(board.get_available_placements(4, 4), [0; 9]);
    }

    #[test]
    fn when_groups_have_no_values_then_all_available_placements() {
        let board = SudokuBoard::with_clues(&[]);
        assert_eq!(board.get_available_placements(4, 4), [1; 9]);
    }

    #[test]
    fn when_row_has_same_value_then_remove_from_placements() {
        let board = SudokuBoard::with_clues(&[(4, 8, 1)]);
        assert_eq!(board.get_available_placements(4, 4), [0, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn when_col_has_same_value_then_remove_from_placements() {
        let board = SudokuBoard::with_clues(&[(8, 4, 5)]);
        assert_eq!(board.get_available_placements(4, 4), [1, 1, 1, 1, 0, 1, 1, 1, 1]);
    }

    #[test]
    fn when_box_has_same_value_then_remove_from_placements() {
        let board = SudokuBoard::with_clues(&[(5, 5, 9)]);
        assert_eq!(board.get_available_placements(4, 4), [1, 1, 1, 1, 1, 1, 1, 1, 0]);
    }
}
