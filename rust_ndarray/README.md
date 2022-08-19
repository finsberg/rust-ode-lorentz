# Using ndarry

In [rust_ndarry](rust_ndarry) we use a library called [ndarry](https://docs.rs/ndarray/latest/ndarray/) which is similar to Numpy NDArray. Here we also use [ndarray_np](https://docs.rs/ndarray-npy/latest/ndarray_npy/) to save the resulting array. 

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