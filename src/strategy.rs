use bitmaps::Bitmap;
use std::ops::BitOr;

use crate::Row;

pub(crate) fn safe_to_remove(row: &Row) -> Vec<usize> {
    let perms = row.permutations();
    let mut all = Bitmap::<8>::new();

    for (_, bitmap) in perms.into_iter().enumerate() {
        all = all.bitor(bitmap)
    }

    let mut safe = Vec::new();
    for i in 0..row.row.len() {
        if !all.get(i) {
            safe.push(row.row[i]);
        }
    }
    safe
}
