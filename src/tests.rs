use super::{solver::Solver, sudoku::Sudoku};

fn generate_sample() -> Sudoku {
    let mut sample_board = Sudoku::new();
    sample_board.update_value_using_position("a1".to_string(), 8);
    sample_board.update_value_using_position("a4".to_string(), 4);
    sample_board.update_value_using_position("a6".to_string(), 6);
    sample_board.update_value_using_position("a9".to_string(), 7);
    sample_board.update_value_using_position("b7".to_string(), 4);
    sample_board.update_value_using_position("c2".to_string(), 1);
    sample_board.update_value_using_position("c7".to_string(), 6);
    sample_board.update_value_using_position("c8".to_string(), 5);
    sample_board.update_value_using_position("d1".to_string(), 5);
    sample_board.update_value_using_position("d3".to_string(), 9);
    sample_board.update_value_using_position("d5".to_string(), 3);
    sample_board.update_value_using_position("d7".to_string(), 7);
    sample_board.update_value_using_position("d8".to_string(), 8);
    sample_board.update_value_using_position("e5".to_string(), 7);
    sample_board.update_value_using_position("f2".to_string(), 4);
    sample_board.update_value_using_position("f3".to_string(), 8);
    sample_board.update_value_using_position("f5".to_string(), 2);
    sample_board.update_value_using_position("f7".to_string(), 1);
    sample_board.update_value_using_position("f9".to_string(), 3);
    sample_board.update_value_using_position("g2".to_string(), 5);
    sample_board.update_value_using_position("g3".to_string(), 2);
    sample_board.update_value_using_position("g8".to_string(), 9);
    sample_board.update_value_using_position("h3".to_string(), 1);
    sample_board.update_value_using_position("i1".to_string(), 3);
    sample_board.update_value_using_position("i4".to_string(), 9);
    sample_board.update_value_using_position("i6".to_string(), 2);
    sample_board.update_value_using_position("i9".to_string(), 5);
    return sample_board;
}

