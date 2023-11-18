use crate::solver::Solver;

// ------------| Unicode Art |--------------
const TOP_ROW: &str = "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗";
const MIDDLE_ROW_DOUBLE: &str = "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣";
const MIDDLE_ROW_SINGLE: &str = "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢";
const BOTTOM_ROW: &str = "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝";
const STRAIGHT_DOUBLE: &str = "║";
const STRAIGHT_SINGLE: &str = "│";
const EMPTY: &str = "   ";

// ------------| Lookup Tables |--------------
// const ROWS: [[usize; 9]; 9] = [
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [1, 1, 1, 1, 1, 1, 1, 1, 1],
//     [2, 2, 2, 2, 2, 2, 2, 2, 2],
//     [3, 3, 3, 3, 3, 3, 3, 3, 3],
//     [4, 4, 4, 4, 4, 4, 4, 4, 4],
//     [5, 5, 5, 5, 5, 5, 5, 5, 5],
//     [6, 6, 6, 6, 6, 6, 6, 6, 6],
//     [7, 7, 7, 7, 7, 7, 7, 7, 7],
//     [8, 8, 8, 8, 8, 8, 8, 8, 8],
// ];

// const COLS: [[usize; 9]; 9] = [
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
//     [0, 1, 2, 3, 4, 5, 6, 7, 8],
// ];

const SQUARES: [[usize; 9]; 9] = [
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [0, 0, 0, 1, 1, 1, 2, 2, 2],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [3, 3, 3, 4, 4, 4, 5, 5, 5],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
    [6, 6, 6, 7, 7, 7, 8, 8, 8],
];

type SudokuBoard = [[u8; 9]; 9];

#[derive(Clone)]
pub struct Sudoku {
    board: SudokuBoard,
    valid: bool,
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            board: [[0; 9]; 9],
            valid: true,
        }
    }

    pub fn load_board_from_str(&mut self, board: &str) {
        if board.len() != 81 {
            panic!("Board must be 81 characters long.");
        }
        let board = board
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid character in board string.") as u8)
            .collect::<Vec<u8>>();

        for (i, value) in board.iter().enumerate() {
            let row = i / 9;
            let col = i % 9;
            self.set(row, col, *value);
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.board[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.board[row][col] = value;
        self.valid = self.is_valid_row(row)
            && self.is_valid_column(col)
            && self.is_valid_square(SQUARES[row][col]);
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_solved(&self) -> bool {
        self.valid && self.get_open_cells().is_empty()
    }

    pub fn solve(&mut self, solver: &dyn Solver) {
        // let mut count = 0;
        match solver.solve(self) {
            Some(s) => {
                *self = s;
                // println!("Solved in {} iterations.", count);
            }
            None => panic!("No solution found."),
        }
    }

    pub fn get_open_cells(&self) -> Vec<(usize, usize)> {
        let mut open_cells = Vec::new();
        for (i, row) in self.board.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value == 0 {
                    open_cells.push((i, j));
                }
            }
        }
        open_cells
    }

    fn is_valid_row(&self, row: usize) -> bool {
        let mut seen = 0u16;
        let mut mask: u16;
        for value in self.board[row].iter() {
            if *value == 0 {
                continue;
            }
            mask = 1 << value;
            if seen & mask != 0 {
                return false;
            }
            seen |= mask;
        }
        true
    }

    fn is_valid_column(&self, col: usize) -> bool {
        let mut seen = 0u16;
        let mut mask: u16;
        for row in 0..9 {
            let value = self.board[row][col];
            if value == 0 {
                continue;
            }
            mask = 1 << value;
            if seen & mask != 0 {
                return false;
            }
            seen |= mask;
        }
        true
    }

    fn is_valid_square(&self, square: usize) -> bool {
        let mut seen = 0u16;
        let mut mask: u16;
        for i in 0..9 {
            let row = (square / 3) * 3 + (i / 3);
            let col = (square % 3) * 3 + (i % 3);
            let value = self.board[row][col];
            if value == 0 {
                continue;
            }
            mask = 1 << value;
            if seen & mask != 0 {
                return false;
            }
            seen |= mask;
        }
        true
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(TOP_ROW);
        result.push('\n');

        for (i, row) in self.board.iter().enumerate() {
            result.push_str("║");
            for (j, value) in row.iter().enumerate() {
                if value == &0 {
                    result.push_str(EMPTY);
                } else {
                    result.push_str(&format!(" {} ", value));
                }

                if j == 2 || j == 5 {
                    result.push_str(STRAIGHT_DOUBLE);
                } else if j != 8 {
                    result.push_str(STRAIGHT_SINGLE);
                }
            }
            result.push_str(STRAIGHT_DOUBLE);
            result.push('\n');

            if i == 2 || i == 5 {
                result.push_str(MIDDLE_ROW_DOUBLE);
                result.push('\n');
            } else if i != 8 {
                result.push_str(MIDDLE_ROW_SINGLE);
                result.push('\n');
            }
        }

        result.push_str(BOTTOM_ROW);

        write!(f, "{}", result)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const TEST_BOARD_VALID: [[u8; 9]; 9] = [
//         [5, 3, 0, 0, 7, 0, 0, 0, 0],
//         [6, 0, 0, 1, 9, 5, 0, 0, 0],
//         [0, 9, 8, 0, 0, 0, 0, 6, 0],
//         [8, 0, 0, 0, 6, 0, 0, 0, 3],
//         [4, 0, 0, 8, 0, 3, 0, 0, 1],
//         [7, 0, 0, 0, 2, 0, 0, 0, 6],
//         [0, 6, 0, 0, 0, 0, 2, 8, 0],
//         [0, 0, 0, 4, 1, 9, 0, 0, 5],
//         [0, 0, 0, 0, 8, 0, 0, 7, 9],
//     ];

//     const TEST_BOARD_INVALID: [[u8; 9]; 9] = [
//         [5, 3, 0, 0, 7, 0, 0, 0, 0],
//         [6, 5, 0, 1, 9, 5, 0, 0, 0],
//         [0, 9, 8, 0, 0, 0, 0, 6, 0],
//         [8, 0, 0, 0, 6, 0, 0, 5, 3],
//         [4, 0, 0, 8, 0, 3, 0, 0, 1],
//         [7, 0, 0, 0, 2, 0, 5, 0, 6],
//         [0, 6, 0, 0, 7, 0, 2, 8, 0],
//         [0, 5, 0, 4, 1, 9, 0, 6, 5],
//         [0, 0, 0, 0, 8, 0, 0, 7, 9],
//     ];

//     const TEST_BOARD_INVALID_VALID_ROWS: [u8; 7] = [0, 2, 3, 4, 5, 6, 8];
//     const TEST_BOARD_INVALID_INVALID_ROWS: [u8; 2] = [1, 7];
//     const TEST_BOARD_INVALID_VALID_COLUMNS: [u8; 6] = [0, 2, 3, 5, 6, 8];
//     const TEST_BOARD_INVALID_INVALID_COLUMNS: [u8; 3] = [1, 4, 7];
//     const TEST_BOARD_INVALID_VALID_SQUARES: [u8; 7] = [1, 2, 3, 4, 6, 7, 8];
//     const TEST_BOARD_INVALID_INVALID_SQUARES: [u8; 2] = [0, 5];
// }
