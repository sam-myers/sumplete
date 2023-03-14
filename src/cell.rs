use std::fmt;

use crate::cell_status::CellStatus;

#[derive(Debug, PartialEq, Hash)]
pub struct Cell {
    pub value: usize,
    pub status: CellStatus,
}

impl Cell {
    pub(crate) fn new(value: usize) -> Cell {
        Cell {
            value,
            status: CellStatus::Unknown,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.status, self.value)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_new_equality() {
        assert_eq!(
            Cell::new(1),
            Cell {
                status: CellStatus::Unknown,
                value: 1
            }
        );
    }

    #[test]
    fn test_print() {
        assert_eq!(
            format!(
                "{}",
                Cell {
                    status: CellStatus::Unknown,
                    value: 1
                }
            ),
            "?1"
        );
        assert_eq!(
            format!(
                "{}",
                Cell {
                    status: CellStatus::Known,
                    value: 2
                }
            ),
            "O2"
        );
        assert_eq!(
            format!(
                "{}",
                Cell {
                    status: CellStatus::Removed,
                    value: 3
                }
            ),
            "X3"
        );
    }
}
