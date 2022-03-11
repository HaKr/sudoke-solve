use crate::api::{CellOptions, GRID_COLUMNS};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub index: usize,
    pub column: usize,
    pub row: usize,
    pub square_nr: usize,
    pub value: Option<usize>,
    pub options: CellOptions,
    pub todo_count: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            index: Default::default(),
            column: Default::default(),
            row: Default::default(),
            square_nr: Default::default(),
            value: Default::default(),
            options: Default::default(),
            todo_count: 0,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self.value {
            Some(val) => format!("{}", val),
            None => match self.value {
                Some(_) => "".to_string(),
                None => {
                    let os: Vec<String> = self
                        .options
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, o)| {
                                if *o {
                                    Some(format!("{}", index))
                                } else {
                                    None
                                }
                            },
                        )
                        .collect();
                    format!("({})", os.join(","))
                }
            },
        };

        f.write_fmt(format_args!("{:2}: {:13}", self.index, v))
    }
}

impl Cell {
    pub(crate) fn new(index: usize, row: usize, column: usize, square_nr: usize) -> Self {
        Self {
            index,
            column,
            row,
            square_nr,
            value: None,
            options: [false, true, true, true, true, true, true, true, true, true],
            todo_count: GRID_COLUMNS,
        }
    }

    pub(crate) fn can_choose(&self, value: usize) -> bool {
        self.options[value]
    }

    pub(crate) fn has_value(&self, value: usize) -> bool {
        self.value.unwrap_or(0) == value
    }

    pub(crate) fn has_options(&self) -> bool {
        self.value.is_none()
    }

    pub(crate) fn is_rightmost(&self) -> bool {
        self.column == (GRID_COLUMNS - 1)
    }

    // pub(crate) fn has_index(&self, index: Option<usize>) -> bool {
    //     match index {
    //         Some(index) => self.index == index,
    //         None => false,
    //     }
    // }

    pub(crate) fn remove_option_if_available(&mut self, value: usize) {
        if self.options[value] && self.todo_count > 0 {
            self.options[value] = false;
            self.todo_count -= 1;
        }
    }

    /// Determines whether there is only one option left and return that as the value
    pub(crate) fn solution(&self) -> Option<usize> {
        if self.todo_count == 1 {
            self.options
                .iter()
                .enumerate()
                .find_map(|(index, option)| if *option { Some(index) } else { None })
        } else {
            None
        }
    }

    pub(crate) fn choose(&mut self, value: usize) {
        self.options.iter_mut().for_each(|o| *o = false);
        self.value = Some(value);
        self.todo_count = 0;
    }
}
