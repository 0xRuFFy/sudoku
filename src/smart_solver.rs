use crate::backtracking_ds;
use crate::board::{Sudoku, SQUARES, SQUARES_INVERSE};
use crate::solver::Solver;

pub struct SmartSolver {
    pencil_marks: [[u16; 9]; 9],
}

impl SmartSolver {
    pub fn new() -> Self {
        Self {
            pencil_marks: [[0; 9]; 9],
        }
    }

    fn init_pencil_marks(&mut self, sudoku: &Sudoku) {
        for row in 0..9 {
            for col in 0..9 {
                let value = sudoku.get(row, col);
                if value == 0 {
                    self.pencil_marks[row][col] = sudoku.get_pencil_marks_raw(row, col);
                }
            }
        }
    }

    fn logic_process(&mut self, sudoku: &Sudoku) -> Sudoku {
        self.init_pencil_marks(sudoku);
        let mut new_sudoku = sudoku.clone();

        loop {
            while let Some(cells) = self.get_unambiguous_cells() {
                // println!("Unambiguous cells: {:?}", cells);
                for (row, col, value) in cells.clone() {
                    new_sudoku.set(row, col, value, true);
                }
                // println!("{}", new_sudoku);
                self.update_pencil_marks(&cells);
            }

            if new_sudoku.is_solved() {
                break;
            }

            if let Some(cells) = self.get_unique_pencil_marked_cells() {
                for (row, col, value) in cells.clone() {
                    new_sudoku.set(row, col, value, true);
                }
                self.update_pencil_marks(&cells);
            } else {
                break;
            }
        }

        new_sudoku
    }

    fn get_unambiguous_cells(&self) -> Option<Vec<(usize, usize, u8)>> {
        let mut cells = Vec::new();

        for row in 0..9 {
            for col in 0..9 {
                let marks = self.pencil_marks[row][col];
                if marks.count_ones() == 1 {
                    let value = marks.trailing_zeros() as u8 + 1;
                    cells.push((row, col, value));
                }
            }
        }

        if cells.is_empty() {
            None
        } else {
            Some(cells)
        }
    }

    fn get_unique_pencil_marked_cells(&self) -> Option<Vec<(usize, usize, u8)>> {
        todo!("Implement SmartSolver::get_unique_pencil_marked_cell")
    }

    fn update_pencil_marks(&mut self, placed_cells: &[(usize, usize, u8)]) {
        // todo!("Implement SmartSolver::update_pencil_marks")
        for cell in placed_cells {
            let (row, col, value) = *cell;
            self.pencil_marks[row][col] = 0;
            for i in 0..9 {
                self.pencil_marks[row][i] &= !(1 << (value - 1));
                self.pencil_marks[i][col] &= !(1 << (value - 1));
            }
            let square = SQUARES[row][col];
            for i in 0..9 {
                let (row, col) = SQUARES_INVERSE[square][i];
                self.pencil_marks[row][col] &= !(1 << (value - 1));
            }
        }
    }

    fn back_track(&self, sudoku: &Sudoku) -> Sudoku {
        todo!("Implement SmartSolver::back_track_ds")
    }
}

impl Solver for SmartSolver {
    fn solve(&mut self, sudoku: &Sudoku) -> Option<Sudoku> {
        let mut sudoku = self.logic_process(sudoku);

        if sudoku.is_solved() {
            return Some(sudoku);
        }

        // TODO: use backtrack to find a value that must be correct -> set it -> logic process -> backtrack ...
        // a value must be correct if all other options are proven to be wrong and no other values
        // were guessed before
        sudoku = backtracking_ds::BDSS::new().solve(&sudoku)?;

        if sudoku.is_solved() {
            return Some(sudoku);
        }

        None
    }
}
