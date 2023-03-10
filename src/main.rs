mod permutations;

use permutations::{row_permutations, row_bitmap_debug};

fn main() {
    let sum = 16;
    let row = &vec![5, 9, 7, 3, 6, 1, 5];
    for bitmap in row_permutations(sum, row) {
        println!("{}", row_bitmap_debug(row, bitmap));
    }
}
