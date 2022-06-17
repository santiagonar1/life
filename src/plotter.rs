use crate::field::{Field, Size};
use plotters::prelude::*;

#[derive(Clone)]
pub struct Plotter<'a> {
    base_filename: &'a str,
    num_plots: u32,
}

impl<'a> Plotter<'a> {
    pub fn new(base_path: &'a str) -> Self {
        Plotter {
            base_filename: base_path,
            num_plots: 0,
        }
    }

    pub fn plot(&mut self, field: &Field) -> Result<(), Box<dyn std::error::Error>> {
        let filename = self.base_filename.to_string() + &self.num_plots.to_string();
        let filename = filename + ".png";
        let root_drawing_area = BitMapBackend::new(&filename, (300, 200)).into_drawing_area();
        root_drawing_area.fill(&Palette99::pick(19))?;

        let child_drawing_areas = root_drawing_area.split_evenly(field.dimensions());
        let rows = field.rows();
        for (row, column) in field.coord_alive_cells() {
            let id = row * rows + column;
            child_drawing_areas[id].fill(&Palette99::pick(2))?;
        }
        self.num_plots += 1;
        Ok(())
    }
}
