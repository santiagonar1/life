use crate::field::Field;

pub struct Game {
    field: Field,
}

impl Game {
    pub fn new(field: Field) -> Self {
        Game { field }
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
        let field: Field = Field::new(6, &[]);
        let _game = Game::new(field);
    }
}