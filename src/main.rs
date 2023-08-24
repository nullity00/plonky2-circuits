use plonky2::plonk::{circuit_data::CircuitConfig, circuit_builder::CircuitBuilder};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::iop::witness::{PartialWitness, Witness}; 

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    // Set up 
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, 2>::new(config.clone());

    // build circuit, a - b = d 
    // a = 3, b = 3, d = 0
    let user_t = builder.add_virtual_target();
    let verifier_t = builder.add_virtual_target();

    let lhs_t = builder.sub(user_t, verifier_t);

    let d_t = builder.add_virtual_target();
    builder.connect(lhs_t, d_t);
    let data = builder.build::<C>();

    // assign witness data
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(user_t, GoldilocksField(3));
    pw.set_target(verifier_t, GoldilocksField(3));
    pw.set_target(d_t, GoldilocksField(0));

    //proof 
    let proof = data.prove(pw).unwrap();
    match data.verify(proof) {
        Ok(()) => println!("Proof Success!"),
        Err(x) => println!("{}", x)
    }
}