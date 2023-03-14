use bitmaps::Bitmap;
use std::ops::BitOr;

#[derive(Debug, Clone, Copy)]
pub struct CellSelection(Bitmap<8>);

impl CellSelection {
    pub fn new() -> CellSelection {
        CellSelection(Bitmap::<8>::new())
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
}
