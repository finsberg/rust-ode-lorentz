# Solving the Lorentz ODE in Rust

This a simple proof of concept for implementing a Rust based ODE solver.

## Install
First you need to [install rust](https://www.rust-lang.org/tools/install).

## Usage
Next you can run the ODE solver using
```
cargo run
```
To build the code with optimizations run
```
cargo build --release
```
And run the binary
```
./target/release/lorentz
```
## Plotting
Plot the results with python and `matplotlib` 
```
python plot.py
```

## License
MIT