use qip::{OpBuilder, UnitaryBuilder, Register, CircuitError, Complex, run_local};
use std::f64::consts::{FRAC_PI_2, PI};

fn rx(b: &mut dyn UnitaryBuilder, r: Register, theta: f64) -> Result<Register, CircuitError> {
    let radians = theta / 2.0;
    let c = radians.cos();
    let s = radians.sin();

    b.mat(
        "rx",
        r,
        vec![Complex::from(c) , Complex::new(0.0, -s), Complex::new(0.0, -s), Complex::from(c)],
    )
}

fn spin_energy(angles: Vec<f64>, j: i64, h: i64, iterations: u64) -> f64 {
    let mut sum: i64 = 0;
    let nqubits = angles.len();
    let mut s = vec![0, nqubits];

    for iter in 1..iterations {
        let mut b = OpBuilder::new();
        let qr = b.register(nqubits as u64).unwrap();

        let qr = rx(&mut b, qr, angle).unwrap();
        let (qr, qr_m) =  b.measure(qr);
        let (_, result) = run_local::<f64>(&qr).unwrap();
        let (m, _) = result.get_measurement(&qr_m).unwrap();

        for x in s.iter_mut() {

        }

        let spin: i64 = 2 * (m as i64) - 1;
        sum = sum + spin;
    }
    (sum as f64) / (iterations as f64)
}

fn main() {
    let parameter = vec![0.0, 0.0, 0.0];
    let result = spin_energy(parameter, 0, 0, 1000);
    println!("Average spin {}", result)
}