fn generate_target() -> Sudoku {
    let mut target_board = Sudoku::new();
    // Row "a"
    target_board.update_value_using_position("a1".to_string(), 8);
    target_board.update_value_using_position("a2".to_string(), 3);
    target_board.update_value_using_position("a3".to_string(), 5);
    target_board.update_value_using_position("a4".to_string(), 4);
    target_board.update_value_using_position("a5".to_string(), 1);
    target_board.update_value_using_position("a6".to_string(), 6);
    target_board.update_value_using_position("a7".to_string(), 9);
    target_board.update_value_using_position("a8".to_string(), 2);
    target_board.update_value_using_position("a9".to_string(), 7);
    // Row "b"
    target_board.update_value_using_position("b1".to_string(), 2);
    target_board.update_value_using_position("b2".to_string(), 9);
    target_board.update_value_using_position("b3".to_string(), 6);
    target_board.update_value_using_position("b4".to_string(), 8);
    target_board.update_value_using_position("b5".to_string(), 5);
    target_board.update_value_using_position("b6".to_string(), 7);
    target_board.update_value_using_position("b7".to_string(), 4);
    target_board.update_value_using_position("b8".to_string(), 3);
    target_board.update_value_using_position("b9".to_string(), 1);
    // Row "c"
    target_board.update_value_using_position("c1".to_string(), 4);
    target_board.update_value_using_position("c2".to_string(), 1);
    target_board.update_value_using_position("c3".to_string(), 7);
    target_board.update_value_using_position("c4".to_string(), 2);
    target_board.update_value_using_position("c5".to_string(), 9);
    target_board.update_value_using_position("c6".to_string(), 3);
    target_board.update_value_using_position("c7".to_string(), 6);
    target_board.update_value_using_position("c8".to_string(), 5);
    target_board.update_value_using_position("c9".to_string(), 8);
    // Row "d"
    target_board.update_value_using_position("d1".to_string(), 5);
    target_board.update_value_using_position("d2".to_string(), 6);
    target_board.update_value_using_position("d3".to_string(), 9);
    target_board.update_value_using_position("d4".to_string(), 1);
    target_board.update_value_using_position("d5".to_string(), 3);
    target_board.update_value_using_position("d6".to_string(), 4);
    target_board.update_value_using_position("d7".to_string(), 7);
    target_board.update_value_using_position("d8".to_string(), 8);
    target_board.update_value_using_position("d9".to_string(), 2);
    // Row "e"
    target_board.update_value_using_position("e1".to_string(), 1);
    target_board.update_value_using_position("e2".to_string(), 2);
    target_board.update_value_using_position("e3".to_string(), 3);
    target_board.update_value_using_position("e4".to_string(), 6);
    target_board.update_value_using_position("e5".to_string(), 7);
    target_board.update_value_using_position("e6".to_string(), 8);
    target_board.update_value_using_position("e7".to_string(), 5);
    target_board.update_value_using_position("e8".to_string(), 4);
    target_board.update_value_using_position("e9".to_string(), 9);
    // Row "f"
    target_board.update_value_using_position("f1".to_string(), 7);
    target_board.update_value_using_position("f2".to_string(), 4);
    target_board.update_value_using_position("f3".to_string(), 8);
    target_board.update_value_using_position("f4".to_string(), 5);
    target_board.update_value_using_position("f5".to_string(), 2);
    target_board.update_value_using_position("f6".to_string(), 9);
    target_board.update_value_using_position("f7".to_string(), 1);
    target_board.update_value_using_position("f8".to_string(), 6);
    target_board.update_value_using_position("f9".to_string(), 3);
    // Row "g"
    target_board.update_value_using_position("g1".to_string(), 6);
    target_board.update_value_using_position("g2".to_string(), 5);
    target_board.update_value_using_position("g3".to_string(), 2);
    target_board.update_value_using_position("g4".to_string(), 7);
    target_board.update_value_using_position("g5".to_string(), 8);
    target_board.update_value_using_position("g6".to_string(), 1);
    target_board.update_value_using_position("g7".to_string(), 3);
    target_board.update_value_using_position("g8".to_string(), 9);
    target_board.update_value_using_position("g9".to_string(), 4);
    // Row "h"
    target_board.update_value_using_position("h1".to_string(), 9);
    target_board.update_value_using_position("h2".to_string(), 8);
    target_board.update_value_using_position("h3".to_string(), 1);
    target_board.update_value_using_position("h4".to_string(), 3);
    target_board.update_value_using_position("h5".to_string(), 4);
    target_board.update_value_using_position("h6".to_string(), 5);
    target_board.update_value_using_position("h7".to_string(), 2);
    target_board.update_value_using_position("h8".to_string(), 7);
    target_board.update_value_using_position("h9".to_string(), 6);
    // Row "i"
    target_board.update_value_using_position("i1".to_string(), 3);
    target_board.update_value_using_position("i2".to_string(), 7);
    target_board.update_value_using_position("i3".to_string(), 4);
    target_board.update_value_using_position("i4".to_string(), 9);
    target_board.update_value_using_position("i5".to_string(), 6);
    target_board.update_value_using_position("i6".to_string(), 2);
    target_board.update_value_using_position("i7".to_string(), 8);
    target_board.update_value_using_position("i8".to_string(), 1);
    target_board.update_value_using_position("i9".to_string(), 5);
    return target_board;
}

#[test]
#[ignore] //for now
fn solve_sudoku() {
    let sample = generate_sample();
    let target = generate_target();
    assert_eq!(sample, target);
}

#[test]
fn check_board_rows_ok() {
    let sample = generate_sample();
    let solver = Solver::new(sample);
    solver.check_board().unwrap();
}

#[test]
#[should_panic]
fn check_board_rows_fail() {
    let mut sample = generate_sample();
    sample.update_value_using_position("a2".to_string(), 8);
    let solver = Solver::new(sample);
    solver.check_board().unwrap();
}

#[test]
fn check_board_columns_ok() {
    let sample = generate_sample();
    let solver = Solver::new(sample);
    solver.check_board().unwrap();
}

#[test]
#[should_panic]
fn check_board_columns_fail() {
    let mut sample = generate_sample();
    sample.update_value_using_position("b1".to_string(), 8);
    let solver = Solver::new(sample);
    solver.check_board().unwrap();
}
#[test]
fn check_board_subsquares() -> Result<(), ()> {
    Ok(())
}
