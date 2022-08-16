use std::fs::File;
use std::io::prelude::*;

fn init_state_values() -> [f64; 3] {
    [1.0, 2.0, 3.05]
}

fn init_parameter_values() -> [f64; 3] {
    [2.4, 21.0, 12.0]
}

fn _rhs(states: &[f64], _t: f64, parameters: &[f64]) -> [f64; 3] {
    let x = states.get(0).unwrap();
    let y = states.get(1).unwrap();
    let z = states.get(2).unwrap();

    let beta = parameters.get(0).unwrap();
    let rho = parameters.get(1).unwrap();
    let sigma = parameters.get(2).unwrap();

    [sigma * (-x + y), -y + (rho - z) * x, x * y - beta * z]
}

fn forward_explicit_euler(states: &[f64], _t: f64, dt: f64, parameters: &[f64]) -> [f64; 3] {
    let x = states.get(0).unwrap();
    let y = states.get(1).unwrap();
    let z = states.get(2).unwrap();

    let beta = parameters.get(0).unwrap();
    let rho = parameters.get(1).unwrap();
    let sigma = parameters.get(2).unwrap();

    let dx_dt = sigma * (-x + y);
    let dy_dt = -y + (rho - z) * x;
    let dz_dt = x * y - beta * z;

    [dt * dx_dt + x, dt * dy_dt + y, dt * dz_dt + z]
}

fn main() {
    println!("Solving the Lorentz ODE");
    let mut state_values = init_state_values();
    let parameter_values = init_parameter_values();
    println!("Initial state values: {:?}", state_values);
    println!("Parameter values: {:?}", parameter_values);

    let mut prev_state_values = state_values;
    let path = "lorentz.txt";
    let mut out = File::create(path).unwrap();

    let dt = 0.01;
    let mut _t = 0.0;
    for index in 0..10000 {
        _t = index as f64 * dt;
        write!(out, "{}\n", state_values.map(|x| x.to_string()).join(" ")).unwrap();
        state_values = forward_explicit_euler(&prev_state_values, _t, dt, &parameter_values);
        prev_state_values = state_values;
    }

    println!("Success!");
}
