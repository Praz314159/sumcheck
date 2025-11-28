use ark_ff::UniformRand;
use ark_test_curves::bls12_381::Fr;
use rand::thread_rng;

use multilinear_extensions::multilinear::mle::{
    BCubeMapOracle, EvaluationType, MultilinearExtension,
};
use multilinear_extensions::multilinear::traits::MLE;

/// test MLE evaluation:
///     1. construct a random map: B^dim -> F
///     2. choose random point z
///     3. evaluate using naive
fn main() {
    let dim = 4;
    let mut rng = thread_rng();

    // 1. construct a random map
    let oracle =
        BCubeMapOracle::<Fr>::new_rand(dim, &mut rng).expect("Failed to create random oracle");

    // 2. choose random point z in F^dim
    let z: Vec<Fr> = (0..dim).map(|_| Fr::rand(&mut rng)).collect();

    // 3. create MLE and evaluate using naive
    let mle = MultilinearExtension::new(oracle, dim, EvaluationType::Naive);
    let result = mle.evaluate(&z).expect("Failed to evaluate MLE");

    println!(
        "Point z: [{}]",
        z.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("Evaluated MLE at z: {}", result);
}
