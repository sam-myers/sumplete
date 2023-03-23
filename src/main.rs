mod cell;
mod cell_group;
mod cell_group_location;
mod cell_selection;
mod cell_status;

use crate::cell::Cell;
use crate::cell_group::CellGroup;
use crate::cell_group_location::CellGroupLocation;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let _log = slog::Logger::root(drain, o!());

    let mut row = CellGroup::new(
        CellGroupLocation::Row(0),
        vec![
            Cell::new(8),
            Cell::new(2),
            Cell::new(5),
            Cell::new(7),
            Cell::new(5),
            Cell::new(3),
        ],
        25,
    );

    info!(_log, "{}", row);
    row = row.solve();
    info!(_log, "{}", row);
}
