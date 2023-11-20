use crate::{backtracking_ds, board::Sudoku, solver::Solver};

pub struct SmartSolver {
    pencil_marks: [[u128; 9]; 9],
}

impl SmartSolver {
    pub fn new(sudoku: &Sudoku) -> Self {
        let mut pencil_marks = [[0; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                let value = sudoku.get(row, col);
                if value == 0 {
                    pencil_marks[row][col] = sudoku.get_pencil_marks_raw(row, col);
                }
            }
        }

        Self { pencil_marks }
    }

    fn logic_process(&self, sudoku: &Sudoku) -> Sudoku {
        let mut new_sudoku = sudoku.clone();

        loop {
            while let Some(cells) = self.get_unambiguous_cells() {
                for (row, col, value) in cells {
                    new_sudoku.set(row, col, value, true);
                }
            }

            if let Some(cells) = self.get_unique_pencil_marked_cells() {
                for (row, col, value) in cells {
                    new_sudoku.set(row, col, value, true);
                }
            } else {
                break;
            }
        }

        new_sudoku
    }

    fn get_unambiguous_cells(&self) -> Option<Vec<(usize, usize, u8)>> {
        todo!("Implement SmartSolver::get_unambiguous_cell")
    }

    fn get_unique_pencil_marked_cells(&self) -> Option<Vec<(usize, usize, u8)>> {
        todo!("Implement SmartSolver::get_unique_pencil_marked_cell")
    }

    fn update_pencil_marks(&self) {
        todo!("Implement SmartSolver::update_pencil_marks")
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
