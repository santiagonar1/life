#[derive(PartialEq)]
enum State {
    Alive,
    Dead,
}

pub struct Cell {
    state: State,
    pub num_neighbors: i8,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            state: State::Dead,
            num_neighbors: 0,
        }
    }

    pub fn update_state(&mut self) {
        if self.num_neighbors == 2 || self.num_neighbors == 3 {
            self.state = State::Alive;
        } else {
            self.state = State::Dead;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == State::Alive
    }

    pub fn bring_to_life(&mut self) {
        self.state = State::Alive;
    }

    pub fn kill(&mut self) {
        self.state = State::Dead;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_cell() {
        let _cell = Cell::new();
    }

    #[test]
    fn can_update_cell_state() {
        let mut cell = Cell::new();
        assert!(!cell.is_alive());

        let num_neighbors_to_live = [2, 3];
        for num_neighbors in num_neighbors_to_live {
            cell.num_neighbors = num_neighbors;
            cell.update_state();
            assert!(cell.is_alive());
        }


        let num_neighbors_to_die = [1, 4, 5, 6, 7, 8, 9];
        for num_neighbors in num_neighbors_to_die {
            cell.num_neighbors = num_neighbors;
            cell.update_state();
            assert!(!cell.is_alive());
        }
    }

    #[test]
    fn can_bring_cell_to_life() {
        let mut cell = Cell::new();
        cell.bring_to_life();
        assert!(cell.is_alive());
    }

    #[test]
    fn can_kill_a_cell() {
        let mut cell = Cell::new();
        cell.bring_to_life();
        assert!(cell.is_alive());
        cell.kill();
        assert!(!cell.is_alive());
    }
}