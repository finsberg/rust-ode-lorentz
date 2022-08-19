# Using pyO3 and rust-numpy

In [rust_numpy_pyO3](rust_numpy_pyO3) we use [rust-numpy](https://github.com/PyO3/rust-numpy) in order to make bindings to python using [pyO3](https://github.com/PyO3/pyo3)

## Install
Create a virtual environment 
```
python -m virtualenv venv
```
activate it
```
. venv/bin/activate
```
and install the library
```
python -m pip install -e .
```
 

## Usage
Test out the library using the demo called [`plot.py`](plot.py). There are also a few test in the [`tests`](tests) directory. 

