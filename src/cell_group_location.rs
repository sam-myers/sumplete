use std::fmt;

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
