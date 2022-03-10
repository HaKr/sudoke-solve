use std::slice::{Iter, IterMut};

use crate::{Cell, GRID_COLUMNS, GRID_SQUARE_SIDE};

pub(crate) struct CellIterator<'iter> {
    cells: Iter<'iter, Cell>,
    cell_range: CellRange,
}

impl<'iter> CellIterator<'iter> {
    pub(crate) fn new<'i>(cells: &'i [Cell], cell_range: CellRange) -> Self
    where
        'i: 'iter,
    {
        Self {
            cells: cells.iter(),
            cell_range,
        }
    }
}

impl<'iter> Iterator for CellIterator<'iter> {
    type Item = &'iter Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells
            .find(|cell_ref| self.cell_range.contains(cell_ref.index))
    }
}

pub(crate) struct CellIteratorMut<'iter> {
    cells: IterMut<'iter, Cell>,
    cell_range: CellRange,
}

impl<'iter> CellIteratorMut<'iter> {
    #[allow(dead_code)]
    pub(crate) fn new<'i>(cells: &'i mut [Cell], cell_range: CellRange) -> Self
    where
        'i: 'iter,
    {
        Self {
            cells: cells.iter_mut(),
            cell_range,
        }
    }
}

impl<'iter> Iterator for CellIteratorMut<'iter> {
    type Item = &'iter mut Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells
            .find(|cell_ref| self.cell_range.contains(cell_ref.index))
    }
}

pub struct CellLinearDefinition {
    pub start_cell_index: usize,
    pub end_cell_index: usize,
    pub step_by: usize,
}

pub struct CellRange {
    cell_indices: Vec<usize>,
}

impl CellRange {
    pub(crate) fn from_linear(definition: CellLinearDefinition) -> Self {
        Self {
            cell_indices: if definition.step_by > 1 {
                (definition.start_cell_index..definition.end_cell_index)
                    .step_by(definition.step_by)
                    .collect()
            } else {
                (definition.start_cell_index..definition.end_cell_index).collect()
            },
        }
    }

    pub(crate) fn from_square_nr(square_nr: usize) -> Self {
        let square_nr = square_nr;
        let square_start_index =
            (square_nr / 3) * 3 * GRID_COLUMNS + (square_nr % 3) * GRID_SQUARE_SIDE;

        Self {
            cell_indices: (square_start_index..(square_start_index + GRID_SQUARE_SIDE))
                .chain(
                    (square_start_index + GRID_COLUMNS)
                        ..(square_start_index + GRID_COLUMNS + GRID_SQUARE_SIDE),
                )
                .chain(
                    (square_start_index + GRID_COLUMNS + GRID_COLUMNS)
                        ..(square_start_index + GRID_COLUMNS + GRID_COLUMNS + GRID_SQUARE_SIDE),
                )
                .collect(),
        }
    }

    fn contains(&self, cell_index: usize) -> bool {
        self.cell_indices.contains(&cell_index)
    }
}
