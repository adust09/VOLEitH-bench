use std::{
    io::Cursor,
    path::Path,
    time::{Duration, Instant},
};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use merlin::Transcript;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use utils::voleith::{
    create_buffer_with_capacity, get_process_usage, init_system_monitoring, read_file_cached,
    BenchmarkResult, MonitoringConfig,
};

mod utils;

fn create_transcript() -> Transcript {
    Transcript::new(b"voleith transcript")
}

fn sha256_single(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/single/circuit.txt";
    let private_path = "circuits/sha256/single/private.txt";
    let public_path = "circuits/sha256/single/public.txt";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_voleith_benchmark(
        c,
        "sha256_single",
        circuit_path,
        private_path,
        public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

fn sha256_hash_chain_10(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/hashchain/circuit.txt";
    let private_path = "circuits/sha256/hashchain/private.txt";
    let public_path = "circuits/sha256/hashchain/public.txt";

    if Path::new(circuit_path).exists()
        && Path::new(private_path).exists()
        && Path::new(public_path).exists()
    {
        let monitoring_config = MonitoringConfig {
            enabled: true,
            refresh_interval_ms: 50,
            stabilization_delay_ms: 100,
        };

        run_voleith_benchmark(
            c,
            "sha256_hash_chain_10",
            circuit_path,
            private_path,
            public_path,
            create_transcript,
            Some(monitoring_config),
        );
    } else {
        println!("One or more files do not exist.");
    }
}

fn keccak_f_single(c: &mut Criterion) {
    // Benchmark f2 field
    let circuit_path = "circuits/keccak_f/circuit.txt";
    let private_path = "circuits/keccak_f/private.txt";
    let public_path = "circuits/keccak_f/public.txt";

    // Create a monitoring config with reduced overhead
    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_voleith_benchmark(
        c,
        "keccak_f",
        circuit_path,
        private_path,
        public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

criterion_group!(benches, sha256_single, sha256_hash_chain_10, keccak_f_single);
criterion_main!(benches);

pub fn run_voleith_proof(
    circuit_path_str: &str,
    private_input_path_str: &str,
    public_input_path_str: &str,
    create_transcript_fn: fn() -> Transcript,
    monitoring_config: Option<MonitoringConfig>,
) -> BenchmarkResult {
    // Use provided monitoring config or default
    let config = monitoring_config.unwrap_or_default();
    // Number of runs to average
    const NUM_RUNS: u32 = 10;

    // Read circuit and input files efficiently
    let circuit_path = Path::new(circuit_path_str);
    let circuit_bytes = read_file_cached(circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at {:?}", circuit_path));
    let circuit_bytes_slice = circuit_bytes.as_slice();

    let private_input_path = Path::new(private_input_path_str);
    assert!(
        private_input_path.exists(),
        "Private input file does not exist at {:?}",
        private_input_path
    );

    let public_input_path = Path::new(public_input_path_str);
    assert!(
        public_input_path.exists(),
        "Public input file does not exist at {:?}",
        public_input_path
    );

    println!("Running {} iterations for accurate measurements...", NUM_RUNS);

    // System stabilization before measuring
    if config.stabilization_delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(config.stabilization_delay_ms));
    }

    // ----- PROVER MEASUREMENTS -----
    // Initialize system monitoring for prover measurements
    let mut prover_system = init_system_monitoring(&config);
    // Create a buffer with pre-allocated capacity
    let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
    let mut circuit_for_proof = Cursor::new(circuit_buffer);
    let mut transcript_for_proof = create_transcript_fn();
    let rng_for_proof = &mut thread_rng();

    let mut total_proving_time = Duration::ZERO;

    // Generate the first proof and measure its time
    let start_main = Instant::now();
    let proof = Proof::<InsecureVole>::prove::<_, _>(
        &mut circuit_for_proof,
        private_input_path,
        &mut transcript_for_proof,
        rng_for_proof,
    )
    .expect("Failed to generate main proof");
    total_proving_time += start_main.elapsed();
    // return_cursor_to_pool(circuit_for_proof);

    // Pre-allocate a vector for storing RNGs to minimize allocations
    let mut rngs = Vec::with_capacity(NUM_RUNS as usize);
    for _ in 0..NUM_RUNS {
        rngs.push(thread_rng());
    }

    // Run additional (NUM_RUNS-1) iterations for a total of NUM_RUNS
    for i in 1..NUM_RUNS {
        // Reuse the buffer by creating a new cursor with the same data
        let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
        let mut circuit_run = Cursor::new(circuit_buffer);
        let mut transcript_run = create_transcript_fn();
        let rng_run = &mut rngs[i as usize];

        let start = Instant::now();
        let _ = Proof::<InsecureVole>::prove::<_, _>(
            &mut circuit_run,
            private_input_path,
            &mut transcript_run,
            rng_run,
        )
        .expect(&format!("Failed to generate proof in iteration {}", i));
        total_proving_time += start.elapsed();
    }

    // Calculate average proving time across all runs
    let prove_duration = total_proving_time / NUM_RUNS;
    println!("Average proving time ({} runs): {:?}", NUM_RUNS, prove_duration);

    let (prover_cpu_usage, prover_mem_usage) = get_process_usage(&mut prover_system, &config);
    println!("Prover CPU Usage: {:.2}%", prover_cpu_usage);

    // Calculate proof size in bytes
    let proof_string = format!("{:?}", proof);
    let proof_size_bytes = proof_string.len();
    println!("Proof size: {} bytes", proof_size_bytes);
    // Calculate communication overhead (proof size plus public inputs and protocol overhead)
    // Read public input file to calculate its size for communication overhead
    // Read public input file once and reuse the content
    let public_input_content = read_file_cached(public_input_path).unwrap_or_else(|_| Vec::new());
    let public_input_size = public_input_content.len();

    // Communication overhead includes the proof size and the public inputs
    let communication_overhead_bytes = proof_size_bytes + public_input_size;

    println!("Communication overhead: {} bytes", communication_overhead_bytes);

    // Allow system to stabilize before verification
    if config.stabilization_delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(config.stabilization_delay_ms));
    }

    // ----- VERIFIER MEASUREMENTS -----
    // Initialize system monitoring for verifier measurements
    let mut verifier_system = init_system_monitoring(&config);

    // Measure verification time with NUM_RUNS iterations
    let mut total_verification_time = Duration::ZERO;
    // Pre-allocate transcripts for verification
    let mut verification_transcripts = Vec::with_capacity(NUM_RUNS as usize);
    for _ in 0..NUM_RUNS {
        verification_transcripts.push(create_transcript_fn());
    }

    for i in 0..NUM_RUNS {
        // Create a new cursor for each verification
        let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
        let mut circuit_verify = Cursor::new(circuit_buffer);
        let verification_transcript = &mut verification_transcripts[i as usize];

        let start = Instant::now();
        let verification_result = proof.verify(&mut circuit_verify, verification_transcript);
        total_verification_time += start.elapsed();

        assert!(
            verification_result.is_ok(),
            "{}",
            &format!("Proof verification failed in iteration {}", i)
        );
    }

    let verify_duration = total_verification_time / NUM_RUNS;
    println!("Average verification time ({} runs): {:?}", NUM_RUNS, verify_duration);

    let (verifier_cpu_usage, verifier_mem_usage) = get_process_usage(&mut verifier_system, &config);
    println!("Verifier CPU Usage: {:.2}%", verifier_cpu_usage);

    BenchmarkResult {
        proof_generation_time_ms: prove_duration.as_millis() as u64,
        proof_verification_time_ms: verify_duration.as_millis() as u64,
        proof_size_bytes,
        communication_overhead_bytes,
        prover_cpu_usage,
        prover_memory_usage_mb: prover_mem_usage,
        verifier_cpu_usage,
        verifier_memory_usage_mb: verifier_mem_usage,
    }
}

pub fn run_voleith_benchmark(
    c: &mut Criterion,
    group_name: &str,
    circuit_path: &str,
    private_path: &str,
    public_path: &str,
    create_transcript_fn: fn() -> Transcript,
    monitoring_config: Option<MonitoringConfig>,
) {
    assert!(Path::new(circuit_path).exists(), "Circuit file does not exist at {}", circuit_path);
    assert!(
        Path::new(private_path).exists(),
        "Private input file does not exist at {}",
        private_path
    );
    assert!(Path::new(public_path).exists(), "Public input file does not exist at {}", public_path);

    println!("\n====== {} BENCHMARK START ======", group_name);

    // Get circuit size as parameter for throughput measurements
    let circuit_bytes = read_file_cached(Path::new(circuit_path))
        .unwrap_or_else(|_| panic!("Failed to read circuit file at {}", circuit_path));
    let circuit_size = circuit_bytes.len();
    println!("Circuit size: {} bytes", circuit_size);

    let mut group = c.benchmark_group(group_name);
    group.sample_size(10); // Run 10 times for Criterion measurements
    group.throughput(Throughput::Bytes(circuit_size as u64));

    println!("Running detailed benchmark with 10 iterations...");

    // Use provided monitoring config or default
    let config = monitoring_config.unwrap_or_default();
    let benchmark_result = run_voleith_proof(
        circuit_path,
        private_path,
        public_path,
        create_transcript_fn,
        Some(config),
    );

    println!("Running Criterion measurements for proof generation...");
    group.bench_function("proof_generation_time", |b| {
        b.iter_custom(|iters| {
            let mut total_time = Duration::ZERO;

            for _ in 0..iters {
                // Set up for proof generation - reuse cached circuit data
                let circuit_bytes = read_file_cached(Path::new(circuit_path))
                    .unwrap_or_else(|_| panic!("Failed to read circuit file at {}", circuit_path));
                let circuit_bytes_slice = circuit_bytes.as_slice();

                // Create a buffer with pre-allocated capacity
                let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
                let mut circuit = Cursor::new(circuit_buffer);
                let mut transcript_instance = create_transcript_fn();
                let rng = &mut thread_rng();

                // Measure proof generation time
                let start = Instant::now();
                let _proof = Proof::<InsecureVole>::prove::<_, _>(
                    &mut circuit,
                    Path::new(private_path),
                    &mut transcript_instance,
                    rng,
                )
                .expect("Failed to generate proof");
                total_time += start.elapsed();
            }

            total_time
        });
    });

    println!("Running Criterion measurements for verification...");
    group.bench_function("proof_verification_time", |b| {
        b.iter_custom(|iters| {
            // Generate the proof once outside the timing loop - reuse cached circuit data
            let circuit_bytes = read_file_cached(Path::new(circuit_path))
                .unwrap_or_else(|_| panic!("Failed to read circuit file at {}", circuit_path));
            let circuit_bytes_slice = circuit_bytes.as_slice();

            // Create a buffer with pre-allocated capacity
            let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
            let mut circuit = Cursor::new(circuit_buffer);
            let mut transcript_instance = create_transcript_fn();
            let rng = &mut thread_rng();

            let proof = Proof::<InsecureVole>::prove::<_, _>(
                &mut circuit,
                Path::new(private_path),
                &mut transcript_instance,
                rng,
            )
            .expect("Failed to generate proof");

            let mut total_time = Duration::ZERO;

            // Pre-allocate verification transcripts
            let mut verification_transcripts = Vec::with_capacity(iters as usize);
            for _ in 0..iters {
                verification_transcripts.push(create_transcript_fn());
            }

            for i in 0..iters {
                // Create a new cursor for each verification
                let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
                let mut circuit = Cursor::new(circuit_buffer);
                let verification_transcript = &mut verification_transcripts[i as usize];

                // Measure verification time
                let start = Instant::now();
                let _ = proof.verify(&mut circuit, verification_transcript);
                total_time += start.elapsed();
            }

            total_time
        });
    });

    // --- Report comprehensive metrics ---
    println!("\n====== {} BENCHMARK RESULTS ======", group_name);
    println!(
        "Proof Generation Time: {:?} ({} ms)",
        Duration::from_millis(benchmark_result.proof_generation_time_ms),
        benchmark_result.proof_generation_time_ms
    );
    println!(
        "Proof Verification Time: {:?} ({} ms)",
        Duration::from_millis(benchmark_result.proof_verification_time_ms),
        benchmark_result.proof_verification_time_ms
    );
    println!("Proof Size: {} bytes", benchmark_result.proof_size_bytes);
    println!("Communication Overhead: {} bytes", benchmark_result.communication_overhead_bytes);
    println!("Circuit Size: {} bytes", circuit_size);
    println!("Prover Computation Load:");
    println!("  - CPU Usage: {:.2}%", benchmark_result.prover_cpu_usage);
    println!("  - Memory Usage: {:.2} MB", benchmark_result.prover_memory_usage_mb);
    println!("Verifier Computation Load:");
    println!("  - CPU Usage: {:.2}%", benchmark_result.verifier_cpu_usage);
    println!("  - Memory Usage: {:.2} MB", benchmark_result.verifier_memory_usage_mb);

    println!("====== {} BENCHMARK COMPLETE ======\n", group_name);

    group.finish();
}
