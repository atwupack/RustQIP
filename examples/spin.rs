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

fn spin_energy(angle: f64, iterations: u64) -> f64 {
    let mut sum: i64 = 0;
    for i in 1..iterations {
        let mut b = OpBuilder::new();
        let qr = b.qubit();

        let qr = rx(&mut b, qr, angle).unwrap();
        let (qr, qr_m) =  b.measure(qr);
        let (_, result) = run_local::<f64>(&qr).unwrap();
        let (m, _) = result.get_measurement(&qr_m).unwrap();
        let spin: i64 = 2 * (m as i64) - 1;
        sum = sum + spin;
    }
    (sum as f64) / (iterations as f64)
}

fn main() {
    let result = spin_energy(4.0 * PI / 5.0, 1000);
    println!("Average spin {}", result)
}