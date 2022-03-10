use crate::{
    cell::Cell, cell_iterators::CellIteratorMut, CellIterator, CellLinearDefinition, CellOptions,
    CellOrigins, CellRange, Cells, SudokuError, SudokuResult, GRID_COLUMNS, GRID_INDEX_MAX,
    GRID_ROWS, GRID_SIZE, GRID_SQUARE_SIDE,
};

pub struct Sudoku {
    cells: Cells,
    pub todo_count: usize,
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = Vec::<String>::new();
        let mut cells = Vec::<String>::new();

        for cell in self.cells.iter() {
            cells.push(format!("{}", cell));
            if cell.is_rightmost() {
                rows.push(cells.join(" | "));
                cells = Vec::<String>::new();
            }
        }

        let grid = format!("\n\t{}\n", rows.join("\n\t"));

        f.write_fmt(format_args!(
            "Puzzle\n\ttodo_count: {}\n\tcells: {}",
            self.todo_count, grid
        ))

        // f.debug_struct("Puzzle")
        //     // .field("cells", &self.cells)
        //     .field("todo_count", &self.todo_count)
        //     .field("cells", &grid)
        //     .finish()
    }
}

impl Sudoku {
    pub fn new() -> Self {
        let mut cells: Cells = [Cell::default(); GRID_SIZE];
        for (cell, index) in cells.iter_mut().zip(0..GRID_SIZE) {
            let row = index / GRID_COLUMNS;
            let column = index % GRID_COLUMNS;
            let square_row_index = row / 3;
            let square_col_index = column / 3;

            *cell = Cell::new(index, row, column, square_row_index * 3 + square_col_index)
        }
        Self {
            cells,
            todo_count: GRID_SIZE,
        }
    }

    pub fn choose(&mut self, cell_index: usize, value: usize) -> SudokuResult<usize> {
        if cell_index >= GRID_SIZE {
            Err(SudokuError::InvalidCellIndex { cell_index })
        } else if value < 1 || value > GRID_COLUMNS {
            Err(SudokuError::InvalidCellIndex { cell_index })
        } else {
            let cell = &self.cells[cell_index];
            if cell.has_value(value) {
                Ok(0)
            } else {
                if cell.can_choose(value) {
                    let mut counter = 1_usize;
                    self.todo_count -= 1;
                    {
                        let mut_cell = &mut self.cells[cell_index];
                        mut_cell.choose(value);
                    }
                    self.remove_options(cell_index);

                    let solutions: Vec<(usize, usize)> = self
                        .cells
                        .iter()
                        .enumerate()
                        .filter_map(|(cell_index, cell)| match cell.solution() {
                            Some(value) => Some((cell_index, value)),
                            _ => None,
                        })
                        .collect();
                    for (cell_index, cell_value) in solutions {
                        counter += self.choose(cell_index, cell_value)?;
                    }
                    Ok(counter)
                } else {
                    Err(SudokuError::CannotChoose {
                        cell: cell.clone(),
                        value,
                    })
                }
            }
        }
    }

    pub fn solve(&mut self) -> SudokuResult<usize> {
        let mut count: usize = 0;
        while let Some(choice) = self.locate_single_options() {
            count += self.choose(choice.cell_index, choice.cell_value)?;
        }
        Ok(count)
    }

    fn locate_single_options(&self) -> Option<Choose> {
        match self.locate_single_options_by_row() {
            None => match self.locate_single_options_by_column() {
                None => self.locate_single_options_by_square(),
                result => result,
            },
            result => result,
        }
    }

    #[allow(dead_code)]
    fn remove_options2(&mut self, cell: &mut Cell, value: usize) {
        cell.choose(value);
        for square_cell in self
            .cells_of_square_mut(cell.square_nr)
            .filter(|candidate| cell.index != candidate.index)
        {
            square_cell.remove_option_if_available(value);
        }

        for square_cell in self
            .cells_of_square_mut(cell.square_nr)
            .filter(|candidate| cell.index != candidate.index)
        {
            square_cell.remove_option_if_available(value);
        }
    }

    fn remove_options(&mut self, cell_index: usize) {
        let cell = &mut self.cells[cell_index];
        let value = cell.value.unwrap();
        let start_row_index = cell.row * GRID_COLUMNS;
        let end_row_index = start_row_index + GRID_COLUMNS;
        let start_col_index = cell.column;
        let end_col_index = start_col_index + GRID_INDEX_MAX;
        let square_nr = cell.square_nr;
        let square_start_index =
            (square_nr / 3) * 3 * GRID_COLUMNS + (square_nr % 3) * GRID_SQUARE_SIDE;
        let square_cells = (square_start_index..(square_start_index + GRID_SQUARE_SIDE))
            .chain(
                (square_start_index + GRID_COLUMNS)
                    ..(square_start_index + GRID_COLUMNS + GRID_SQUARE_SIDE),
            )
            .chain(
                (square_start_index + GRID_COLUMNS + GRID_COLUMNS)
                    ..(square_start_index + GRID_COLUMNS + GRID_COLUMNS + GRID_SQUARE_SIDE),
            );

        for cell_index in (start_col_index..=end_col_index)
            .step_by(GRID_COLUMNS)
            .filter(|index| *index != cell_index)
        {
            let cell = &mut self.cells[cell_index];
            cell.remove_option_if_available(value);
        }

        for cell_index in (start_row_index..end_row_index).filter(|index| *index != cell_index) {
            let cell = &mut self.cells[cell_index];
            cell.remove_option_if_available(value);
        }

        for cell_index in square_cells.filter(|index| *index != cell_index) {
            let cell = &mut self.cells[cell_index];
            cell.remove_option_if_available(value);
        }
    }

