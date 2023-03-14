use std::fmt;

use crate::cell::Cell;
use crate::cell_status::CellStatus;

pub enum CellGroupLocation {
    Row(usize),
    Column(usize),
}

impl fmt::Display for CellGroupLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellGroupLocation::Column(i) => write!(f, "Column({})", i),
            CellGroupLocation::Row(i) => write!(f, "Row({})", i),
        }
    }
}

pub struct CellGroup {
    pub location: CellGroupLocation,
    pub cells: Vec<Cell>,
}

impl CellGroup {
    pub fn new(location: CellGroupLocation, cells: Vec<Cell>) -> CellGroup {
        CellGroup { location, cells }
    }
}

impl fmt::Display for CellGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "[ ".to_string();
        for cell in &self.cells {
            output.push_str(&cell.to_string());
            output.push_str(" ");
        }
        output.push_str("]");
        write!(f, "{}", output)
    }
}
