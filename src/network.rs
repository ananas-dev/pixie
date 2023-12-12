use std::fmt;

use petgraph::{dot::Dot, graph::DiGraph, visit::EdgeRef, Direction};

use crate::matrix::Matrix;

#[derive(Debug, PartialEq, Clone, Copy)]

pub enum Node {
    El(usize),
    Gnd,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Node::El(idx) => write!(f, "{idx}")?,
            Node::Gnd => write!(f, "GND")?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Resistor(usize, f32),
    CurrentSource(usize, f32),
    VoltageSource(usize, f32),
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Component::Resistor(idx, _) => write!(f, "R{idx}"),
            Component::CurrentSource(idx, _) => write!(f, "I{idx}"),
            Component::VoltageSource(idx, _) => write!(f, "V{idx}"),
        }
    }
}

pub struct Network {
    node_count: usize,
    voltage_source_count: usize,
    graph: DiGraph<Node, Component>,
}

impl Network {
    pub fn conductance(&self) -> Matrix<f32> {
        let n = self.node_count + self.voltage_source_count;
        let mut res = Matrix::repeat(n, n, 0.);

        for edge in self.graph.edge_references() {
            let n1 = self.graph[edge.source()];
            let n2 = self.graph[edge.target()];

            if let Component::Resistor(_, r) = *edge.weight() {
                match (n1, n2) {
                    (Node::El(i), Node::El(j)) => {
                        res[(i - 1, j - 1)] = -1. / r;
                        res[(j - 1, i - 1)] = -1. / r;
                        res[(i - 1, i - 1)] += 1. / r;
                        res[(j - 1, j - 1)] += 1. / r;
                    }
                    (Node::El(i), Node::Gnd) => res[(i - 1, i - 1)] += 1. / r,
                    (Node::Gnd, Node::El(j)) => res[(j - 1, j - 1)] += 1. / r,
                    _ => {}
                }
            }

            // Not ideal but should be extended later
            if let Component::VoltageSource(idx, _) = *edge.weight() {
                match (n1, n2) {
                    (Node::El(i), Node::El(j)) => {
                        res[(self.node_count + idx - 1, i - 1)] = -1.;
                        res[(i - 1, self.node_count + idx - 1)] = -1.;
                        res[(self.node_count + idx - 1, j - 1)] = 1.;
                        res[(j - 1, self.node_count + idx - 1)] = 1.;
                    }
                    (Node::El(i), Node::Gnd) => {
                        res[(self.node_count + idx - 1, i - 1)] = -1.;
                        res[(i - 1, self.node_count + idx - 1)] = -1.;
                    }
                    (Node::Gnd, Node::El(i)) => {
                        res[(self.node_count + idx - 1, i - 1)] = 1.;
                        res[(i - 1, self.node_count + idx - 1)] = 1.;
                    }
                    _ => {}
                }
            }
        }

        res
    }

    pub fn rhs(&self) -> Matrix<f32> {
        let n = self.node_count + self.voltage_source_count;
        let mut res = Matrix::repeat(n, 1, 0.);

        for node in self.graph.node_indices() {
            if let Node::El(i) = self.graph[node] {
                self.graph
                    .edges_directed(node, Direction::Incoming)
                    .filter_map(|e| match *e.weight() {
                        Component::CurrentSource(_, c) => Some(c),
                        _ => None,
                    })
                    .for_each(|c| {
                        res[(i - 1, 0)] += c;
                    });

                self.graph
                    .edges_directed(node, Direction::Outgoing)
                    .filter_map(|e| match *e.weight() {
                        Component::CurrentSource(_, c) => Some(c),
                        _ => None,
                    })
                    .for_each(|c| {
                        res[(i - 1, 0)] -= c;
                    });
            }
        }

        for edge in self.graph.edge_weights() {
            if let Component::VoltageSource(idx, v) = edge {
                res[(self.node_count + idx - 1, 0)] = *v;
            }
        }

        res
    }

    pub fn to_dot(&self) -> String {
        Dot::with_config(&self.graph, &[]).to_string()
    }
}

pub struct NetworkBuilder {
    node_count: usize,
    voltage_source_count: usize,
    graph: DiGraph<Node, Component>,
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder {
            node_count: 0,
            voltage_source_count: 0,
            graph: DiGraph::default(),
        }
    }

    pub fn add_component(mut self, component: Component, a: Node, b: Node) -> NetworkBuilder {
        if let Component::VoltageSource(_, _) = component {
            self.voltage_source_count += 1;
        }

        let index_a = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i] == a)
            .unwrap_or_else(|| {
                if let Node::El(_) = a {
                    self.node_count += 1;
                };

                self.graph.add_node(a)
            });

        let index_b = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i] == b)
            .unwrap_or_else(|| {
                if let Node::El(_) = b {
                    self.node_count += 1;
                };

                self.graph.add_node(b)
            });

        self.graph.add_edge(index_a, index_b, component);

        self
    }

    pub fn build(self) -> Network {
        Network {
            node_count: self.node_count,
            voltage_source_count: self.voltage_source_count,
            graph: self.graph,
        }
    }
}
