use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    arithmetic::FieldExt,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

const STEPS: usize = 5;

struct TestCircuit<F: FieldExt> {
    _ph: PhantomData<F>,
    values: Value<Vec<F>>,
}

#[derive(Clone, Debug)]
struct TestConfig<F: FieldExt + Clone> {
    _ph: PhantomData<F>,
    q_enable: Selector,
    advice: Column<Advice>,
}

// ANCHOR: without_witnesses
impl<F: FieldExt> Circuit<F> for TestCircuit<F> {
    type Config = TestConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        TestCircuit {
            _ph: PhantomData,
            values: Value::unknown(),
        }
    }
  
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let q_enable = meta.complex_selector();
        let advice = meta.advice_column();
  
        meta.create_gate("step", |meta| {
            let curr = meta.query_advice(advice, Rotation::cur());
            let next = meta.query_advice(advice, Rotation::next());
            let q_enable = meta.query_selector(q_enable);
            vec![q_enable * (curr - next + Expression::Constant(F::one()))]
        });

        TestConfig {
            _ph: PhantomData,
            q_enable,
            advice,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config, //
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "steps",
            |mut region| {
                for i in 0..STEPS {
                    // assign the witness value to the advice column
                    region.assign_advice(
                        || "assign advice",
                        config.advice,
                        i,
                        || self.values.as_ref().map(|values| values[i]),
                    )?;

                    // turn on the gate
                    config.q_enable.enable(&mut region, i)?;
                }

                // assign the final "next" value
                region.assign_advice(
                    || "assign advice",
                    config.advice,
                    STEPS,
                    || self.values.as_ref().map(|values| values[STEPS]),
                )?;

                Ok(())
            },
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{halo2curves::bn256::Fr, dev::MockProver};
    
    use super::*;

    #[test]
    fn test_add() {
        // generate a witness
        let start = Fr::from(1337u64);
        let mut values = vec![start];
        while values.len() < STEPS + 1 {
            let last = values.last().unwrap();
            values.push(last + Fr::one());
        }

        // run the MockProver
        let circuit = TestCircuit::<Fr> {
            _ph: PhantomData,
            values: Value::known(values),
        };
        let prover = MockProver::run(8, &circuit, vec![]).unwrap();
        prover.verify().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        // generate a witness
        let start = Fr::from(1337u64);
        let mut values = vec![start];
        while values.len() < STEPS - 2  {
            let last = values.last().unwrap();
            values.push(last + Fr::one());
        }

        // run the MockProver
        let circuit = TestCircuit::<Fr> {
            _ph: PhantomData,
            values: Value::known(values),
        };
        let prover = MockProver::run(8, &circuit, vec![]).unwrap();
        prover.verify().unwrap();
    }
    
}