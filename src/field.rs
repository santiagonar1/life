use crate::cell::Cell;

pub struct Field {
    cells: Vec<Vec<Cell>>,
}

impl Field {
    pub fn new(size: usize, coord_alive_cells: &[(usize, usize)]) -> Self {
        let mut cells = vec![vec![Cell::default(); size]; size];
        for coord_alive in coord_alive_cells {
            cells[coord_alive.0][coord_alive.1].bring_to_life();
        }

        Field { cells }
    }

    #[allow(dead_code)]
    pub fn is_cell_alive_at(&self, coordinates: (usize, usize)) -> bool {
        self.cells
            .get(coordinates.0)
            .expect("First coordinate out of bounds")
            .get(coordinates.1)
            .expect("Second coordinate out of bounds")
            .is_alive()
    }

    fn update_num_neighbors(&mut self) {
        let old_cells = self.cells.clone();
        let size = self.cells.len();
        for (i, row) in self.cells.iter_mut().skip(1).take(size - 2).enumerate() {
            for (j, cell) in row.iter_mut().skip(1).take(size - 2).enumerate() {
                let i = i + 1;
                let j = j + 1;

                cell.consider_neighbor(&old_cells[i - 1][j - 1]);
                cell.consider_neighbor(&old_cells[i - 1][j]);
                cell.consider_neighbor(&old_cells[i - 1][j + 1]);

                cell.consider_neighbor(&old_cells[i][j - 1]);
                cell.consider_neighbor(&old_cells[i][j + 1]);

                cell.consider_neighbor(&old_cells[i + 1][j - 1]);
                cell.consider_neighbor(&old_cells[i + 1][j]);
                cell.consider_neighbor(&old_cells[i + 1][j + 1]);
            }
        }
    }

    pub fn update(&mut self) {
        self.update_num_neighbors();
        self.cells
            .iter_mut()
            .flatten()
            .for_each(|cell| cell.update_state());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn can_create_field() {
        let _field: Field = Field::new(5, &[]);
    }

    #[test]
    fn can_update_num_neighbors() {
        let coord_alive_cells = [(2, 2), (3, 1)];
        let mut field: Field = Field::new(6, &coord_alive_cells);

        field.update_num_neighbors();

        let coord_one_neigh = [
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 2),
            (2, 3),
            (3, 1),
            (3, 3),
            (4, 1),
            (4, 2),
        ];
        let coord_two_neigh = [(2, 1), (3, 2)];

        for coord in coord_one_neigh {
            assert_eq!(1, field.cells[coord.0][coord.1].num_neighbors);
        }

        for coord in coord_two_neigh {
            assert_eq!(2, field.cells[coord.0][coord.1].num_neighbors);
        }
    }

    #[test]
    fn can_update_field() {
        let coord_alive_cells = [(1, 2), (2, 1), (2, 2), (2, 4)];

        let mut field: Field = Field::new(7, &coord_alive_cells);

        field.update();

        let coord_alive_cells = [(1, 1), (1, 2), (2, 1), (2, 2), (2, 3)];
        for coord in coord_alive_cells {
            assert!(field.cells[coord.0][coord.1].is_alive());
        }

        let coord_new_dead_cells = [(2, 4)];
        for coord in coord_new_dead_cells {
            assert!(!field.cells[coord.0][coord.1].is_alive());
        }
    }

    #[test]
    fn aircraft_carrier() {
        // See: https://playgameoflife.com/lexicon/aircraft_carrier
        let coord_alive_cells: Vec<(usize, usize)> =
            vec![(1, 1), (1, 2), (2, 1), (2, 4), (3, 3), (3, 4)];
        let coord_space: Vec<(usize, usize)> = (0..=5)
            .permutations(2)
            .collect_vec()
            .iter_mut()
            .map(|coord| (coord[0], coord[1]))
            .collect();

        let mut coord_dead_cells = coord_space.clone();
        coord_dead_cells.retain(|coord| !coord_alive_cells.contains(coord));

        let mut field: Field = Field::new(6, &coord_alive_cells);

        field.update();

        for coord in coord_alive_cells {
            assert!(field.cells[coord.0][coord.1].is_alive());
        }

        for coord in coord_dead_cells {
            assert!(!field.cells[coord.0][coord.1].is_alive());
        }
    }
}
