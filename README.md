# Solving the Lorentz ODE in Rust

This a simple proof of concept for implementing a Rust based ODE solver.

## Install
First you need to [install rust](https://www.rust-lang.org/tools/install).

## There are three different examples

### Using pure rust without any dependencies
In [pure_rust](pure_rust) we basically solve the model using the built in data structures in rust

### Using ndarry
In [rust_ndarry](rust_ndarry) we use a library called [ndarry](https://docs.rs/ndarray/latest/ndarray/) which is similar to Numpy NDArray. Here we also use [ndarray_np](https://docs.rs/ndarray-npy/latest/ndarray_npy/) to save the resulting array. 

### Using pyO3 and rust-numpy
In [rust_numpy_pyO3](rust_numpy_pyO3) we use [rust-numpy](https://github.com/PyO3/rust-numpy) in order to make bindings to python using [pyO3](https://github.com/PyO3/pyo3)

## License
MIT