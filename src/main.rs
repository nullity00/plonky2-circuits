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

fn some_function_in_zk(x_inp: u64) -> (CircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2>, PartialWitness<GoldilocksField>) {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Defining the circuit (wires and gates).
    let square_root = builder.add_virtual_target();
    let x_target = builder.add_virtual_target();

    let mut pw = PartialWitness::new();
    let x = F::from_canonical_u64(x_inp);
    pw.set_target(x_target, x);
    pw.set_target(square_root, F::from_canonical_u64(x_inp.sqrt()));

    // Constraint
    let z = builder.mul(square_root, square_root);
    let isz = builder.sub(x_target, z);
    builder.assert_zero(isz);

    builder.register_public_input(square_root);
    builder.register_public_input(x_target);

    let data: CircuitData<
        GoldilocksField,
        PoseidonGoldilocksConfig,
        2
    > = builder.build::<C>();
    (data, pw)
}

fn main() -> Result<()> {
    // Tests
    let (prover, pw) = some_function_in_zk(16);
    let proof = prover.prove(pw)?;

    println!("Factorial starting at {} is {}", proof.public_inputs[0], proof.public_inputs[1]);

    prover.verify(proof)
}
