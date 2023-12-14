use linalg::solve;
use solver::solve_dc;

use crate::network::{Component, NetworkBuilder};

mod linalg;
mod network;
mod sparse;
mod solver;

fn main() {
    // let net = NetworkBuilder::new()
    //     .add_component(Component::CurrentSource { n: 0, p: 1, i: 5. })
    //     .add_component(Component::Resistor { a: 1, b: 2, r: 10. })
    //     .add_component(Component::Resistor { a: 2, b: 0, r: 5. })
    //     .add_component(Component::Resistor { a: 2, b: 3, r: 2. })
    //     .add_component(Component::VoltageSource { n: 0, p: 3, v: 1. })
    //     .add_component(Component::Resistor { a: 3, b: 0, r: 7. })
    //     .add_component(Component::Resistor { a: 3, b: 4, r: 9. })
    //     .add_component(Component::Resistor { a: 4, b: 0, r: 4. })
    //     .build();

    let net = NetworkBuilder::new()
        .add_component(Component::VoltageSource { n: 0, p: 1, v: 10. })
        .add_component(Component::Diode { n: 1, p: 2, is: 1e-12, t: 300. })
        .add_component(Component::Resistor { a: 2, b: 0, r: 100. })
        .build();

    let x = solve_dc(&net).expect("Did not converge");

    println!("{x}");
}
