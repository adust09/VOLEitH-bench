use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;
use utils::major::MonitoringConfig;

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

    todo!()
}

criterion_group!(benches, sha256_single);
criterion_main!(benches);
