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

    for iter in 1..=iterations {
        let mut b = OpBuilder::new();

        let mut qrs = Vec::new();
        let mut qr_ms = Vec::new();
        for angle in &angles {
            let qr = b.qubit();
            let qr = rx(&mut b, qr, *angle).unwrap();
            let (qr, qr_m) = b.measure(qr);
            qrs.push(qr);
            qr_ms.push(qr_m);
        }

        let mr = b.merge(qrs).unwrap();

        let (_, result) = run_local::<f64>(&mr).unwrap();

        let mut s = Vec::new();
        for qr_m in &qr_ms {
            let (m, _) = result.get_measurement(qr_m).unwrap();
            s.push( 2 * (m as i64) - 1) ;
        }

        for i in 0..=nqubits-2 {
            sum = sum - j * *s.get(i).unwrap() * *s.get(i+1).unwrap();
        }

        for i in 0..=nqubits-1 {
            sum = sum - h * *s.get(i).unwrap();
        }
    }
    (sum as f64) / (iterations as f64)
}

fn main() {
    let parameter = vec![0.0, 0.0, 0.0];
    let result = spin_energy(parameter, 1, 2, 1000);
    println!("Average energy {}", result)
}