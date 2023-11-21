use crate::solver::Solver;

// ------------| Lookup Tables  / Constants |--------------
const CELL_MASK_LEN: usize = 9;
const CELL_MASK: u128 = 0b111111111;

pub const SQUARES: [[usize; 9]; 9] = [
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

pub const SQUARES_INVERSE: [[(usize, usize); 9]; 9] = [
    [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
    [(0, 3), (0, 4), (0, 5), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5)],
    [(0, 6), (0, 7), (0, 8), (1, 6), (1, 7), (1, 8), (2, 6), (2, 7), (2, 8)],
    [(3, 0), (3, 1), (3, 2), (4, 0), (4, 1), (4, 2), (5, 0), (5, 1), (5, 2)],
    [(3, 3), (3, 4), (3, 5), (4, 3), (4, 4), (4, 5), (5, 3), (5, 4), (5, 5)],
    [(3, 6), (3, 7), (3, 8), (4, 6), (4, 7), (4, 8), (5, 6), (5, 7), (5, 8)],
    [(6, 0), (6, 1), (6, 2), (7, 0), (7, 1), (7, 2), (8, 0), (8, 1), (8, 2)],
    [(6, 3), (6, 4), (6, 5), (7, 3), (7, 4), (7, 5), (8, 3), (8, 4), (8, 5)],
    [(6, 6), (6, 7), (6, 8), (7, 6), (7, 7), (7, 8), (8, 6), (8, 7), (8, 8)],
];

pub const IN_SQUARE_IDX: [[usize; 9]; 9] = [
    [0, 1, 2, 0, 1, 2, 0, 1, 2],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
    [0, 1, 2, 0, 1, 2, 0, 1, 2],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
    [0, 1, 2, 0, 1, 2, 0, 1, 2],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
];

#[derive(Clone)]
pub struct Sudoku {
    rows: [u128; 9],
    cols: [u128; 9],
    squares: [u128; 9],
    valid: bool,

    open_cells: Vec<(usize, usize)>,
}

impl Sudoku {
    // ---------------| Constructors |-----------------
    pub fn new() -> Self {
        Self {
            rows: [0; 9],
            cols: [0; 9],
            squares: [0; 9],
            valid: true,
            open_cells: (0..81)
                .map(|i| Self::index_to_row_col(i))
                .collect::<Vec<(usize, usize)>>(),
        }
    }

    // ---------------| Initializers |-----------------
    pub fn load_board_from_str(&mut self, board: &str, null_char: char) {
        self.rows = [0; 9];
        self.cols = [0; 9];
        self.squares = [0; 9];
        self.valid = true;
        self.open_cells = (0..81)
            .map(|i| Self::index_to_row_col(i))
            .collect::<Vec<(usize, usize)>>();

        if board.len() != 81 {
            panic!("Board must be 81 characters long.");
        }
        let board = board
            .chars()
            .map(|c| {
                if c == null_char {
                    return 0;
                }
                c.to_digit(10).expect("Invalid character in board string.") as u8
            })
            .collect::<Vec<u8>>();

        for (i, value) in board.iter().enumerate() {
            if *value == 0 {
                continue;
            }
            let row = i / 9;
            let col = i % 9;
            self.set(row, col, *value, true);
        }
    }

    // ---------------| Getters & Setters |-----------------
    pub fn get_board_as_str(&self) -> String {
        let mut board = String::with_capacity(81);

        for row in 0..9 {
            for col in 0..9 {
                board.push_str(&self.get(row, col).to_string());
            }
        }

        board
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        let cell = self.rows[row] >> (col * CELL_MASK_LEN) & CELL_MASK;
        if cell == 0 {
            return 0;
        }
        cell.trailing_zeros() as u8 + 1
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8, is_valid: bool) {
        let mask = 1 << (value - 1);
        // let unset_mask = 1 << (self.get(row, col) - 1);
        let square = SQUARES[row][col];
        let index_in_square = IN_SQUARE_IDX[row][col];

        self.unset(row, col, false);

        // self.rows[row] &= !(mask << (col * CELL_MASK_LEN));
        self.rows[row] |= mask << (col * CELL_MASK_LEN);

        // self.cols[col] &= !(mask << (row * CELL_MASK_LEN));
        self.cols[col] |= mask << (row * CELL_MASK_LEN);

        // self.squares[square] &= !(mask << (index_in_square * CELL_MASK_LEN));
        self.squares[square] |= mask << (index_in_square * CELL_MASK_LEN);

        if !is_valid {
            self.valid = self.is_valid_cell(row, col);
        }
        // self.valid = self.is_valid_cell(row, col);
        self.open_cells.retain(|(r, c)| *r != row || *c != col); // TODO: This may be slow
    }

    pub fn unset(&mut self, row: usize, col: usize, check: bool) {
        let value = self.get(row, col);
        if value == 0 {
            return;
        }
        let mask = 1 << (value - 1);
        let square = SQUARES[row][col];
        let index_in_square = IN_SQUARE_IDX[row][col];

        self.rows[row] &= !(mask << (col * CELL_MASK_LEN));
        self.cols[col] &= !(mask << (row * CELL_MASK_LEN));
        self.squares[square] &= !(mask << (index_in_square * CELL_MASK_LEN));

        if check {
            self.valid = self.is_valid_cell(row, col);
        }
        self.open_cells.push((row, col));
    }

    // ---------------| Board State |-----------------
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_solved(&self) -> bool {
        self.valid && self.open_cells.is_empty()
    }

    pub fn get_open_cells(&self) -> Vec<(usize, usize)> {
        self.open_cells.clone()
    }

    pub fn get_pencil_marks(&self, row: usize, col: usize) -> Vec<u8> {
        let mut marks = self.get_pencil_marks_raw(row, col);

        let mut result = Vec::with_capacity(9);
        while marks != 0 {
            let value = marks.trailing_zeros() as u8 + 1;
            result.push(value);
            marks &= !(1 << (value - 1));
        }
        result
    }

    #[inline]
    pub fn get_pencil_marks_raw(&self, row: usize, col: usize) -> u16 {
        let mut marks: u128 = 0b111111111;
        let square = SQUARES[row][col];

        for i in 0..9 {
            marks &= !((self.rows[row] >> (i * CELL_MASK_LEN)) & CELL_MASK);
            marks &= !((self.cols[col] >> (i * CELL_MASK_LEN)) & CELL_MASK);
            marks &= !((self.squares[square] >> (i * CELL_MASK_LEN)) & CELL_MASK);
        }

        marks as u16
    }

    // ---------------| Solving |-----------------
    pub fn solve(&mut self, solver: &mut dyn Solver) {
        match solver.solve(self) {
            Some(s) => *self = s,
            None => panic!("No solution found."),
        }
    }

    // ---------------| Private Helpers |-----------------
    fn is_valid_cell(&self, row: usize, col: usize) -> bool {
        self.is_valid_row(row) && self.is_valid_column(col) && self.is_valid_square(SQUARES[row][col])
    }

    fn is_valid_row(&self, row: usize) -> bool {
        self.is_valid_helper(row, &self.rows)
    }

    fn is_valid_column(&self, col: usize) -> bool {
        self.is_valid_helper(col, &self.cols)
    }

    fn is_valid_square(&self, square: usize) -> bool {
        self.is_valid_helper(square, &self.squares)
    }

    #[inline]
    fn is_valid_helper(&self, index: usize, arr: &[u128; 9]) -> bool {
        let mut releavant = arr[index];
        let mut mask = 0;
        for _ in 0..9 {
            let cell = releavant & CELL_MASK;
            if cell == 0 {
                releavant >>= CELL_MASK_LEN;
                continue;
            }
            if mask & cell != 0 {
                return false;
            }
            mask |= cell;
            releavant >>= CELL_MASK_LEN;
        }
        true
    }

    // ---------------| Static Helpers |-----------------
    pub fn index_to_row_col(index: usize) -> (usize, usize) {
        (index / 9, index % 9)
    }
}

const TOP_ROW: &str = "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗";
const MIDDLE_ROW_DOUBLE: &str = "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣";
const MIDDLE_ROW_SINGLE: &str = "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢";
const BOTTOM_ROW: &str = "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝";
const STRAIGHT_DOUBLE: &str = "║";
const STRAIGHT_SINGLE: &str = "│";
const EMPTY: &str = "   ";

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(TOP_ROW);
        result.push('\n');

        for i in 0..9 {
            result.push_str("║");
            for j in 0..9 {
                let value = self.get(i, j);
                if value == 0 {
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
