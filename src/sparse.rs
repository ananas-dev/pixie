use std::slice::Iter;

pub struct CooMatrix<T> {
    values: Vec<T>,
    col_indices: Vec<usize>,
    row_indices: Vec<usize>,
}

impl <T> CooMatrix<T> {
    pub fn new() -> CooMatrix<T> {
        CooMatrix {
            values: Vec::new(),
            col_indices: Vec::new(),
            row_indices: Vec::new(),
        }
    } 

    pub fn insert(&mut self, row: usize, col: usize, value: T) {
        match self.col_indices.binary_search(&col) {
            Ok(pos) => {}
            Err(pos) => {
                self.row_indices.insert(pos, row);
                self.col_indices.insert(pos, col);
                self.values.insert(pos, value);
            },
        }
    } 

    // pub fn iter(&self) -> Iter<'_, (usize, usize, T)> {

    // }
}

pub struct CscMatrix<T> {
    values: Vec<T>,
    col_ptr: Vec<usize>,
    row_indices: Vec<usize>,
}

// impl<T: Copy> From<CooMatrix<T>> for CscMatrix<T> {
//     fn from(value: CooMatrix<T>) -> Self {
//         let mut values = Vec::new();
//         let mut col_ptr = Vec::new();
//         let mut row_indices = Vec::new();

//         let mut last_row = -1;

//         for &(row, col, val) in value.iter() {
//             if row != las
//         }

//         CscMatrix {
//             values,
//             col_ptr,
//             row_indices,
//         }
//     }
// }