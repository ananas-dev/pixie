use std::{collections::HashSet, slice::Iter};

pub const GND: usize = 0;

#[derive(Clone, Copy)]
pub enum Component {
    Resistor { a: usize, b: usize, r: f64 },
    CurrentSource { n: usize, p: usize, i: f64 },
    VoltageSource { n: usize, p: usize, v: f64 },
    Diode { n: usize, p: usize, is: f64, t: f64 },
}

pub struct Network {
    components: Vec<Component>,
    pub num_nodes: usize,
    pub num_vsrc: usize,
}

impl Network {
    pub fn iter(&self) -> Iter<'_, Component> {
        self.components.iter()
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
        nodes.insert(GND);

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
            Component::Diode { p, n, .. } => {
                self.update_node_pair(p, n);
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
