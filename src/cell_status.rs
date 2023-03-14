use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CellStatus {
    Unknown,
    Known,
    Removed,
}

impl fmt::Display for CellStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellStatus::Unknown => write!(f, "?"),
            CellStatus::Known => write!(f, "O"),
            CellStatus::Removed => write!(f, "X"),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_print() {
        assert_eq!(format!("{}", CellStatus::Unknown), "?");
        assert_eq!(format!("{}", CellStatus::Known), "O");
        assert_eq!(format!("{}", CellStatus::Removed), "X");
    }
}
