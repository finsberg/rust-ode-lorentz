# Using pure rust without any dependencies
In [pure_rust](pure_rust) we basically solve the model using the built in data structures in rust

## Usage
You can run the ODE solver using
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