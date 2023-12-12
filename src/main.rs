use matrix::solve;

use crate::network::{Component, NetworkBuilder, Node};

mod network;
mod matrix;

fn main() {
    let net = NetworkBuilder::new()
        .add_component(Component::CurrentSource(1, 5.), Node::Gnd, Node::El(1))
        .add_component(Component::Resistor(1, 10.), Node::El(1), Node::El(2))
        .add_component(Component::Resistor(2, 5.), Node::El(2), Node::Gnd)
        .add_component(Component::Resistor(3, 2.), Node::El(2), Node::El(3))
        .add_component(Component::VoltageSource(1, 1.), Node::Gnd, Node::El(3))
        .add_component(Component::Resistor(4, 7.), Node::El(3), Node::Gnd)
        .add_component(Component::Resistor(5, 9.), Node::El(3), Node::El(4))
        .add_component(Component::Resistor(6, 4.), Node::El(4), Node::Gnd)
        .build();

    let a = net.conductance();
    let b = net.rhs();
    println!("{}", a);
    println!("{}", b);
    let x = solve(a, b).unwrap();
    println!("{}", x);
}
