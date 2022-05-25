use crate::field::Field;

pub struct Game {
    field: Field,
}

impl Game {
    pub fn new(size: usize, coord_alive_cells: &[(usize, usize)]) -> Self {
        Game {
            field: Field::new(size, coord_alive_cells),
        }
    }

    pub fn next(&mut self) {
        self.field.update();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_game() {
        let _game = Game::new(10, &[]);
    }
}
