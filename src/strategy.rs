use bitmaps::Bitmap;
use std::collections::HashSet;
use std::ops::BitOr;

use crate::Row;

pub(crate) fn safe_to_remove(row: &Row) -> HashSet<usize> {
    let perms = row.permutations();
    let mut all = Bitmap::<8>::new();

    for (_, bitmap) in perms.into_iter().enumerate() {
        all = all.bitor(bitmap)
    }

    let mut safe = HashSet::new();
    for i in 0..row.row.len() {
        if !all.get(i) {
            safe.insert(row.row[i]);
        }
    }
    safe
}
