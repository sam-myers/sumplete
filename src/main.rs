mod permutations;
mod row;
mod strategy;

use row::Row;

fn main() {
    let row = Row {
        row: vec![1, 2, 5, 5],
        sum: 10,
    };

    for bitmap in row.permutations() {
        println!("Possibility: {}", row.show_bitmap(bitmap));
    }

    for value in row.safe_to_remove() {
        println!("Safe to remove: {}", value);
    }
}
