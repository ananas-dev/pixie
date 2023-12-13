use std::collections::HashSet;

use crate::matrix::Matrix;

const GROUND: usize = 0;

#[derive(Clone, Copy)]
pub enum Component {
    Resistor { a: usize, b: usize, r: f64 },
    CurrentSource { n: usize, p: usize, i: f64 },
    VoltageSource { n: usize, p: usize, v: f64 },
}

pub struct Network {
    components: Vec<Component>,
    num_nodes: usize,
    num_vsrc: usize,
}

impl Network {
    pub fn compile(&self) -> (Matrix<f64>, Matrix<f64>) {
        let n = self.num_nodes + self.num_vsrc;

        let mut y = Matrix::repeat(n, n, 0.);
        let mut rhs = Matrix::repeat(n, 1, 0.);
        let mut vsrc_counter = 0;

        for c in self.components.iter() {
            match *c {
                Component::Resistor { a, b, r } => {
                    let conductance = 1. / r;

                    if a != GROUND {
                        y[(a - 1, a - 1)] += conductance;
                    }

                    if b != GROUND {
                        y[(b - 1, b - 1)] += conductance;
                    }

                    if a != GROUND && b != GROUND {
                        y[(a - 1, b - 1)] -= conductance;
                        y[(b - 1, a - 1)] -= conductance;
                    }
                }
                Component::CurrentSource { p, n, i } => {
                    if p != GROUND {
                        rhs[(p - 1, 0)] += i;
                    }

                    if n != GROUND {
                        rhs[(n - 1, 0)] -= i;
                    }
                }
                Component::VoltageSource { p, n, v } => {
                    let abs_index = self.num_nodes + vsrc_counter;
                    vsrc_counter += 1;

                    if p != GROUND {
                        y[(abs_index, p - 1)] = 1.;
                        y[(p - 1, abs_index)] = 1.;
                    }

                    if n != GROUND {
                        y[(abs_index, n - 1)] = -1.;
                        y[(n - 1, abs_index)] = -1.;
                    }

                    rhs[(abs_index, 0)] = v;
                }
            }
        }

        (y, rhs)
    }
}

pub struct NetworkBuilder {
    components: Vec<Component>,
    nodes: HashSet<usize>,
    num_nodes: usize,
    num_vsrc: usize,
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        let mut nodes = HashSet::new();
        nodes.insert(0); // Insert the ground node

        NetworkBuilder {
            components: Vec::new(),
            num_nodes: 0,
            num_vsrc: 0,
            nodes,
        }
    }

    pub fn add_component(mut self, component: Component) -> NetworkBuilder {
        match component {
            Component::Resistor { a, b, .. } => {
                self.update_node_pair(a, b);
            }
            Component::CurrentSource { p, n, .. } => {
                self.update_node_pair(p, n);
            }
            Component::VoltageSource { p, n, .. } => {
                self.update_node_pair(p, n);
                self.num_vsrc += 1;
            }
        }

        self.components.push(component);

        self
    }

    pub fn build(self) -> Network {
        Network {
            components: self.components,
            num_nodes: self.num_nodes,
            num_vsrc: self.num_vsrc,
        }
    }

    fn update_node_pair(&mut self, a: usize, b: usize) {
        if !self.nodes.contains(&a) {
            self.nodes.insert(a);
            self.num_nodes += 1;
        }

        if !self.nodes.contains(&b) {
            self.nodes.insert(b);
            self.num_nodes += 1;
        }
    }
}
