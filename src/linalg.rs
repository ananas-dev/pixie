use core::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct DenseMatrix {
    num_rows: usize,
    num_col: usize,
    data: Vec<Vec<f64>>,
}

impl DenseMatrix {
    pub fn zeros(r: usize, c: usize) -> DenseMatrix {
        DenseMatrix {
            num_rows: r,
            num_col: c,
            data: vec![vec![0.; c]; r],
        }
    }

    pub fn swap_row(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
    }

    pub fn augment(&mut self, vec: &Vector) {
        for row in 0..self.num_rows {
            self.data[row].push(vec[row])
        }

        self.num_col += 1;
    }
}

impl Index<(usize, usize)> for DenseMatrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for DenseMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl fmt::Display for DenseMatrix {
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

#[derive(Clone)]
pub struct Vector {
    len: usize,
    data: Vec<f64>,
}

impl Vector {
    pub fn zeros(len: usize) -> Vector {
        Vector {
            len,
            data: vec![0.; len],
        }
    }

    pub fn squared_norm(&self) -> f64 {
        let mut res = 0.;

        for x in self.data.iter() {
            res += x.exp2();
        }

        res
    }

    pub fn squared_diff(&self, other: &Vector) -> f64 {
        (self.squared_norm() - other.squared_norm()).abs()
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "| ")?;
        for el in self.data.iter() {
            write!(f, "{:4.1} ", el)?;
        }
        write!(f, "|")?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum LinAlgError {
    SingularMatrix,
    IncompatibleDimensions,
}

pub fn solve(a: DenseMatrix, rhs: Vector) -> Result<Vector, LinAlgError> {
    if a.num_col != a.num_rows || rhs.len != a.num_col {
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

    let mut x = Vector::zeros(n);

    for i in (0..n).rev() {
        x[i] = system[(i, n)];

        for j in i + 1..n {
            x[i] -= system[(i, j)] * x[j];
        }

        x[i] = x[i] / system[(i, i)];
    }

    Ok(x)
}
