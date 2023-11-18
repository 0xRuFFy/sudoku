use crate::board::Sudoku;

pub trait Solver {
    /// Solves the given sudoku and returns the solution if it exists.
    ///
    /// # Arguments
    /// * `sudoku` - The sudoku to solve.
    ///
    /// # Returns
    /// * `Some(Sudoku)` - The solution to the sudoku.
    /// * `None` - If the sudoku has no solution.
    fn solve(&self, sudoku: &Sudoku) -> Option<Sudoku>;
}
