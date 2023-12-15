use std::{collections::HashSet, slice::Iter, str::FromStr};

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

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NetworkBuilder::from_str(s)?.build())
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

impl FromStr for NetworkBuilder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = NetworkBuilder::new();

        for l in s.lines() {
            let tokens: Vec<&str> = l.split_whitespace().collect();

            if tokens.is_empty() {
                continue;
            }

            match tokens[0].chars().next().ok_or(())? {
                'R' => {
                    let a = tokens[1].parse().map_err(|_| ())?;
                    let b = tokens[2].parse().map_err(|_| ())?;
                    let r = tokens[3].parse().map_err(|_| ())?;

                    builder = builder.add_component(Component::Resistor { a, b, r });
                },
                'V' => {
                    let n = tokens[1].parse().map_err(|_| ())?;
                    let p = tokens[2].parse().map_err(|_| ())?;
                    let v = tokens[3].parse().map_err(|_| ())?;

                    builder = builder.add_component(Component::VoltageSource { n, p, v });
                },
                'I' => {
                    let n = tokens[1].parse().map_err(|_| ())?;
                    let p = tokens[2].parse().map_err(|_| ())?;
                    let i = tokens[3].parse().map_err(|_| ())?;

                    builder = builder.add_component(Component::CurrentSource { n, p, i});
                },
                'D' => {
                    let n = tokens[1].parse().map_err(|_| ())?;
                    let p = tokens[2].parse().map_err(|_| ())?;
                    let is = tokens[3].parse().map_err(|_| ())?;
                    let t = tokens[4].parse().map_err(|_| ())?;

                    builder = builder.add_component(Component::Diode { n, p, is, t });
                }
                _ => return Err(())
            }
        }

        Ok(builder)
    }
}