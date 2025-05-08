use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;

mod utils;
use utils::{common::run_detailed_benchmark, major::MonitoringConfig};

fn create_transcript() -> Transcript {
    Transcript::new(b"sha256 transcript")
}

fn sha256_single(c: &mut Criterion) {
    let circuit_path = "circuits/sha256/single/circuit.txt";
    let private_path = "circuits/sha256/single/private.txt";
    let public_path = "circuits/sha256/single/public.txt";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
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

        run_detailed_benchmark(
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

criterion_group!(benches, sha256_single, sha256_hash_chain_10);
criterion_main!(benches);
