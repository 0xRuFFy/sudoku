mod backtracking_bf; // backtracking brute force
mod backtracking_ds; // backtracking dynamic selection
mod board;
mod smart_solver; // smart solver
mod solver;

pub use backtracking_bf::BBFS;
pub use backtracking_ds::BDSS;
pub use board::Sudoku;
pub use smart_solver::SmartSolver;
pub use solver::Solver;
