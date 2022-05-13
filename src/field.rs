use crate::cell::Cell;

pub struct Field<const N: usize> {
    cells: [[Cell; N]; N],
}

impl<const N: usize> Field<N> {
    pub fn new(cells: [[Cell; N]; N]) -> Self {
        Field { cells: cells }
    }

    pub fn cells(&self) -> &[[Cell; N]; N] {
        &self.cells
    }

    pub fn bring_life_at(&mut self, coordinates: (usize, usize)) {
        self.cells
            .get_mut(coordinates.0)
            .expect("First coordinate out of bounds")
            .get_mut(coordinates.1)
            .expect("Second coordinate out of bounds")
            .bring_to_life();
    }

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
        for (i, row) in self.cells.iter_mut().skip(1).take(N - 2).enumerate() {
            for (j, cell) in row.iter_mut().skip(1).take(N - 2).enumerate() {
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

impl Default for Field<1> {
    fn default() -> Self {
        Field { cells: [[Cell::default(); 1]; 1] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;

    #[test]
    fn can_create_field() {
        let cells = [[Cell::default(); 10]; 10];
        let _field = Field::new(cells);
    }

    #[test]
    fn can_bring_cells_to_life() {
        let coord_alive_cells = [(2, 2), (3, 1)];
        let cells = [[Cell::default(); 10]; 10];
        let mut field = Field::new(cells);
        for coord in coord_alive_cells {
            field.bring_life_at(coord);
            assert!(field.is_cell_alive_at(coord));
        }
    }

    #[test]
    fn can_update_num_neighbors() {
        let coord_alive_cells = [(2, 2), (3, 1)];
        let cells = [[Cell::default(); 6]; 6];
        let mut field = Field::new(cells);

        for coord in coord_alive_cells {
            field.bring_life_at(coord);
        }

        field.update_num_neighbors();
        let cells = field.cells();

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
            assert_eq!(1, cells[coord.0][coord.1].num_neighbors);
        }

        for coord in coord_two_neigh {
            assert_eq!(2, cells[coord.0][coord.1].num_neighbors);
        }
    }

    #[test]
    fn can_update_field() {
        let coord_alive_cells = [(1, 2), (2, 1), (2, 2), (2, 4)];
        let cells = [[Cell::default(); 6]; 6];

        let mut field = Field::new(cells);

        for coord in coord_alive_cells {
            field.bring_life_at(coord);
        }

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
}
