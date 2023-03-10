use bitmaps::Bitmap;

pub(crate) fn row_permutations(sum: usize, row: &[usize]) -> Vec<Bitmap<8>> {
    row_perm_recur(row, sum, Bitmap::<8>::new(), 0, Vec::new()).unwrap()
}

fn row_perm_recur(
    row: &[usize],
    target_sum: usize,
    bitmap: Bitmap<8>,
    current_index: usize,
    mut results: Vec<Bitmap<8>>,
) -> Option<Vec<Bitmap<8>>> {
    match sum(bitmap, row).cmp(&target_sum) {
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

    let mut bitmap_2 = bitmap.clone();
    bitmap_2.set(current_index, true);

    match row_perm_recur(row, target_sum, bitmap_2, current_index + 1, Vec::new()) {
        Some(r) => {
            results.extend(r);
        }
        None => (),
    }
    match row_perm_recur(row, target_sum, bitmap, current_index + 1, Vec::new()) {
        Some(r) => {
            results.extend(r);
        }
        None => (),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let sum = 10;
        let row = &vec![1, 2, 3, 5, 5];
        let perms = row_permutations(sum, row)
            .into_iter()
            .map(|bitmap| row_bitmap_debug(row, bitmap))
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
