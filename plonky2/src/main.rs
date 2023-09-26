use anyhow::Result;
use num::integer::Roots;
use plonky2::field::types::Field;
use plonky2::iop::witness::{ PartialWitness, WitnessWrite };
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{ GenericConfig, PoseidonGoldilocksConfig };
use plonky2::plonk::circuit_data::CircuitData;
use plonky2::field::goldilocks_field::GoldilocksField;

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

fn some_function_in_zk(a_inp: u64, b_inp: u64) -> (CircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2>, PartialWitness<GoldilocksField>) {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Defining the circuit (wires and gates).
    let a = builder.add_virtual_target();
    let b = builder.add_virtual_target();
    let c = builder.add_virtual_target();

    let mut pw = PartialWitness::new();
    let x = F::from_canonical_u64(a_inp);
    let y = F::from_canonical_u64(b_inp);
    pw.set_target(a, x);
    pw.set_target(b, y);
    pw.set_target(c, F::from_canonical_u64(77));

    // Constraint
    let z = builder.add(a, b);
    builder.constrain_to_constant(z, F::from_canonical_u64(77));

    builder.register_public_input(a);
    builder.register_public_input(b);

    let data: CircuitData<
        GoldilocksField,
        PoseidonGoldilocksConfig,
        2
    > = builder.build::<C>();
    (data, pw)
}

fn main() -> Result<()> {
    // Tests
    let (prover, pw) = some_function_in_zk(2, 75);
    let proof = prover.prove(pw)?;

    prover.verify(proof)
}
