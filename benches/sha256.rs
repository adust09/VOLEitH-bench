use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;

mod common;
use common::{run_detailed_benchmark, MonitoringConfig};

fn create_transcript() -> Transcript {
    Transcript::new(b"sha256 transcript")
}

fn sha256_f2_single(c: &mut Criterion) {
    let f2_circuit_path = "circuits/sha256/f2/single/circuit.txt";
    let f2_private_path = "circuits/sha256/f2/single/private.txt";
    let f2_public_path = "circuits/sha256/f2/single/public.txt";

    let monitoring_config =
        MonitoringConfig { enabled: true, refresh_interval_ms: 50, stabilization_delay_ms: 100 };

    run_detailed_benchmark(
        c,
        "sha256_f2_single",
        f2_circuit_path,
        f2_private_path,
        f2_public_path,
        create_transcript,
        Some(monitoring_config),
    );
}

fn sha256_f2_hash_chain_10(c: &mut Criterion) {
    let f2_circuit_path = "circuits/sha256/f2/hashchain/circuit.txt";
    let f2_private_path = "circuits/sha256/f2/hashchain/private.txt";
    let f2_public_path = "circuits/sha256/f2/hashchain/public.txt";

    if Path::new(f2_circuit_path).exists()
        && Path::new(f2_private_path).exists()
        && Path::new(f2_public_path).exists()
    {
        let monitoring_config = MonitoringConfig {
            enabled: true,
            refresh_interval_ms: 50,
            stabilization_delay_ms: 100,
        };

        run_detailed_benchmark(
            c,
            "sha256_f2_hash_chain_10",
            f2_circuit_path,
            f2_private_path,
            f2_public_path,
            create_transcript,
            Some(monitoring_config),
        );
    } else {
        println!("One or more files do not exist.");
    }
}

criterion_group!(benches, sha256_f2_single, sha256_f2_hash_chain_10);
criterion_main!(benches);
