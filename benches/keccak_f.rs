use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;

mod common;
use common::{run_detailed_benchmark, MonitoringConfig};

/// Create a transcript for keccak_f benchmarks
fn create_transcript() -> Transcript {
    Transcript::new(b"keccak_f benchmark transcript")
}

fn keccak_f_benchmark(c: &mut Criterion) {
    // Benchmark f2 field
    let circuit_path = "circuits/keccak_f/circuit.txt";
    let private_path = "circuits/keccak_f/private.txt";
    let public_path = "circuits/keccak_f/public.txt";

    // Create a monitoring config with reduced overhead
    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "keccak_f",
        circuit_path,
        private_path,
        public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

criterion_group!(benches, keccak_f_benchmark);
criterion_main!(benches);
