use crate::{board::Sudoku, solver::Solver};

pub type BBFS = BacktrackingBruteForceSolver;

pub struct BacktrackingBruteForceSolver {}

impl BBFS {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for BBFS {
    fn solve(&self, sudoku: &Sudoku) -> Option<Sudoku> {
        let mut sudoku = sudoku.clone();
        let open = sudoku.get_open_cells();
        let mut i = 0;

        loop {
            let (row, col) = open[i];
            let mut value = sudoku.get(row, col);

            if value < 9 {
                value += 1;
                sudoku.set(row, col, value, false);
                if sudoku.is_solved() {
                    break;
                }
                if sudoku.is_valid() {
                    i += 1;
                }
            } else {
                if i == 0 {
                    return None;
                }
                sudoku.unset(row, col, true);
                i -= 1;
            }
        }

        Some(sudoku)
    }
}
