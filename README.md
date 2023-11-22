# Rust Sudoku Solver

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This project contains muliple implementations of a sudoku solver in rust.
The goal is to keep improving the performance of the solver by using different algorithms and data structures.

## Contents

- [Rust Sudoku Solver](#rust-sudoku-solver)
  - [Contents](#contents)
  - [Brute Force Backtracking](#brute-force-backtracking)
    - [BFB Algorithm](#bfb-algorithm)
  - [Dynamically Selection Backtracking](#dynamically-selection-backtracking)
    - [DSB Algorithm](#dsb-algorithm)
  - [Smart Backtracking](#smart-backtracking)
    - [Smart Algorithm](#smart-algorithm)
  - [The Board](#the-board)
  - [License](#license)

## Brute Force Backtracking

This implementation is located in `src/backtracking_bf.rs` and uses a simple backtracking algorithm to solve the sudoku.
It is the simplest implementation of the backtracking algorithm and is relatively slow.

### BFB Algorithm

The algorithm is a simple backtracking algorithm using depth-first search.
It tries out every number (1-9) for every empty field and backtracks if it reaches a dead end.
A dead end is reached if the current Sudoku is not valid anymore and the value of the current field is 9.
In that case the field is set to 0 again, the algorithm backtracks to the previous field and tries the next number.
If the first field is reached again and the value is 9, the algorithm stops and returns None -> no solution found.

## Dynamically Selection Backtracking

This implementation is located in `src/backtracking_ds.rs` and uses a dynamically selection of the next field to try out.
This implementation is quite a bit faster than the brute force backtracking, but further improvements are possible.

### DSB Algorithm

The algorithm also uses a backtracking depth-first search, but instead of trying out every number for every field,
it only tries out the numbers that are valid for the current field -> those numbers are stored in the pencil marks.
The Algorithm will always try out the field with the least pencil marks first.

## Smart Backtracking

**THIS IMPLEMENTATION IS STILL IN ACTIVE DEVELOPMENT AND MIGHT SEE FURTHER IMPROVEMENTS**

This implementation is located in `src/smart_solver.rs` and uses in addition to the DSB algorithm a logical solver to solve the sudoku as far as possible.
This implementation is the fastest and can solve 11-Hints Sudokus in under 1ms and Easy to Medium Sudokus in under 100µs.

### Smart Algorithm

After using a logical solver to solve the sudoku as far as possible, the algorithm uses the DSB algorithm to solve the rest of the sudoku.
The logical solver uses the following techniques:

- Naked Single
  - A naked single is a field that has only one possible value left. (Only one pencil mark left)
- Hidden Single
  - A hidden single is a field that is the only field in a row, column or block that can have a certain value.


## The Board

The Sudoku board is represented by 3 Arrays of size 9. One for the rows, one for the columns and one for the blocks.
Each array contains 9 u128 values, where every 9 bits represent a field. The value of the field is then represented by the index of the set bit.
The 47 most significant bits are not used and are always set to 0.
e.g.:

- `0b000000100` means that the field has the value 3.
- `0b000000000` means that the field is empty.
- `0b000000000_010000000_000000000_000000000_000000000_000100000_000000000_001000000_000000001` could represent a row like: `[0, 8, 0, 0, 0, 6, 0, 7, 1]`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
