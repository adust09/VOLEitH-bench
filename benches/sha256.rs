use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;

mod common;
use common::{run_detailed_benchmark, MonitoringConfig};

fn create_transcript() -> Transcript {
    Transcript::new(b"sha256 benchmark transcript")
}

fn sha256_f_benchmark(c: &mut Criterion) {
    let f2_circuit_path = "circuits/sha256/f2/circuit.txt";
    let f2_private_path = "circuits/sha256/f2/private.txt";
    let f2_public_path = "circuits/sha256/f2/public.txt";

    // Create a monitoring config with reduced overhead
    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "sha256_f2",
        f2_circuit_path,
        f2_private_path,
        f2_public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

criterion_group!(benches, sha256_f_benchmark);
criterion_main!(benches);
