use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sudoku::Sudoku;

fn backtracking_ds_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("backtracking_ds");

    let mut sudoku = Sudoku::new();
    let mut solver = sudoku::BDSS::new();
    const BOARD: &str = "000000010400000000020000000000050407008000300001090000300400200050100000000806000";

    group.significance_level(0.1).sample_size(500);
    group.measurement_time(std::time::Duration::from_secs(24));
    group.bench_function("backtracking_ds", |b| {
        b.iter(|| {
            sudoku.load_board_from_str(black_box(BOARD), '0');
            sudoku.solve(&mut solver);
        })
    });
    group.finish();
}

fn smart_solver_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("smart_solver");

    let mut sudoku = Sudoku::new();
    let mut solver = sudoku::SmartSolver::new();
    const BOARD: &str = "000000010400000000020000000000050407008000300001090000300400200050100000000806000";

    group.significance_level(0.1).sample_size(500);
    group.measurement_time(std::time::Duration::from_secs(24));
    group.bench_function("smart_solver", |b| {
        b.iter(|| {
            sudoku.load_board_from_str(black_box(BOARD), '0');
            sudoku.solve(&mut solver);
        })
    });
    group.finish();
}

criterion_group!(benches, backtracking_ds_benchmark, smart_solver_benchmark);
criterion_main!(benches);
