use std::fmt;

use crate::cell::Cell;
use crate::cell_group_location::CellGroupLocation;
use crate::cell_selection::CellSelection;
use crate::cell_status::CellStatus;

pub struct CellGroup {
    pub location: CellGroupLocation,
    pub cells: Vec<Cell>,
    pub target_sum: usize,
    generation: usize,
}

impl CellGroup {
    pub fn new(location: CellGroupLocation, cells: Vec<Cell>, target_sum: usize) -> CellGroup {
        CellGroup {
            location,
            cells,
            target_sum,
            generation: 0,
        }
    }

    pub fn sum_max(&self) -> usize {
        self.cells.iter().fold(0, |sum, cell| sum + cell.value)
    }

    pub fn sum_of_selection(&self, selection: &CellSelection) -> usize {
        let mut sum = 0;
        for (i, cell) in self.cells.iter().enumerate() {
            if selection.get(i) {
                sum += cell.value;
            }
        }
        sum
    }

    pub fn fmt_selection(&self, selection: &CellSelection) -> String {
        let mut output = String::new();
        output.push('[');
        for (i, value) in self.cells.iter().enumerate() {
            if selection.get(i) {
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

    pub fn solve(mut self) -> Self {
        self = self.mark_removed();
        self
    }

    fn permutations(&self) -> Vec<CellSelection> {
        self.permutations_recur(CellSelection::new(), 0, 0).unwrap()
    }

    fn permutations_recur(
        &self,
        selection: CellSelection,
        current_index: usize,
        current_sum: usize,
    ) -> Option<Vec<CellSelection>> {
        let mut results: Vec<CellSelection> = Vec::new();
        match current_sum.cmp(&self.target_sum) {
            std::cmp::Ordering::Greater => return None,
            std::cmp::Ordering::Equal => {
                results.push(selection);
                return Some(results);
            }
            std::cmp::Ordering::Less => (),
        }

        if current_index >= self.cells.len() {
            return None;
        }

        let mut calc_selected = true;
        let mut calc_deselected = true;

        let current_cell = &self.cells[current_index];
        match current_cell.status {
            CellStatus::Unknown => (),
            CellStatus::Known => calc_deselected = false,
            CellStatus::Removed => calc_selected = false,
        }

        if calc_selected {
            let selected_sum = current_sum + current_cell.value;
            let mut s = selection.clone();
            s.set(current_index, true);
            if let Some(r) = self.permutations_recur(s, current_index + 1, selected_sum) {
                results.extend(r);
            }
        }

        if calc_deselected {
            let mut s = selection.clone();
            s.set(current_index, false);
            if let Some(r) = self.permutations_recur(s, current_index + 1, current_sum) {
                results.extend(r);
            }
        }

        Some(results)
    }

    fn mark_removed(mut self) -> Self {
        self.generation += 1;

        let perms = self.permutations();
        let mut all = CellSelection::new();

        for (_, selection) in perms.iter().enumerate() {
            all = all.bitor(selection);
        }

        for i in 0..self.cells.len() {
            if !all.get(i) {
                self.cells[i].status = CellStatus::Removed;
            }
        }

        self
    }
}

impl fmt::Display for CellGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = "[ ".to_string();
        for cell in &self.cells {
            output.push_str(&cell.to_string());
            output.push_str(" ");
        }
        output.push_str("]");
        write!(f, "{}", output)
    }
}
