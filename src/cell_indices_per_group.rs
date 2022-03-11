use crate::{GRID_COLUMNS, GRID_INDEX_MAX, GRID_ROWS, GRID_SQUARE_SIDE};

type Rows = [[usize; GRID_COLUMNS]; GRID_ROWS];
type Columns = [[usize; GRID_ROWS]; GRID_COLUMNS];
type Squares = [[usize; GRID_COLUMNS]; GRID_COLUMNS];

#[allow(dead_code)]
#[derive(Debug)]
pub struct CellIndices {
    rows: Rows,
    columns: Columns,
    squares: Squares,
}

impl CellIndices {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn for_row(&self, row_nr: usize) -> &[usize] {
        &self.rows[row_nr]
    }

    #[allow(dead_code)]
    pub fn for_column(&self, column_nr: usize) -> &[usize] {
        &self.columns[column_nr]
    }

    #[allow(dead_code)]
    pub fn for_square(&self, square_nr: usize) -> &[usize] {
        &self.squares[square_nr]
    }
}

impl Default for CellIndices {
    fn default() -> Self {
        let mut rows = Default::default();
        let mut columns = Default::default();
        let mut squares = Default::default();

        fill_rows(&mut rows);
        fill_columns(&mut columns);
        fill_squares(&mut squares);

        Self {
            rows,
            columns,
            squares,
        }
    }
}

#[inline]
fn fill_rows(rows: &mut Rows) {
    for (row_nr, row_indices) in rows.iter_mut().enumerate() {
        let start_row_index = row_nr * GRID_COLUMNS;
        let end_row_index = start_row_index + GRID_COLUMNS;

        for (row_cell, row_index) in row_indices.iter_mut().zip(start_row_index..end_row_index) {
            *row_cell = row_index;
        }
    }
}

#[inline]
fn fill_columns(columns: &mut Columns) {
    for (column_nr, column_indices) in columns.iter_mut().enumerate() {
        let start_col_index = column_nr;
        let end_col_index = start_col_index + GRID_INDEX_MAX;

        for (column_cell, column_index) in column_indices
            .iter_mut()
            .zip((start_col_index..=end_col_index).step_by(GRID_COLUMNS))
        {
            *column_cell = column_index;
        }
    }
}

#[inline]
fn fill_squares(squares: &mut Squares) {
    for (square_nr, square_indices) in squares.iter_mut().enumerate() {
        let square_start_index =
            (square_nr / 3) * 3 * GRID_COLUMNS + (square_nr % 3) * GRID_SQUARE_SIDE;

        for (square_cell, square_index) in square_indices.iter_mut().zip(
            (square_start_index..(square_start_index + GRID_SQUARE_SIDE))
                .chain(
                    (square_start_index + GRID_COLUMNS)
                        ..(square_start_index + GRID_COLUMNS + GRID_SQUARE_SIDE),
                )
                .chain(
                    (square_start_index + GRID_COLUMNS + GRID_COLUMNS)
                        ..(square_start_index + GRID_COLUMNS + GRID_COLUMNS + GRID_SQUARE_SIDE),
                ),
        ) {
            *square_cell = square_index;
        }
    }
}

#[cfg(test)]
#[test]
fn grid_indices() {
    let grid = CellIndices::new();
    println!("Grid {:?}", grid)
}
