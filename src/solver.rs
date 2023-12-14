use crate::{
    linalg::{DenseMatrix, Vector, solve},
    network::{Component, Network, GND},
};

#[derive(Debug)]
pub enum SolveError {
    InvalidCircuit,
    NoConvergence,
}

pub fn solve_dc(net: &Network) -> Result<Vector, SolveError> {
    let n = net.num_nodes + net.num_vsrc;
    let mut j = DenseMatrix::zero(n, n);
    let mut rhs = Vector::zero(n);

    compile_linear(net, &mut j, &mut rhs);

    let mut last_x = Vector::zero(n);

    let max_iter = 10000;

    for _ in 0..max_iter {
        let mut j = j.clone();
        let mut rhs = rhs.clone();

        compile_nonlinear(net, &mut j, &mut rhs, &last_x);

        let x = solve(j, rhs).map_err(|_| SolveError::InvalidCircuit)?;

        if x.squared_diff(&last_x) <= 0.00001 {
            return Ok(x);
        }

        last_x = x;
    }

    Err(SolveError::NoConvergence)
}

pub fn compile_linear(net: &Network, j: &mut DenseMatrix, rhs: &mut Vector) {
    let mut vsrc_counter = 0;

    for c in net.iter() {
        match *c {
            Component::Resistor { a, b, r } => {
                let conductance = 1. / r;

                if a != GND {
                    j[(a - 1, a - 1)] += conductance;
                }

                if b != GND {
                    j[(b - 1, b - 1)] += conductance;
                }

                if a != GND && b != GND {
                    j[(a - 1, b - 1)] -= conductance;
                    j[(b - 1, a - 1)] -= conductance;
                }
            }
            Component::CurrentSource { p, n, i } => {
                if p != GND {
                    rhs[p - 1] += i;
                }

                if n != GND {
                    rhs[n - 1] -= i;
                }
            }
            Component::VoltageSource { p, n, v } => {
                let abs_index = net.num_nodes + vsrc_counter;
                vsrc_counter += 1;

                if p != GND {
                    j[(abs_index, p - 1)] = 1.;
                    j[(p - 1, abs_index)] = 1.;
                }

                if n != GND {
                    j[(abs_index, n - 1)] = -1.;
                    j[(n - 1, abs_index)] = -1.;
                }

                rhs[abs_index] = v;
            }
            _ => {}
        }
    }
}

pub fn compile_nonlinear(
    net: &Network,
    j: &mut DenseMatrix,
    rhs: &mut Vector,
    last_x: &Vector,
) {
    for c in net.iter() {
        match *c {
            Component::Diode { n, p, is, t } => {
                // This is a bad lineariser because it can produces infinite values

                // TODO: label consts
                let vt = 2.5852E-2;
                let voltage = last_x[n - 1] - last_x[p - 1];
                let current = is * (f64::exp(voltage / vt) - 1.);

                let d = current / vt;

                if p != GND {
                    j[(p - 1, p - 1)] += d;
                    rhs[p - 1] += current * (1. - voltage/vt);
                }

                if n != GND {
                    j[(n - 1, n - 1)] += d;
                    rhs[n - 1] -= current * (1. - voltage/vt);
                }

                if p != GND && n != GND {
                    j[(p - 1, n - 1)] -= d;
                    j[(n - 1, p - 1)] -= d;
                }
            }
            _ => {}
        }
    }
}
