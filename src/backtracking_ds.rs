use crate::{board::Sudoku, solver::Solver};

pub type BDSS = BacktrackingDynamicSelectionSolver;

pub struct BacktrackingDynamicSelectionSolver;

impl BDSS {
    pub fn new() -> Self {
        Self {}
    }

    #[time_graph::instrument]
    fn get_possible_values(sudoku: &mut Sudoku, row: usize, col: usize) -> Vec<u8> {
        // let mut possible_values = Vec::new();
        let mut possible_values = Vec::with_capacity(9);
        for value in 1..=9 {
            if sudoku.can_set_validly(row, col, value) {
                possible_values.push(value);
            }
        }
        possible_values
    }

    #[time_graph::instrument]
    fn get_least_variable_cell(sudoku: &mut Sudoku) -> ((usize, usize), Vec<u8>) {
        let open = sudoku.get_open_cells();
        let mut min = 9;
        let mut min_cell = (0, 0);
        let mut min_values = Vec::new();

        for (row, col) in open.iter() {
            let values = Self::get_possible_values(sudoku, *row, *col);
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
    #[time_graph::instrument]
    fn solve(&self, sudoku: &Sudoku) -> Option<Sudoku> {
        let mut sudoku = sudoku.clone();

        loop {
            let ((row, col), values) = Self::get_least_variable_cell(&mut sudoku);

            if values.is_empty() {
                return None;
            }

            for value in values {
                sudoku.set(row, col, value);
                if sudoku.is_solved() {
                    return Some(sudoku);
                }
                if sudoku.is_valid() {
                    let result = self.solve(&sudoku);
                    if result.is_some() {
                        return result;
                    }
                }
            }

            return None;
        }
    }
}
