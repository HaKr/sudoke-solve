use sudoku_solver::{Sudoku, SudokuError};

fn main() -> Result<(), SudokuError> {
    const SAMPLE: &str = include_str!("../examples/gelderlander-20220308.txt");
    // const SAMPLE: &str = include_str!("../examples/heuristic_2_1.txt");
    let mut puzzle = Sudoku::new();
    for (index, value) in SAMPLE
        .lines()
        .flat_map(|line| line.split_ascii_whitespace())
        .enumerate()
        .filter(|(_, value)| "_".ne(*value))
    {
        if let Ok(num_value) = value.parse() {
            let result = puzzle.choose(index, num_value);
            match result {
                Ok(_) => (),
                Err(e) => println!("Err {:?}", e),
            }
        } else {
            println!("Could not parse {}", value);
        }
    }

    let count = puzzle.solve()?;

    // println!("Solved in {} iterations\n{}", count, puzzle);
    Ok(())
}
