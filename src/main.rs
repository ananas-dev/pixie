use std::{
    env,
    fs,
};

use pixie::solver::solve_dc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: pixie <filename>")
    }

    let net = fs::read_to_string(&args[1])
        .expect("Invalid file name")
        .parse()
        .expect("Could not parse netlist");

    let x = solve_dc(&net).expect("Unsolvable circuit");

    println!("Simulation result: {x}");
}