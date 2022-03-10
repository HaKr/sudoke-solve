#[cfg(test)]
use crate::{
    sudoku::Sudoku, CellIterator, SudokuError, SudokuResult, GRID_COLUMNS, GRID_INDEX_MAX,
    GRID_ROWS,
};
#[cfg(test)]
use std::fmt::Arguments;

#[cfg(test)]
const GELDERLANDER_0308: &str = include_str!("../examples/gelderlander-20220308.txt");

#[cfg(test)]
#[test]
fn gelderlander_20220308() {
    let sudoku = create_sudoku(GELDERLANDER_0308);
    assert_eq!(sudoku.todo_count, 46, "Incorrect number of cells filled");
    let cell = sudoku.cell_at(3, 5);
    assert_eq!(cell.value, Some(9));
    let cell = sudoku.cell_at(5, 5);
    assert_eq!(cell.todo_count, 2);
    let cell = sudoku.cell_at(8, 8);
    assert_eq!(cell.todo_count, 3);
}

#[cfg(test)]
#[test]
fn check_row_indices() -> SudokuResult {
    let sudoku = create_sudoku(GELDERLANDER_0308);

    let check_one_row = |row: usize, result: CellIterator| {
        let start_row_index = row * GRID_COLUMNS;
        let mut expected_indices = start_row_index..(start_row_index + GRID_COLUMNS);
        assert_indices_eq(
            &mut expected_indices,
            result,
            format_args!("Row {} incorrect", row),
        );
    };

    for row in 0..GRID_ROWS {
        let result = sudoku.row_cells_with_check(row)?;
        check_one_row(row, result);
    }

    let mut row_8 = 72..81_usize;
    assert_indices_eq(
        &mut row_8,
        sudoku.row_cells_with_check(8)?,
        format_args!("These are not the indices for row 8"),
    );

    let realised = sudoku.row_cells_with_check(9_999);
    assert_eq!(
        SudokuError::InvalidCellIndex { cell_index: 9_999 },
        realised.err().unwrap(),
        "Expected to fail for invalid row"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn check_column_indices() -> SudokuResult {
    let sudoku = Sudoku::new();

    let check_one_column = |column: usize, result: CellIterator| {
        let start_col_index = column;
        let end_col_index = start_col_index + GRID_INDEX_MAX;
        let mut expected_indices = (start_col_index..=end_col_index).step_by(GRID_COLUMNS);
        assert_indices_eq(
            &mut expected_indices,
            result,
            format_args!("Column {} incorrect", column),
        );
    };

    for column in 0..GRID_COLUMNS {
        let result = sudoku.column_cells_with_check(column)?;
        check_one_column(column, result);
    }

    let mut column_2 = (2..=74_usize).step_by(9);
    assert_indices_eq(
        &mut column_2,
        sudoku.column_cells_with_check(2)?,
        format_args!("Column 2 should be [2, 11, 20, 29, 38, 47, 56, 65, 74]"),
    );

    let realised = sudoku.column_cells_with_check(10);
    assert_eq!(
        SudokuError::InvalidCellIndex { cell_index: 10 },
        realised.err().unwrap(),
        "Expected to fail for invalid column"
    );

    Ok(())
}

#[cfg(test)]
fn assert_indices_eq(
    expected: &mut dyn Iterator<Item = usize>,
    cells: CellIterator,
    msg: Arguments,
) {
    let expected_indices: Vec<usize> = expected.collect();
    let realised_indices: Vec<usize> = cells.map(|cell_ref| cell_ref.index).collect();

    assert_eq!(realised_indices, expected_indices, "{}", msg);
}

#[cfg(test)]
fn create_sudoku(text: &str) -> Sudoku {
    let mut sudoku = Sudoku::new();

    for (index, value) in text
        .lines()
        .flat_map(|line| line.split_ascii_whitespace())
        .enumerate()
        .filter(|(_, value)| "_".ne(*value))
    {
        if let Ok(num_value) = value.parse() {
            let result = sudoku.choose(index, num_value);
            if result.is_err() {
                assert!(false, "got choose error: {:?}", result);
            }
        } else {
            assert!(false, "Could not parse {}", value);
        }
    }

    sudoku
}
