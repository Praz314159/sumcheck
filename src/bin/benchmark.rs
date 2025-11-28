//! Benchmark binary for MLE evaluation algorithms
//!
//! Measures evaluation time across different dimensions and generates plots.

use std::fs;
use std::time::{Duration, Instant};

use ark_ff::UniformRand;
use ark_test_curves::bls12_381::Fr;
use rand::thread_rng;

use plotters::prelude::*;

use multilinear_extensions::multilinear::mle::{DenseOracle, EvaluationType, MultilinearExtension};
use multilinear_extensions::multilinear::traits::MLE;

/// Result of benchmarking a single dimension
struct BenchResult {
    dim: usize,
    avg_time_ms: f64,
}

/// Run benchmark for a single dimension, averaging over multiple runs
fn bench_dimension(dim: usize, num_runs: usize, strategy: EvaluationType) -> BenchResult {
    let mut rng = thread_rng();
    let mut total_time = Duration::ZERO;

    for _ in 0..num_runs {
        // Create fresh oracle for each run
        let oracle = DenseOracle::<Fr>::new_rand(dim, &mut rng);

        // Create random evaluation point
        let z: Vec<Fr> = (0..dim).map(|_| Fr::rand(&mut rng)).collect();

        // Create MLE
        let mle = MultilinearExtension::new(oracle, dim, strategy);

        // Time the evaluation
        let start = Instant::now();
        let _result = mle.evaluate(&z).expect("Failed to evaluate");
        total_time += start.elapsed();
    }

    let avg_time_ms = total_time.as_secs_f64() * 1000.0 / num_runs as f64;

    BenchResult { dim, avg_time_ms }
}

/// Generate a line chart of timing results
fn generate_chart(results: &[BenchResult], title: &str, output_path: &str) {
    let max_time = results.iter().map(|r| r.avg_time_ms).fold(0.0, f64::max);
    let min_dim = results.first().map(|r| r.dim).unwrap_or(0);
    let max_dim = results.last().map(|r| r.dim).unwrap_or(20);

    let root = BitMapBackend::new(output_path, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 48))
        .margin(40)
        .x_label_area_size(80)
        .y_label_area_size(120)
        .build_cartesian_2d(
            (min_dim as f64)..(max_dim as f64 + 0.5),
            0.0..(max_time * 1.1),
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Dimension")
        .y_desc("Time (ms)")
        .axis_desc_style(("sans-serif", 36))
        .label_style(("sans-serif", 28))
        .x_label_formatter(&|x| format!("{}", *x as i32))
        .draw()
        .unwrap();

    // Draw the line
    let data: Vec<(f64, f64)> = results
        .iter()
        .map(|r| (r.dim as f64, r.avg_time_ms))
        .collect();

    chart
        .draw_series(LineSeries::new(data.clone(), &BLUE))
        .unwrap()
        .label("Naive")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // Draw points
    chart
        .draw_series(PointSeries::of_element(data, 5, &BLUE, &|c, s, st| {
            Circle::new(c, s, st.filled())
        }))
        .unwrap();

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .label_font(("sans-serif", 28))
        .draw()
        .unwrap();

    root.present().unwrap();

    println!("Chart saved to {}", output_path);
}

/// Find the next available benchmark number in benchmarks/
fn next_benchmark_number() -> u32 {
    let bench_dir = "benchmarks";

    if fs::create_dir_all(bench_dir).is_err() {
        return 1;
    }

    let mut max_num = 0u32;
    if let Ok(entries) = fs::read_dir(bench_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            // Parse benchmark_XXX.png
            if let Some(num_str) = name_str
                .strip_prefix("benchmark_")
                .and_then(|s| s.strip_suffix(".png"))
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
    let min_dim = 4;
    let max_dim = 20;
    let num_runs = 5;

    let bench_num = next_benchmark_number();
    let output_path = format!("benchmarks/benchmark_{:03}.png", bench_num);

    println!("Benchmarking MLE evaluation (Naive algorithm)");
    println!("Dimensions: {} to {}", min_dim, max_dim);
    println!("Runs per dimension: {}", num_runs);
    println!("Output: {}", output_path);
    println!();

    let mut results = Vec::new();

    for dim in min_dim..=max_dim {
        let result = bench_dimension(dim, num_runs, EvaluationType::Naive);
        println!("dim = {:2}: {:.3} ms", result.dim, result.avg_time_ms);
        results.push(result);
    }

    println!();
    println!("Generating chart...");

    generate_chart(
        &results,
        "MLE Naive Evaluation Time vs Dimension",
        &output_path,
    );
}
