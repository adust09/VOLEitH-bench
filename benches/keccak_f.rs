use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;
use std::path::Path;

mod common;
use common::{run_detailed_benchmark, MonitoringConfig};

/// Create a transcript for keccak_f benchmarks
fn create_transcript() -> Transcript {
    Transcript::new(b"keccak_f benchmark transcript")
}

fn keccak_f_benchmark(c: &mut Criterion) {
    // Benchmark f2 field
    let f2_circuit_path = "circuits/keccak_f/f2/circuit.txt";
    let f2_private_path = "circuits/keccak_f/f2/private.txt";
    let f2_public_path = "circuits/keccak_f/f2/public.txt";

    // Create a monitoring config with reduced overhead
    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "keccak_f_f2",
        f2_circuit_path,
        f2_private_path,
        f2_public_path,
        create_transcript,
        Some(monitoring_config),
    );

    // Benchmark f64 field if available
    if Path::new("circuits/keccak_f/f64").exists() {
        let f64_circuit_path = "circuits/keccak_f/f64/circuit.txt";
        let f64_private_path = "circuits/keccak_f/f64/private.txt";
        let f64_public_path = "circuits/keccak_f/f64/public.txt";

        if Path::new(f64_circuit_path).exists()
            && Path::new(f64_private_path).exists()
            && Path::new(f64_public_path).exists()
        {
            // Reuse the same monitoring config
            run_detailed_benchmark(
                c,
                "keccak_f_f64",
                f64_circuit_path,
                f64_private_path,
                f64_public_path,
                create_transcript,
                Some(monitoring_config),
            );
        }
    }
}

criterion_group!(benches, keccak_f_benchmark);
criterion_main!(benches);
