use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;

mod common;
use common::{run_detailed_benchmark, MonitoringConfig};

fn create_transcript() -> Transcript {
    Transcript::new(b"sha256 transcript")
}

fn sha256_f2_single(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/circuit.txt";
    let private_input_path = "circuits/sha256/private.txt";
    let public_input_path = "circuits/sha256/public.txt";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "sha256_f2_single",
        circuit_path,
        private_input_path,
        public_input_path,
        create_transcript,
        Some(monitoring_config),
    );
}

fn sha256_f2_hash_chain_10(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/circuit.txt";
    let private_input_path = "circuits/sha256/private.txt";
    let public_input_path = "circuits/sha256/public.txt";

    if Path::new(circuit_path).exists()
        && Path::new(private_input_path).exists()
        && Path::new(public_input_path).exists()
    {
        let monitoring_config = MonitoringConfig {
            enabled: true,
            refresh_interval_ms: 50,
            stabilization_delay_ms: 100,
        };

        run_detailed_benchmark(
            c,
            "sha256_f2_hash_chain_10",
            circuit_path,
            private_input_path,
            public_input_path,
            create_transcript,
            Some(monitoring_config),
        );
    } else {
        println!("One or more files do not exist.");
    }
}

criterion_group!(benches, sha256_f2_single, sha256_f2_hash_chain_10);
criterion_main!(benches);
