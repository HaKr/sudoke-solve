pub const GRID_SQUARE_SIDE: usize = 3;
pub const GRID_COLUMNS: usize = GRID_SQUARE_SIDE * GRID_SQUARE_SIDE;
pub const GRID_ROWS: usize = GRID_COLUMNS;
pub const GRID_SIZE: usize = GRID_ROWS * GRID_COLUMNS;
pub const GRID_INDEX_MAX: usize = (GRID_ROWS - 1) * GRID_COLUMNS + 1;

pub type CellOptions = [bool; GRID_COLUMNS + 1];
pub type CellOrigins = [Option<usize>; GRID_COLUMNS + 1];
pub type SudokuResult<RT = ()> = Result<RT, SudokuError>;
use crate::Cell;
pub type Cells = [Cell; GRID_SIZE];

#[derive(Debug, PartialEq)]
pub enum SudokuError {
    CannotChoose { cell: Cell, value: usize },
    InvalidCellIndex { cell_index: usize },
    IllegalValue { value: usize },
}
