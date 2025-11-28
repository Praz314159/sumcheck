//! Memory profiling binary for MLE evaluation
//!
//! Run with: cargo run --bin profile_memory --release --features dhat-heap

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::fs;

use ark_ff::UniformRand;
use ark_test_curves::bls12_381::Fr;
use rand::thread_rng;

use multilinear_extensions::multilinear::mle::{DenseOracle, EvaluationType, MultilinearExtension};
use multilinear_extensions::multilinear::traits::MLE;

/// Find the next available trace number in dhat_traces/
fn next_trace_number() -> u32 {
    let trace_dir = "dhat_traces";

    if fs::create_dir_all(trace_dir).is_err() {
        return 1;
    }

    let mut max_num = 0u32;
    if let Ok(entries) = fs::read_dir(trace_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            // Parse trace_XXX.json
            if let Some(num_str) = name_str
                .strip_prefix("trace_")
                .and_then(|s| s.strip_suffix(".json"))
            {
                if let Ok(num) = num_str.parse::<u32>() {
                    max_num = max_num.max(num);
                }
            }
        }
    }

    max_num + 1
}

fn main() {
    let trace_num = next_trace_number();
    let trace_path = format!("dhat_traces/trace_{:03}.json", trace_num);

    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().file_name(&trace_path).build();

    println!("Trace will be saved to: {}", trace_path);

    let dim = 16; // Adjust dimension as needed
    let mut rng = thread_rng();

    println!("Profiling MLE evaluation with dim = {}", dim);
    println!("Number of points: 2^{} = {}", dim, 1 << dim);

    // Create oracle
    println!("\n--- Creating oracle ---");
    let oracle = DenseOracle::<Fr>::new_rand(dim, &mut rng);

    // Create evaluation point
    let z: Vec<Fr> = (0..dim).map(|_| Fr::rand(&mut rng)).collect();

    // Evaluate
    println!("\n--- Evaluating MLE ---");
    let mle = MultilinearExtension::new(oracle, dim, EvaluationType::Naive);
    let _result = mle.evaluate(&z).expect("Failed to evaluate");

    println!("\n--- Done ---");
    println!("Trace saved to: {}", trace_path);
}