    fn locate_single_options_by(
        &self,
        length: usize,
        cells: fn(&Self, index: usize) -> CellIterator,
    ) -> Option<Choose> {
        (0..length).find_map(|row| {
            let mut group_options = GroupOptions::new();

            for cell in cells(&self, row) {
                if cell.has_options() {
                    group_options.xor(&cell.options, cell.index)
                }
            }

            group_options.single_option()
        })
    }

    fn locate_single_options_by_row(&self) -> Option<Choose> {
        self.locate_single_options_by(GRID_ROWS, Sudoku::cells_of_row)
    }

    fn locate_single_options_by_column(&self) -> Option<Choose> {
        self.locate_single_options_by(GRID_COLUMNS, Sudoku::cells_of_column)
    }

    fn locate_single_options_by_square(&self) -> Option<Choose> {
        self.locate_single_options_by(GRID_COLUMNS, Sudoku::cells_of_square)
    }

    fn cells_of_row(&self, row: usize) -> CellIterator {
        let start_cell_index = row * GRID_COLUMNS;
        CellIterator::new(
            &self.cells,
            CellRange::from_linear(CellLinearDefinition {
                start_cell_index,
                end_cell_index: start_cell_index + GRID_COLUMNS,
                step_by: 1,
            }),
        )
    }

    fn cells_of_column(&self, column: usize) -> CellIterator {
        let start_cell_index = column;

        CellIterator::new(
            &self.cells,
            CellRange::from_linear(CellLinearDefinition {
                start_cell_index,
                end_cell_index: start_cell_index + GRID_INDEX_MAX,
                step_by: GRID_COLUMNS,
            }),
        )
    }

    fn cells_of_square(&self, square: usize) -> CellIterator {
        CellIterator::new(&self.cells, CellRange::from_square_nr(square))
    }

    fn cells_of_square_mut(&mut self, square: usize) -> CellIteratorMut {
        CellIteratorMut::new(&mut self.cells, CellRange::from_square_nr(square))
    }

    #[cfg(test)]
    pub(crate) fn row_cells_with_check(&self, row: usize) -> SudokuResult<CellIterator> {
        if row >= GRID_ROWS {
            Err(SudokuError::InvalidCellIndex { cell_index: row })
        } else {
            Ok(self.cells_of_row(row))
        }
    }

    #[cfg(test)]
    pub(crate) fn column_cells_with_check(&self, column: usize) -> SudokuResult<CellIterator> {
        if column >= GRID_COLUMNS {
            Err(SudokuError::InvalidCellIndex { cell_index: column })
        } else {
            Ok(self.cells_of_column(column))
        }
    }

    #[cfg(test)]
    pub fn cell_at(&self, row: usize, column: usize) -> &Cell {
        let (row, column) = (row - 1, column - 1);
        let index = row * GRID_COLUMNS + column;
        &self.cells[index]
    }
}

#[derive(Debug)]
struct GroupOptions {
    options: CellOptions,
    set_from: CellOrigins,
}

impl Default for GroupOptions {
    fn default() -> Self {
        Self {
            options: Default::default(),
            set_from: Default::default(),
        }
    }
}

impl GroupOptions {
    fn new() -> Self {
        GroupOptions::default()
    }

    fn xor(&mut self, other: &CellOptions, cell_index: usize) {
        other
            .iter()
            .zip(self.set_from.iter_mut().zip(self.options.iter_mut()))
            .filter(|(other, _)| **other)
            .for_each(|(_, (used, option))| match *used {
                Some(_) => *option = false,
                None => {
                    *option = true;
                    *used = Some(cell_index)
                }
            });
    }

    fn single_option(&self) -> Option<Choose> {
        self.options
            .iter()
            .enumerate()
            .zip(self.set_from.iter())
            .find_map(|((cell_value, is_single), cell_index)| {
                if *is_single {
                    Some(Choose {
                        cell_index: cell_index.unwrap(),
                        cell_value,
                    })
                } else {
                    None
                }
            })
    }
}

struct Choose {
    cell_index: usize,
    cell_value: usize,
}
