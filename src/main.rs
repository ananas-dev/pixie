use matrix::solve;

use crate::network::{Component, NetworkBuilder};

mod matrix;
mod network;

fn main() {
    let net = NetworkBuilder::new()
        .add_component(Component::CurrentSource { n: 0, p: 1, i: 5. })
        .add_component(Component::Resistor { a: 1, b: 2, r: 10. })
        .add_component(Component::Resistor { a: 2, b: 0, r: 5. })
        .add_component(Component::Resistor { a: 2, b: 3, r: 2. })
        .add_component(Component::VoltageSource { n: 0, p: 3, v: 1. })
        .add_component(Component::Resistor { a: 3, b: 0, r: 7. })
        .add_component(Component::Resistor { a: 3, b: 4, r: 9. })
        .add_component(Component::Resistor { a: 4, b: 0, r: 4. })
        .build();

    let (a, b) = net.compile();
    println!("{}", a);
    println!("{}", b);
    let x = solve(a, b).unwrap();
    println!("{}", x);
}
