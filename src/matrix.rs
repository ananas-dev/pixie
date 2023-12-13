use core::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Matrix<T> {
    pub num_rows: usize,
    pub num_col: usize,
    data: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    pub fn repeat(r: usize, c: usize, value: T) -> Matrix<T> {
        Matrix {
            num_rows: r,
            num_col: c,
            data: vec![vec![value; c]; r],
        }
    }

    pub fn swap_row(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
    }

    pub fn augment(&mut self, vec: &Matrix<T>) {
        for row in 0..self.num_rows {
            self.data[row].push(vec[(row, 0)])
        }

        self.num_col += 1;
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            write!(f, "| ")?;
            for el in row.iter() {
                write!(f, "{:4.1} ", el)?;
            }
            write!(f, "|\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum LinAlgError {
    SingularMatrix,
    IncompatibleDimensions,
}

pub fn solve(a: Matrix<f64>, rhs: Matrix<f64>) -> Result<Matrix<f64>, LinAlgError> {
    if a.num_col != a.num_rows || rhs.num_rows != a.num_col || rhs.num_col != 1 {
        return Err(LinAlgError::IncompatibleDimensions);
    }

    let n = a.num_col;

    let mut system = a.clone();
    system.augment(&rhs);

    for k in 0..n {
        let mut i_max = k;
        let mut v_max = system[(i_max, k)];

        for i in k + 1..n {
            if system[(i, k)].abs() > v_max {
                v_max = system[(i, k)];
                i_max = i;
            }
        }

        if system[(k, i_max)] == 0. {
            return Err(LinAlgError::SingularMatrix);
        }

        if i_max != k {
            system.swap_row(k, i_max);
        }

        for i in k + 1..n {
            let f = system[(i, k)] / system[(k, k)];

            for j in k + 1..n + 1 {
                system[(i, j)] -= system[(k, j)] * f;
            }

            system[(i, k)] = 0.;
        }
    }

    let mut x = Matrix::repeat(n, 1, 0.);

    for i in (0..n).rev() {
        x[(i, 0)] = system[(i, n)];

        for j in i + 1..n {
            x[(i, 0)] -= system[(i, j)] * x[(j, 0)];
        }

        x[(i, 0)] = x[(i, 0)] / system[(i, i)];
    }

    Ok(x)
}
