use core::num;
use std::{
    fs,
    io::Cursor,
    path::Path,
    time::{Duration, Instant},
};

use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSystem;
use ark_snark::SNARK;
use arkworks_solidity_verifier::SolidityVerifier;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use merlin::Transcript;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use schmivitz_snark::build_circuit;

// Import the serialize module from the crate
extern crate voleith_bench;
use utils::e2e::{
    create_buffer_with_capacity, get_process_usage, init_system_monitoring, read_file_cached,
    BenchmarkResult, MonitoringConfig,
};

mod utils;

fn create_transcript() -> Transcript {
    Transcript::new(b"e2e transcript")
}

fn e2e(c: &mut Criterion) {
    let circuit_path = "circuits/sample_circuit.txt";
    let private_path = "circuits/sample_private.txt";
    let public_path = "circuits/sample_public.txt";
    let proof_output_path = "foundry/src/proof.json";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_e2e_benchmark(
        c,
        "sample",
        circuit_path,
        private_path,
        public_path,
        proof_output_path,
        create_transcript,
        Some(monitoring_config),
    );
}
fn _sha256_single(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/single/circuit.txt";
    let private_path = "circuits/sha256/single/private.txt";
    let public_path = "circuits/sha256/single/public.txt";
    let proof_output_path = "foundry/script/snark_proof.json";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_e2e_benchmark(
        c,
        "sha256_single_e2e",
        circuit_path,
        private_path,
        public_path,
        proof_output_path,
        create_transcript,
        Some(monitoring_config),
    );
}

fn _sha256_hash_chain_10(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/hashchain/circuit.txt";
    let private_path = "circuits/sha256/hashchain/private.txt";
    let public_path = "circuits/sha256/hashchain/public.txt";
    let proof_output_path = "foundry/script/snark_proof.json";

    if Path::new(circuit_path).exists()
        && Path::new(private_path).exists()
        && Path::new(public_path).exists()
    {
        let monitoring_config = MonitoringConfig {
            enabled: true,
            refresh_interval_ms: 50,
            stabilization_delay_ms: 100,
        };

        run_e2e_benchmark(
            c,
            "sha256_hash_chain_10_e2e",
            circuit_path,
            private_path,
            public_path,
            proof_output_path,
            create_transcript,
            Some(monitoring_config),
        );
    } else {
        println!("One or more files do not exist.");
    }
}

fn _keccak_f_single(c: &mut Criterion) {
    let circuit_path = "circuits/keccak_f/circuit.txt";
    let private_path = "circuits/keccak_f/private.txt";
    let public_path = "circuits/keccak_f/public.txt";
    let proof_output_path = "foundry/script/snark_proof.json";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_e2e_benchmark(
        c,
        "keccak_f_e2e",
        circuit_path,
        private_path,
        public_path,
        proof_output_path,
        create_transcript,
        Some(monitoring_config),
    );
}

// criterion_group!(benches, sample, sha256_single, sha256_hash_chain_10, keccak_f_single);
criterion_group!(benches, e2e);
criterion_main!(benches);

