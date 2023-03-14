mod cell;
mod cell_group;
mod cell_group_location;
mod cell_selection;
mod cell_status;

use crate::cell::Cell;
use crate::cell_group::CellGroup;
use crate::cell_group_location::CellGroupLocation;

fn main() {
    let mut row = CellGroup::new(
        CellGroupLocation::Row(0),
        vec![Cell::new(1), Cell::new(2), Cell::new(5), Cell::new(5)],
        10,
    );

    row = row.solve();
    println!("{}", row);
}
