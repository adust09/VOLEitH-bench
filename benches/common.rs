use criterion::{Criterion, Throughput};
use merlin::Transcript;
use once_cell::sync::Lazy;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use std::{fs, io::Cursor, path::Path, time::Instant};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub proof_generation_time_ms: u64,
    pub proof_verification_time_ms: u64,
    pub proof_size_bytes: usize,
    pub communication_overhead_bytes: usize,
    pub prover_cpu_usage: f32,
    pub prover_memory_usage_mb: f64,
    pub verifier_cpu_usage: f32,
    pub verifier_memory_usage_mb: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub refresh_interval_ms: u64,
    pub stabilization_delay_ms: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            refresh_interval_ms: 50,     // Reduced from 100ms
            stabilization_delay_ms: 100, // Reduced from 200ms
        }
    }
}

// Global cache for file contents to avoid repeated file I/O
static FILE_CACHE: Lazy<Mutex<HashMap<String, Vec<u8>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Read file contents efficiently, using cache when possible
/// Returns file content as bytes
pub fn read_file_cached(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let path_str = path.to_string_lossy().to_string();

    // Try to get from cache first
    {
        let cache = FILE_CACHE.lock().unwrap();
        if let Some(content) = cache.get(&path_str) {
            return Ok(content.clone());
        }
    }

    // Not in cache, read from disk
    let content = fs::read(path)?;

    // Store in cache for future use
    {
        let mut cache = FILE_CACHE.lock().unwrap();
        cache.insert(path_str, content.clone());
    }

    Ok(content)
}

/// Initialize system monitoring and get the initial CPU/memory values
pub fn init_system_monitoring(config: &MonitoringConfig) -> Option<System> {
    if !config.enabled {
        return None;
    }

    let mut system = System::new_all();

    // Only refresh specific components we need instead of all
    system.refresh_processes();
    system.refresh_memory();
    system.refresh_cpu();

    // Sleep to allow initial measurements to settle (configurable)
    if config.stabilization_delay_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(config.stabilization_delay_ms));
    }

    // Must refresh again to establish the baseline for CPU measurements
    system.refresh_processes();
    system.refresh_cpu();

    Some(system)
}

/// Get process resource usage from a System instance
/// Note: cpu_usage() already returns the delta since last refresh
pub fn get_process_usage(system_opt: &mut Option<System>, config: &MonitoringConfig) -> (f32, f64) {
    // If monitoring is disabled, return zeros
    let Some(system) = system_opt else {
        return (0.0, 0.0);
    };

    // Refresh only the specific components we need
    system.refresh_processes();
    system.refresh_memory();
    system.refresh_cpu();

    // Sleep briefly to allow CPU measurement to register (configurable)
    if config.refresh_interval_ms > 0 {
        std::thread::sleep(std::time::Duration::from_millis(config.refresh_interval_ms));
    }

    // Refresh again to get measurements that account for the time passed
    system.refresh_processes();
    system.refresh_cpu();

    let pid = std::process::id();
    if let Some(process) = system.process(sysinfo::Pid::from_u32(pid)) {
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory() as f64 / 1024.0 / 1024.0;
        return (cpu_usage, memory_usage);
    }

    (0.0, 0.0)
}

/// Run the proof generation and verification process with detailed measurements
/// Performs 10 runs and averages the results
pub fn run_proof(
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

    let circuit_for_proof = &mut Cursor::new(circuit_bytes_slice);
    let mut transcript_for_proof = create_transcript_fn();
    let rng_for_proof = &mut thread_rng();

    let mut total_proving_time = Duration::ZERO;

    // Generate the first proof and measure its time
    let start_main = Instant::now();
    let proof = Proof::<InsecureVole>::prove::<_, _>(
        circuit_for_proof,
        private_input_path,
        &mut transcript_for_proof,
        rng_for_proof,
    )
    .expect("Failed to generate main proof");
    total_proving_time += start_main.elapsed();

    // Run additional (NUM_RUNS-1) iterations for a total of NUM_RUNS
    for i in 1..NUM_RUNS {
        let circuit_run = &mut Cursor::new(circuit_bytes_slice);
        let mut transcript_run = create_transcript_fn();
        let rng_run = &mut thread_rng();

        let start = Instant::now();
        let _ = Proof::<InsecureVole>::prove::<_, _>(
            circuit_run,
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
    let public_input_content = read_file_cached(public_input_path).unwrap_or_else(|_| Vec::new());
    let public_input_size = public_input_content.len();
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
    for i in 0..NUM_RUNS {
        let circuit_verify = &mut Cursor::new(circuit_bytes_slice);
        let mut verification_transcript = create_transcript_fn();

        let start = Instant::now();
        let verification_result = proof.verify(circuit_verify, &mut verification_transcript);
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

pub fn run_detailed_benchmark(
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
    let benchmark_result =
        run_proof(circuit_path, private_path, public_path, create_transcript_fn, Some(config));

    println!("Running Criterion measurements for proof generation...");
    group.bench_function("proof_generation_time", |b| {
        b.iter_custom(|iters| {
            let mut total_time = Duration::ZERO;

            for _ in 0..iters {
                // Set up for proof generation - reuse cached circuit data
                let circuit_bytes = read_file_cached(Path::new(circuit_path))
                    .unwrap_or_else(|_| panic!("Failed to read circuit file at {}", circuit_path));
                let circuit_bytes_slice = circuit_bytes.as_slice();
                let circuit = &mut Cursor::new(circuit_bytes_slice);
                let mut transcript_instance = create_transcript_fn();
                let rng = &mut thread_rng();

                // Measure proof generation time
                let start = Instant::now();
                let _proof = Proof::<InsecureVole>::prove::<_, _>(
                    circuit,
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
            let circuit = &mut Cursor::new(circuit_bytes_slice);
            let mut transcript_instance = create_transcript_fn();
            let rng = &mut thread_rng();

            let proof = Proof::<InsecureVole>::prove::<_, _>(
                circuit,
                Path::new(private_path),
                &mut transcript_instance,
                rng,
            )
            .expect("Failed to generate proof");

            let mut total_time = Duration::ZERO;

            for _ in 0..iters {
                // Reset circuit cursor for verification
                let circuit = &mut Cursor::new(circuit_bytes_slice);
                let mut verification_transcript = create_transcript_fn();

                // Measure verification time
                let start = Instant::now();
                let _ = proof.verify(circuit, &mut verification_transcript);
                total_time += start.elapsed();
            }

            total_time
        });
    });

    // --- Report comprehensive metrics ---
    println!("\n====== {} BENCHMARK RESULTS ======", group_name);
    println!("--- Performance Metrics (10-run average) ---");
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

    println!("\n--- Size Metrics ---");
    println!("Proof Size: {} bytes", benchmark_result.proof_size_bytes);
    println!("Communication Overhead: {} bytes", benchmark_result.communication_overhead_bytes);
    println!("Circuit Size: {} bytes", circuit_size);

    println!("\n--- Resource Usage Metrics ---");
    println!("Prover Computation Load:");
    println!("  - CPU Usage: {:.2}%", benchmark_result.prover_cpu_usage);
    println!("  - Memory Usage: {:.2} MB", benchmark_result.prover_memory_usage_mb);
    println!("Verifier Computation Load:");
    println!("  - CPU Usage: {:.2}%", benchmark_result.verifier_cpu_usage);
    println!("  - Memory Usage: {:.2} MB", benchmark_result.verifier_memory_usage_mb);

    println!("====== {} BENCHMARK COMPLETE ======\n", group_name);

    group.finish();
}
