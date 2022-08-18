use ndarray;
use ndarray_npy::write_npy;

fn init_state_values() -> ndarray::Array1<f64> {
    ndarray::arr1(&[1.0, 2.0, 3.05])
}

fn init_parameter_values() -> ndarray::Array1<f64> {
    ndarray::arr1(&[2.4, 21.0, 12.0])
}

fn _rhs(
    states: &ndarray::Array1<f64>,
    _t: f64,
    parameters: &ndarray::Array1<f64>,
) -> ndarray::Array1<f64> {
    let x = states[0];
    let y = states[1];
    let z = states[2];

    let beta = parameters[0];
    let rho = parameters[1];
    let sigma = parameters[2];

    ndarray::arr1(&[sigma * (-x + y), -y + (rho - z) * x, x * y - beta * z])
}

fn forward_explicit_euler(
    states: &ndarray::Array1<f64>,
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

fn main() {
    println!("Solving the Lorentz ODE");
    let mut state_values = init_state_values();
    let parameter_values = init_parameter_values();
    println!("Initial state values: {:?}", state_values);
    println!("Parameter values: {:?}", parameter_values);

    let mut prev_state_values = state_values;
    let dt = 0.01;
    let mut _t = 0.0;
    let n = 10_000;
    let mut arr = ndarray::Array2::<f64>::zeros((n, 3));
    for index in 0..n {
        _t = index as f64 * dt;
        // write!(out, "{}\n", state_values.map(|x| x.to_string()).join(" ")).unwrap();
        // out.write_all(&state_values).expect("Failed to write file");
        state_values = forward_explicit_euler(&prev_state_values, _t, dt, &parameter_values);

        for i in 0..3 {
            arr[[index, i]] = state_values[i];
        }

        prev_state_values = state_values;
    }
    write_npy("lorentz.npy", &arr).unwrap();

    println!("Success!");
}
