use crate::field::Field;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ALIVE_SYMBOL: &str = "x";

pub struct Game {
    field: Field,
}

impl Game {
    pub fn new(size: usize, coord_alive_cells: &[(usize, usize)]) -> Self {
        Game {
            field: Field::new(size, coord_alive_cells),
        }
    }

    pub fn new_from_file(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = match File::open(filepath) {
            Err(why) => {
                eprintln!("The file {} does not exist", filepath);
                return Err(Box::new(why));
            }
            Ok(file) => file,
        };

        let reader = BufReader::new(file);

        let mut coord_alive_cells = Vec::new();
        let mut size = 0;

        for (row, line) in reader.lines().enumerate() {
            size = row + 1;
            for (column, symbol) in line.unwrap().split(' ').enumerate() {
                if symbol == ALIVE_SYMBOL {
                    coord_alive_cells.push((row, column));
                }
            }
        }

        Ok(Game::new(size, &coord_alive_cells))
    }

    pub fn next(&mut self) {
        self.field.update();
    }

    pub fn coord_alive_cells(&self) -> Vec<(usize, usize)> {
        self.field.coord_alive_cells()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_game() {
        let _game = Game::new(10, &[]);
    }

    #[test]
    fn fails_when_file_does_not_exist() {
        if let Ok(_game) = Game::new_from_file("does_not_exist.txt") {
            panic! {"It did not return an error when file does not exist"};
        }
    }
}
