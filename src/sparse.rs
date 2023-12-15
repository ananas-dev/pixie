use std::cmp::Ordering;

use crate::linalg::DenseMatrix;

#[derive(Debug, Clone, Copy)]
pub struct CooEntry {
    pub row: usize,
    pub col: usize,
    pub val: f64,
}

impl Eq for CooEntry {}

impl PartialEq for CooEntry {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl PartialOrd for CooEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for CooEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.col.cmp(&other.col),
            other => other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CooMatrix {
    num_row: usize,
    num_col: usize,
    entries: Vec<CooEntry>,
}

impl CooMatrix {
    // TODO: More consistent arg names
    pub fn new(r: usize, c: usize) -> CooMatrix {
        CooMatrix {
            num_row: r,
            num_col: c,
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, row: usize, col: usize, val: f64) {
        let entry = CooEntry { row, col, val };

        match self.entries.binary_search(&entry) {
            Ok(pos) => self.entries[pos].val += val,
            Err(pos) => self.entries.insert(pos, entry),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &CooEntry> {
        self.entries.iter()
    }

    pub fn to_dense(&self) -> DenseMatrix {
        let mut res = DenseMatrix::zeros(self.num_row, self.num_col);

        for entry in self.iter() {
            res[(entry.row, entry.col)] = entry.val
        }

        res
    }
}

// pub struct CsrMatrix {
//     values: Vec<f64>,
//     row_ptr: Vec<usize>,
//     col_indices: Vec<usize>,
// }

// impl CsrMatrix {
//     pub fn lu(&self) {
//     }
// }

// impl From<CooMatrix> for CsrMatrix {
//     fn from(value: CooMatrix) -> Self {
//         let mut values = Vec::new();
//         let mut col_indices = Vec::new();
//         let mut row_ptr = vec![0];

//         let mut last_row = 0;

//         for entry in value.iter() {
//             values.push(entry.val);
//             col_indices.push(entry.col);

//             if entry.row != last_row {
//                 row_ptr.push(entry.row);
//                 last_row = entry.row;
//             }
//         }

//         CsrMatrix {
//             values,
//             row_ptr,
//             col_indices,
//         }
//     }
// }