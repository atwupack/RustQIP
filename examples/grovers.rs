extern crate num;
extern crate qip;

use qip::pipeline::{run_local_with_init, LocalQuantumState, run_local, run_with_state, QuantumState};
use qip::pipeline_debug::run_debug;
use qip::qubits::*;
use qip::types::Precision;

fn prepare_state<P: Precision>(n: u64) -> LocalQuantumState<P> {
    let mut b = OpBuilder::new();
    let q = b.qubit(n).unwrap();
    let q = b.hadamard(q);

    let anc = b.qubit(1).unwrap();
    let anc = b.not(anc);
    let anc = b.hadamard(anc);

    let q = b.merge(vec![q, anc]);

    let (s, _) = run_local(&q);
    s
}



fn apply_us(b: &mut UnitaryBuilder, search: Qubit, ancillary: Qubit) -> (Qubit, Qubit) {
    let search = b.hadamard(search);
    let (search, ancillary) = apply_function(b, search, ancillary, |x| {
        (0, if x == 0 {
            std::f64::consts::PI
        } else {
            0.0
        })
    });
    let search = b.hadamard(search);

    (search, ancillary)
}

fn apply_uw(b: &mut UnitaryBuilder, search: Qubit, ancillary: Qubit, x0: u64) -> (Qubit, Qubit) {
    // Need to move the x0 value into the closure.
    apply_function(b, search, ancillary, move |x| {
        ((x == x0) as u64, 0.0)
    })
}

fn apply_grover_iteration<P: Precision>(x: u64, s: LocalQuantumState<P>) -> LocalQuantumState<P> {
    let mut b = OpBuilder::new();
    let q = b.qubit(s.n - 1).unwrap();
    let anc = b.qubit(1).unwrap();

    let (q, anc) = apply_uw(&mut b, q, anc, x);
    let (q, _) = apply_us(&mut b, q, anc);

    let (s, _) = run_with_state(&q, s).unwrap();
    s
}

fn main() {
    let n = 10;
    let x = 42;

    let s = prepare_state::<f64>(n);

    let iters = 100;
    let (s, states) = (0 .. iters).fold((s, vec![]), |(s, mut vecs), _| {
        let s = apply_grover_iteration(x, s);
        let indices: Vec<u64> = (0 .. n).collect();
        let f = s.stochastic_measure(&indices)[x as usize];
        vecs.push(f);
        (s, vecs)
    });

    states.into_iter().enumerate().for_each(|(i, f)| {
        println!("{:?}\t{:.*}", i, 5, f);
    });
}