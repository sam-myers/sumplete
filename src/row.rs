use bitmaps::Bitmap;

use crate::permutations::row_permutations;
use crate::strategy::safe_to_remove;

pub struct Row {
    pub sum: usize,
    pub row: Vec<usize>,
}

impl Row {
    pub fn permutations(&self) -> Vec<Bitmap<8>> {
        row_permutations(self.sum, &self.row)
    }

    pub fn safe_to_remove(&self) -> Vec<usize> {
        safe_to_remove(self)
    }

    pub fn show_bitmap(&self, bitmap: Bitmap<8>) -> String {
        let mut output = String::new();
        output.push('[');
        for (i, value) in self.row.iter().enumerate() {
            if bitmap.get(i) {
                output.push_str(" <");
                output.push_str(&value.to_string());
                output.push_str("> ");
            } else {
                output.push_str("  ");
                output.push_str(&value.to_string());
                output.push_str("  ");
            }
        }
        output.push(']');
        output
    }
}
