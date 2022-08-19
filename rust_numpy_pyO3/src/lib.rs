use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArrayDyn};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

/// A Python module implemented in Rust.
#[pymodule]
fn lorentz(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn default_states<'py>(py: Python<'py>) -> &PyArray1<f64> {
        let array = rust_fn::init_state_values();
        array.into_pyarray(py)
    }

    #[pyfn(m)]
    fn default_parameters<'py>(py: Python<'py>) -> &PyArray1<f64> {
        let array = rust_fn::init_parameters_values();
        array.into_pyarray(py)
    }

    #[pyfn(m)]
    fn solve<'py>(
        py: Python<'py>,
        end_time: f64,
        dt: f64,
        states: PyReadonlyArrayDyn<f64>,
        parameters: PyReadonlyArrayDyn<f64>,
    ) -> &'py PyArray2<f64> {
        let states_ndarray = states.as_array();
        let parameters_ndarray = parameters.as_array();
        let array = rust_fn::solve(end_time, dt, &states_ndarray, &parameters_ndarray);
        array.into_pyarray(py)
    }

    Ok(())
}

// The rust side functions
// Put it in mod to separate it from the python bindings
// These are just some random operations
// you probably want to do something more meaningful.
mod rust_fn {
    use ndarray::{arr1, Array1, ArrayViewD};

    pub fn init_state_values() -> Array1<f64> {
        arr1(&[1.0, 2.0, 3.05])
    }

    pub fn init_parameters_values() -> Array1<f64> {
        arr1(&[2.4, 21.0, 12.0])
    }

    pub fn forward_explicit_euler(
        states: ndarray::Array1<f64>,
        _t: f64,
        dt: f64,
        parameters: &ndarray::Array1<f64>,
    ) -> ndarray::Array1<f64> {
        let x = states[0];
        let y = states[1];
        let z = states[2];

        let beta = parameters[0];
        let rho = parameters[1];
        let sigma = parameters[2];

        let dx_dt = sigma * (-x + y);
        let dy_dt = -y + (rho - z) * x;
        let dz_dt = x * y - beta * z;

        ndarray::arr1(&[dt * dx_dt + x, dt * dy_dt + y, dt * dz_dt + z])
    }

    pub fn solve(
        end_time: f64,
        dt: f64,
        states: &ArrayViewD<'_, f64>,
        parameters: &ArrayViewD<'_, f64>,
    ) -> ndarray::Array2<f64> {
        let mut state_values = Array1::zeros(3);
        for i in 0..3 {
            state_values[i] = states[i];
        }

        let mut parameter_values = Array1::zeros(3);
        for i in 0..3 {
            parameter_values[i] = parameters[i];
        }

        println!("Initial state values: {:?}", state_values);
        println!("Parameter values: {:?}", parameters);

        let mut prev_state_values = state_values;

        let mut _t = 0.0;
        let n = (end_time / dt) as usize;
        let mut arr = ndarray::Array2::<f64>::zeros((n, 3));
        for index in 0..n {
            _t = index as f64 * dt;
            state_values = forward_explicit_euler(prev_state_values, _t, dt, &parameter_values);

            for i in 0..3 {
                arr[[index, i]] = state_values[i];
            }

            prev_state_values = state_values;
        }
        arr
    }
}
