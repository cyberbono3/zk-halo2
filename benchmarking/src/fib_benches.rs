#[cfg(test)]
mod tests {
    use ark_std::{end_timer, start_timer};
    use fib::fib1::TestFibCircuit;
    use halo2_proofs_zcash::{dev::MockProver, pasta::Fp};
  

    #[test]
    fn bench_fib1() {
        let k = 4;
        let a = Fp::from(1); // F[0]
        let b = Fp::from(1); // F[1]
        let out = Fp::from(55); // F[9]
    
        let circuit = TestFibCircuit::default();

        let public_input = vec![a, b, out];

        let start = start_timer!(|| "start MockProver");
        let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
        prover.assert_satisfied();
        end_timer!(start);
    }
}
