use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;
use std::path::Path;

mod common;
use common::{run_detailed_benchmark, MonitoringConfig};

/// Create a transcript for poseidon benchmarks
fn create_transcript() -> Transcript {
    Transcript::new(b"bench poseidon transcript")
}

fn bench_f2_single(c: &mut Criterion) {
    let circuit_path = "circuits/poseidon/f2/single/circuit.txt";
    let private_path = "circuits/poseidon/f2/single/private.txt";
    let public_path = "circuits/poseidon/f2/single/public.txt";

    // Create a monitoring config with reduced overhead
    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "F2_Single_Hash",
        circuit_path,
        private_path,
        public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

fn bench_f2_hash_chain_10(c: &mut Criterion) {
    let circuit_path = "circuits/poseidon/f2/hash_chain_10/circuit.txt";
    let private_path = "circuits/poseidon/f2/hash_chain_10/private.txt";
    let public_path = "circuits/poseidon/f2/hash_chain_10/public.txt";

    // Reuse the same monitoring config
    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "F2_Hash_Chain_10",
        circuit_path,
        private_path,
        public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

fn bench_f2_hash_chain_100(c: &mut Criterion) {
    let circuit_path = "circuits/poseidon/f2/hash_chain_100/circuit.txt";
    let private_path = "circuits/poseidon/f2/hash_chain_100/private.txt";
    let public_path = "circuits/poseidon/f2/hash_chain_100/public.txt";

    if Path::new(circuit_path).exists()
        && Path::new(private_path).exists()
        && Path::new(public_path).exists()
    {
        // Reuse the same monitoring config
        let monitoring_config = MonitoringConfig {
            enabled: true,
            refresh_interval_ms: 50,
            stabilization_delay_ms: 100,
        };

        run_detailed_benchmark(
            c,
            "F2_Hash_Chain_100",
            circuit_path,
            private_path,
            public_path,
            create_transcript,
            Some(monitoring_config),
        );
    }
}

criterion_group!(benches, bench_f2_single, bench_f2_hash_chain_10, bench_f2_hash_chain_100);
criterion_main!(benches);
