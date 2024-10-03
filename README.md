# Pixie ⚡

A small circuit simulator made for the purpose of learning.

## Usage

```sh
cargo run --release -- test/diode.net
```

To write your own netlist you can take a look at the tests in `./test`, you can use current/voltage sources, resitors and diodes. Each node is labeled with a number and the `0` node is always ground.
