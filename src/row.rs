use bitmaps::Bitmap;
use std::collections::HashSet;

use crate::strategy::safe_to_remove;

pub struct Row {
    pub sum: usize,
    pub row: Vec<usize>,
}

impl Row {
    pub fn permutations(&self) -> Vec<Bitmap<8>> {
        Row::permutations_recur(&self.row, self.sum, Bitmap::<8>::new(), 0).unwrap()
    }

    pub fn safe_to_remove(&self) -> HashSet<usize> {
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

    fn permutations_recur(
        row: &[usize],
        target_sum: usize,
        bitmap: Bitmap<8>,
        current_index: usize,
    ) -> Option<Vec<Bitmap<8>>> {
        let mut results = Vec::new();

        match Row::sum(bitmap, row).cmp(&target_sum) {
            std::cmp::Ordering::Greater => return None,
            std::cmp::Ordering::Equal => {
                results.push(bitmap);
                return Some(results);
            }
            std::cmp::Ordering::Less => (),
        }

        if current_index >= row.len() {
            return None;
        }

        let mut bitmap_2 = bitmap;
        bitmap_2.set(current_index, true);

        if let Some(r) = Row::permutations_recur(row, target_sum, bitmap_2, current_index + 1) {
            results.extend(r);
        }

        if let Some(r) = Row::permutations_recur(row, target_sum, bitmap, current_index + 1) {
            results.extend(r);
        }

        Some(results)
    }

    fn sum(bitmap: Bitmap<8>, row: &[usize]) -> usize {
        let mut sum = 0;
        for (i, value) in row.iter().enumerate() {
            if bitmap.get(i) {
                sum += value;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let row = Row {
            sum: 10,
            row: vec![1, 2, 3, 5, 5],
        };

        let perms = row.permutations()
            .into_iter()
            .map(|bitmap| row.show_bitmap(bitmap))
            .collect::<Vec<_>>();
        assert_eq!(
            perms,
            vec![
                "[  1   <2>  <3>  <5>   5  ]",
                "[  1   <2>  <3>   5   <5> ]",
                "[  1    2    3   <5>  <5> ]"
            ]
        );
    }
}
