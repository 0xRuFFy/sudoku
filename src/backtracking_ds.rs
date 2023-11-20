use crate::{board::Sudoku, solver::Solver};

pub type BDSS = BacktrackingDynamicSelectionSolver;

pub struct BacktrackingDynamicSelectionSolver;

impl BDSS {
    pub fn new() -> Self {
        Self {}
    }

    fn get_least_variable_cell(sudoku: &mut Sudoku) -> ((usize, usize), Vec<u8>) {
        let open = sudoku.get_open_cells();
        let mut min = 9;
        let mut min_cell = (0, 0);
        let mut min_values = Vec::new();

        for (row, col) in open.iter() {
            let values = sudoku.get_pencil_marks(*row, *col);
            if values.is_empty() {
                return ((*row, *col), values);
            }

            if values.len() == 1 {
                return ((*row, *col), values);
            }

            if values.len() < min {
                min = values.len();
                min_cell = (*row, *col);
                min_values = values;
            }
        }

        (min_cell, min_values)
    }
}

impl Solver for BDSS {
    fn solve(&self, sudoku: &Sudoku) -> Option<Sudoku> {
        // TODO: maybe dont clone the sudoku each recursion (sounds like kinda a lot of work)
        let mut sudoku = sudoku.clone();

        loop {
            let ((row, col), values) = Self::get_least_variable_cell(&mut sudoku);
            if values.is_empty() {
                return None;
            }

            for value in values {
                sudoku.set(row, col, value, true); // true -> pencil mark is already valid
                if sudoku.is_solved() {
                    return Some(sudoku);
                }
                let result = self.solve(&sudoku);
                if result.is_some() {
                    return result;
                }
            }

            return None;
        }
    }
}
