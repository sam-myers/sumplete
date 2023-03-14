use bitmaps::Bitmap;
use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellSelection(Bitmap<8>);

impl CellSelection {
    pub fn new() -> CellSelection {
        CellSelection(Bitmap::<8>::new())
    }

    pub fn new_full() -> CellSelection {
        let mut cell = CellSelection(Bitmap::<8>::new());
        for i in 0..8 {
            cell.0.set(i, true);
        }
        cell
    }

    pub fn get(&self, index: usize) -> bool {
        self.0.get(index)
    }

    pub fn set(&mut self, index: usize, value: bool) {
        self.0.set(index, value);
    }

    pub fn bitor(&mut self, &other: &CellSelection) -> CellSelection {
        CellSelection(self.0.bitor(other.0))
    }

    pub fn bitand(&mut self, &other: &CellSelection) -> CellSelection {
        CellSelection(self.0.bitand(other.0))
    }
}
