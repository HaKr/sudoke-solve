use crate::{
    cell::Cell, CellIndices, CellOptions, CellOrigins, Cells, SudokuError, SudokuResult,
    GRID_COLUMNS, GRID_ROWS, GRID_SIZE,
};

type CellFilter<'i> = Box<dyn Fn(&&Cell) -> bool + 'i>;

pub struct Sudoku {
    cells: Cells,
    cell_indices: CellIndices,
    pub todo_count: usize,
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
            cell_indices: CellIndices::new(),
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
                    Ok(self.assign_cell_value(cell_index, value))
                } else {
                    Err(SudokuError::CannotChoose {
                        cell: cell.clone(),
                        value,
                    })
                }
            }
        }
    }

    fn assign_cell_value(&mut self, cell_index: usize, value: usize) -> usize {
        let cell = &self.cells[cell_index];

        if cell.has_value(value) {
            0
        } else {
            let mut counter = 1_usize;
            self.todo_count -= 1;

            let (row_nr, column_nr, square_nr) = {
                let mut_cell = &mut self.cells[cell_index];
                mut_cell.choose(value);
                (mut_cell.row, mut_cell.column, mut_cell.square_nr)
            };

            self.remove_options(value, row_nr, column_nr, square_nr, cell_index);

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
                counter += self.assign_cell_value(cell_index, cell_value);
            }

            counter
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

    fn remove_options(
        &mut self,
        value: usize,
        row_nr: usize,
        column_nr: usize,
        square_nr: usize,
        except_cell_index: usize,
    ) {
        {
            let row_indices = self.cell_indices.for_row(row_nr);

            for row_cell in self.cells.iter_mut().filter(|cell: &&mut Cell| {
                cell.index != except_cell_index && row_indices.contains(&cell.index)
            }) {
                row_cell.remove_option_if_available(value);
            }
        }

        {
            let column_indices = self.cell_indices.for_column(column_nr);

            for column_cell in self.cells.iter_mut().filter(|cell: &&mut Cell| {
                cell.index != except_cell_index && column_indices.contains(&cell.index)
            }) {
                column_cell.remove_option_if_available(value);
            }
        }

        {
            let square_indices = self.cell_indices.for_square(square_nr);

            for square_cell in self.cells.iter_mut().filter(|cell: &&mut Cell| {
                cell.index != except_cell_index && square_indices.contains(&cell.index)
            }) {
                square_cell.remove_option_if_available(value);
            }
        }
    }

    fn locate_single_options_by<'i>(
        &'i self,
        length: usize,
        cell_filter: fn(&'i Self, index: usize) -> CellFilter<'i>,
    ) -> Option<Choose> {
        (0..length).find_map(|row| {
            let mut group_options = GroupOptions::new();

            for cell in self.cells.iter().filter(cell_filter(&self, row)) {
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

    fn cells_of_row<'i>(&'i self, row_nr: usize) -> CellFilter<'i> {
        let row_indices: &'i [usize] = self.cell_indices.for_row(row_nr);

        Box::new(move |cell: &&Cell| row_indices.contains(&cell.index))
    }

    fn cells_of_column<'i>(&'i self, column_nr: usize) -> CellFilter<'i> {
        let column_indices: &'i [usize] = self.cell_indices.for_column(column_nr);

        Box::new(move |cell: &&Cell| column_indices.contains(&cell.index))
    }

    fn cells_of_square<'i>(&'i self, square_nr: usize) -> CellFilter<'i> {
        let square_indices: &'i [usize] = self.cell_indices.for_square(square_nr);

        Box::new(move |cell: &&Cell| square_indices.contains(&cell.index))
    }
    /*
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
    */

    #[cfg(test)]
    pub fn cell_at(&self, row: usize, column: usize) -> &Cell {
        let (row, column) = (row - 1, column - 1);
        let index = row * GRID_COLUMNS + column;
        &self.cells[index]
    }
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

#[cfg(test)]
#[test]
fn pick_one() {
    let mut sudoku = Sudoku::new();

    sudoku.assign_cell_value(10, 5);
    sudoku.assign_cell_value(20, 6);
    sudoku.assign_cell_value(18, 8);
    sudoku.assign_cell_value(0, 4);
    sudoku.assign_cell_value(2, 2);

    let s = sudoku
        .cells
        .iter()
        .filter(sudoku.cells_of_square(0))
        .map(|c| {
            let cont = match c.value {
                Some(v) => format!("={}=", v),
                None => format!(
                    "{:?}",
                    c.options
                        .iter()
                        .enumerate()
                        .filter_map(|(i, o)| if *o { Some(i) } else { None })
                        .collect::<Vec<usize>>()
                ),
            };
            format!("{:2}:[{}]", c.index, cont)
        })
        .collect::<Vec<String>>()
        .join(", ");

    assert_eq!(s, " 0:[=4=],  1:[[1, 3, 7, 9]],  2:[=2=],  9:[[1, 3, 7, 9]], 10:[=5=], 11:[[1, 3, 7, 9]], 18:[=8=], 19:[[1, 3, 7, 9]], 20:[=6=]". to_string())
}