pub fn run_e2e_proof(
    circuit_path_str: &str,
    private_input_path_str: &str,
    public_input_path_str: &str,
    proof_output_path: &str,
    create_transcript_fn: fn() -> Transcript,
    monitoring_config: Option<MonitoringConfig>,
) -> BenchmarkResult {
    // Use provided monitoring config or default
    let config = monitoring_config.unwrap_or_default();
    // Number of runs to average
    const NUM_RUNS: u32 = 1;

    // Read circuit and input files efficiently
    let circuit_path = Path::new(circuit_path_str);
    let circuit_bytes = read_file_cached(circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at {:?}", circuit_path));
    let circuit_bytes_slice = circuit_bytes.as_slice();

    let private_input_path = Path::new(private_input_path_str);
    let public_input_path = Path::new(public_input_path_str);

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

    let prove_duration = total_proving_time / NUM_RUNS;

    let (prover_cpu_usage, prover_mem_usage) = get_process_usage(&mut prover_system, &config);

    let proof_string = format!("{:?}", proof);
    let proof_size_bytes = proof_string.len();

    let public_input_content = read_file_cached(public_input_path).unwrap_or_else(|_| Vec::new());
    let public_input_size = public_input_content.len();

    let communication_overhead_bytes = proof_size_bytes + public_input_size;

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
    let (verifier_cpu_usage, verifier_mem_usage) = get_process_usage(&mut verifier_system, &config);

    // Create a constraint system for boolean conversions
    let cs = ConstraintSystem::<Bn254Fr>::new_ref();

    // Build the circuit using boolean arrays
    let circuit = build_circuit(cs.clone(), proof.clone());
    let mut rng = ark_std::test_rng();
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

    let public_input = vec![];

    // ----- SNARK PROOF GENERATION MEASUREMENTS -----
    let mut snark_system = init_system_monitoring(&config);

    let snark_start = Instant::now();
    let snark_proof =
        Groth16::prove(&pk, circuit.clone(), &mut rng).expect("Failed to generate SNARK proof");
    let snark_duration = snark_start.elapsed();

    let is_valid =
        Groth16::verify(&vk, &public_input, &snark_proof).expect("Failed to verify SNARK proof");

    println!("SNARK proof verification result: {}", if is_valid { "VALID" } else { "INVALID" });

    let snark_proof_string = format!("{:?}", snark_proof);
    let snark_proof_size_bytes = snark_proof_string.len();

    let (snark_cpu_usage, snark_mem_usage) = get_process_usage(&mut snark_system, &config);

    if let Err(e) = std::fs::create_dir_all(Path::new(proof_output_path).parent().unwrap()) {
        println!("Warning: Failed to create directory for proof output: {}", e);
    }

    // Try to save the proof using the project's serialization module
    match std::fs::create_dir_all(Path::new(proof_output_path).parent().unwrap()) {
        Ok(_) => {
            // Use the project's serialization module to save the proof
            if let Err(e) =
                voleith_bench::serialize::serialize_and_save_proof(&snark_proof, proof_output_path)
            {
                println!("Warning: Failed to save proof to {}: {}", proof_output_path, e);
            } else {
                println!("SNARK proof saved to: {}", proof_output_path);
            }
        }
        Err(e) => println!("Warning: Failed to create directory for proof output: {}", e),
    }

    BenchmarkResult {
        proof_generation_time_ns: prove_duration.as_nanos() as u64,
        proof_verification_time_ns: verify_duration.as_nanos() as u64,
        proof_size_bytes,
        communication_overhead_bytes,
        prover_cpu_usage,
        prover_memory_usage_mb: prover_mem_usage,
        verifier_cpu_usage,
        verifier_memory_usage_mb: verifier_mem_usage,
        snark_cpu_usage,
        snark_memory_usage_mb: snark_mem_usage,
        snark_proof_generation_time_ms: snark_duration.as_millis() as u64,
        snark_proof_size_bytes,
    }
}

pub fn run_e2e_benchmark(
    c: &mut Criterion,
    group_name: &str,
    circuit_path: &str,
    private_path: &str,
    public_path: &str,
    proof_output_path: &str,
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

    let mut group = c.benchmark_group(group_name);
    group.sample_size(10); // Run 10 times for Criterion measurements
    group.throughput(Throughput::Bytes(circuit_size as u64));

    // Use provided monitoring config or default
    let config = monitoring_config.unwrap_or_default();
    let benchmark_result = run_e2e_proof(
        circuit_path,
        private_path,
        public_path,
        proof_output_path,
        create_transcript_fn,
        Some(config),
    );

    group.bench_function("vole_proof_generation_time", |b| {
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

    group.bench_function("vole_proof_verification_time", |b| {
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

    // Add SNARK proof generation benchmark
    group.bench_function("snark_proof_generation_time", |b| {
        b.iter_custom(|iters| {
            let mut total_time = Duration::ZERO;

            for _ in 0..iters {
                // Generate a VOLE proof first
                let circuit_bytes = read_file_cached(Path::new(circuit_path))
                    .unwrap_or_else(|_| panic!("Failed to read circuit file at {}", circuit_path));
                let circuit_bytes_slice = circuit_bytes.as_slice();

                let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
                let mut circuit = Cursor::new(circuit_buffer);
                let mut transcript_instance = create_transcript_fn();
                let rng = &mut thread_rng();

                let vole_proof = Proof::<InsecureVole>::prove::<_, _>(
                    &mut circuit,
                    Path::new(private_path),
                    &mut transcript_instance,
                    rng,
                )
                .expect("Failed to generate VOLE proof");

                let cs = ConstraintSystem::<Bn254Fr>::new_ref();

                // Measure SNARK proof generation time
                let start = Instant::now();

                let circuit = build_circuit(cs.clone(), vole_proof.clone());

                let mut rng = ark_std::test_rng();
                let (pk, _vk) =
                    Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

                let _public_input: Vec<Bn254Fr> = Vec::new();
                let _snark_proof = Groth16::prove(&pk, circuit.clone(), &mut rng)
                    .expect("Failed to generate SNARK proof");
                println!("num_constraints:{:?}\n", cs.num_constraints());

                total_time += start.elapsed();
            }

            total_time
        });
    });

    // Add SNARK proof verification benchmark
    group.bench_function("snark_proof_verification_time", |b| {
        b.iter_custom(|iters| {
            // Generate a VOLE proof first
            let circuit_bytes = read_file_cached(Path::new(circuit_path))
                .unwrap_or_else(|_| panic!("Failed to read circuit file at {}", circuit_path));
            let circuit_bytes_slice = circuit_bytes.as_slice();

            let circuit_buffer = create_buffer_with_capacity(circuit_bytes_slice);
            let mut circuit = Cursor::new(circuit_buffer);
            let mut transcript_instance = create_transcript_fn();
            let rng = &mut thread_rng();

            let vole_proof = Proof::<InsecureVole>::prove::<_, _>(
                &mut circuit,
                Path::new(private_path),
                &mut transcript_instance,
                rng,
            )
            .expect("Failed to generate VOLE proof");

            // Create a constraint system for boolean conversions
            let cs = ConstraintSystem::<Bn254Fr>::new_ref();

            // Build the circuit using boolean arrays
            let circuit = build_circuit(cs.clone(), vole_proof.clone());

            let mut rng = ark_std::test_rng();
            let (pk, vk) =
                Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

            let solidity_verifier = Groth16::<Bn254>::export(&vk);
            let output_dir = Path::new("foundry");
            if !output_dir.exists() {
                let _ = fs::create_dir_all(output_dir);
            }
            let output_path = output_dir.join("src/verifier.sol");
            let _ = fs::write(&output_path, solidity_verifier);
            println!("Solidity verifier generated at: {}", output_path.display());

            let public_input = vec![];

            let snark_proof = Groth16::prove(&pk, circuit.clone(), &mut rng)
                .expect("Failed to generate SNARK proof");

            let mut total_time = Duration::ZERO;

            for _ in 0..iters {
                // Measure SNARK proof verification time
                let start = Instant::now();
                let _ = Groth16::verify(&vk, &public_input, &snark_proof)
                    .expect("Failed to verify SNARK proof");
                total_time += start.elapsed();
            }

            total_time
        });
    });

    // --- Report comprehensive metrics ---
    println!("\n====== {} BENCHMARK RESULTS ======", group_name);
    println!("VOLE Proof Metrics:");
    println!(
        "  - Generation Time: {:?} ({} ms)",
        Duration::from_nanos(benchmark_result.proof_generation_time_ns),
        benchmark_result.proof_generation_time_ns / 1_000_000
    );
    println!(
        "  - Verification Time: {:?} ({} ms)",
        Duration::from_nanos(benchmark_result.proof_verification_time_ns),
        benchmark_result.proof_verification_time_ns / 1_000_000
    );
    println!("  - Proof Size: {} bytes", benchmark_result.proof_size_bytes);
    println!("  - Communication Overhead: {} bytes", benchmark_result.communication_overhead_bytes);
    println!("Circuit Size: {} bytes", circuit_size);
    println!("VOLE Prover Computation Load:");
    println!("  - CPU Usage: {:.2}%", benchmark_result.prover_cpu_usage);
    println!("  - Memory Usage: {:.2} MB", benchmark_result.prover_memory_usage_mb);
    println!("VOLE Verifier Computation Load:");
    println!("  - CPU Usage: {:.2}%", benchmark_result.verifier_cpu_usage);
    println!("  - Memory Usage: {:.2} MB", benchmark_result.verifier_memory_usage_mb);
    println!("SNARK Prover Computation Load:");
    println!(
        "  - SNARK proof generation time: {} ms",
        benchmark_result.snark_proof_generation_time_ms
    );
    println!("  - SNARK CPU Usage: {:.2}%", benchmark_result.snark_cpu_usage);
    println!("  - SNARK proof size: {} bytes", benchmark_result.snark_proof_size_bytes);
    group.finish();
}
