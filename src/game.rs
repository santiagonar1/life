use crate::field::Field;

pub struct Game<const N: usize> {
    field: Field<N>,
}

impl<const N: usize> Game<N> {
    pub fn new(field: Field<N>) -> Self {
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
        let field = Field::default();
        let _game = Game::new(field);
    }
}