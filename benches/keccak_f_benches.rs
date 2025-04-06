use criterion::{criterion_group, criterion_main, Criterion};
use merlin::Transcript;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use std::{fs, io::Cursor, path::Path};

fn keccak_f_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccak_f");

    // Benchmark f2 field
    benchmark_field(&mut group, "f2");

    // Benchmark f64 field if available
    if Path::new("circuits/keccak_f/f64").exists() {
        benchmark_field(&mut group, "f64");
    }

    group.finish();
}

fn benchmark_field(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    field: &str,
) {
    let circuit_path = format!("circuits/keccak_f/{}/circuit.txt", field);
    let private_input_path = format!("circuits/keccak_f/{}/private.txt", field);

    // Read circuit file
    let circuit_path_obj = Path::new(&circuit_path);
    let circuit_bytes = fs::read_to_string(circuit_path_obj)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at {:?}", circuit_path_obj));
    let circuit_bytes_slice = circuit_bytes.as_bytes();

    // Read private input file
    let private_input_path_obj = Path::new(&private_input_path);
    if !private_input_path_obj.exists() {
        panic!("Private input file does not exist at {:?}", private_input_path_obj);
    }

    group.bench_function(format!("keccak_f_{}", field), |b| {
        b.iter(|| {
            // Set up for proof generation
            let circuit = &mut Cursor::new(circuit_bytes_slice);
            let mut transcript_instance = create_transcript();
            let rng = &mut thread_rng();

            // Generate the proof
            let proof = Proof::<InsecureVole>::prove::<_, _>(
                circuit,
                private_input_path_obj,
                &mut transcript_instance,
                rng,
            )
            .unwrap();

            // Reset circuit cursor for verification
            let circuit = &mut Cursor::new(circuit_bytes_slice);

            // Create a new transcript for verification
            let mut verification_transcript = create_transcript();

            // Verify the proof
            proof.verify(circuit, &mut verification_transcript).unwrap();
        });
    });
}

fn create_transcript() -> Transcript {
    Transcript::new(b"keccak_f benchmark transcript")
}

criterion_group!(benches, keccak_f_benchmark);
criterion_main!(benches);
