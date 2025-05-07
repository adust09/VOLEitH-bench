use eyre::{Result, WrapErr};
use merlin::Transcript;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use std::{fs, io::Cursor, path::Path};
// for snark
use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
use ark_snark::SNARK;
use arkworks_solidity_verifier::SolidityVerifier;
use schmivitz_snark::build_circuit;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::File, io::Write};
use tempfile::tempdir;
use tracing_subscriber::layer::SubscriberExt;

pub fn prove(
    circuit_path_str: &str,
    private_input_path_str: &str,
    public_input_path_str: &str,
    proof_output_path: &str,
) -> Result<()> {
    // Read circuit and input files
    let circuit_path = Path::new(circuit_path_str);
    let circuit_bytes = fs::read_to_string(circuit_path)
        .wrap_err_with(|| format!("Failed to read circuit file at {:?}", circuit_path))?;
    let circuit_bytes_slice = circuit_bytes.as_bytes();

    let private_input_path = Path::new(private_input_path_str);
    if !private_input_path.exists() {
        return Err(eyre::eyre!("Private input file does not exist at {:?}", private_input_path));
    }

    let public_input_path = Path::new(public_input_path_str);
    if !public_input_path.exists() {
        return Err(eyre::eyre!("Public input file does not exist at {:?}", public_input_path));
    }

    // Set up for proof generation
    let circuit = &mut Cursor::new(circuit_bytes_slice);
    let mut transcript_instance = create_transcript();
    let rng = &mut thread_rng();

    // Generate the proof
    let proof = Proof::<InsecureVole>::prove::<_, _>(
        circuit,
        private_input_path,
        &mut transcript_instance,
        rng,
    )
    .wrap_err("Failed to generate proof")?;
    println!("Proof generation successful!");

    // Write proof to file
    let proof_string = format!("{:?}", proof);
    fs::write(proof_output_path, &proof_string)
        .wrap_err_with(|| format!("Failed to write proof to file at {}", proof_output_path))?;

    println!("Proof written to {}", proof_output_path);

    // Reset circuit cursor for verification
    let circuit = &mut Cursor::new(circuit_bytes_slice);

    // Create a new transcript for verification
    let mut verification_transcript = create_transcript();

    // Verify the proof
    proof.verify(circuit, &mut verification_transcript).wrap_err("Proof verification failed")?;

    println!("Proof verification successful!");

    Ok(())
}

fn create_transcript() -> Transcript {
    Transcript::new(b"basic happy test transcript")
}

pub fn snark() -> Result<()> {
    let mut layer = ConstraintLayer::default();
    layer.mode = TracingMode::OnlyConstraints;
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    let _guard = tracing::subscriber::set_default(subscriber);
    // target circuit
    let circuit_str = "version 2.0.0;
        circuit;
        @type field 2;
        @begin
            $0 ... $4 <- @private(0);
            $5 <- @add(0: $0, $0);
            $6 <- @add(0: $0, $1);
            $7 <- @add(0: $0, $2);
            $8 <- @add(0: $0, $3);
            $9 <- @add(0: $0, $4);
            $10 <- @mul(0: $0, $5);
            $11 <- @mul(0: $0, $6);
            $12 <- @mul(0: $0, $7);
            $13 <- @mul(0: $0, $8);
            $14 <- @mul(0: $0, $9);
        @end ";
    let circuit = Cursor::new(circuit_str.as_bytes());

    let private_input_bytes = "version 2.0.0;
        private_input;
        @type field 2;
        @begin
            < 1 >;
            < 1 >;
            < 1 >;
            < 0 >;
            < 0 >;
        @end";

    let dir = tempdir().unwrap();
    let private_input_path = dir.path().join("private_inputs");
    let mut private_input = File::create(private_input_path.clone()).unwrap();
    writeln!(private_input, "{}", private_input_bytes).unwrap();

    let mut transcript = Transcript::new(b"schmivitz-snark");
    let rng = &mut thread_rng();
    let schmivitz_proof: Proof<InsecureVole> = Proof::<InsecureVole>::prove(
        &mut circuit.clone(),
        &private_input_path,
        &mut transcript,
        rng,
    )?;

    // validate proof
    let mut test_verify_transcript = Transcript::new(b"schmivitz-snark");
    schmivitz_proof
        .verify(&mut circuit.clone(), &mut test_verify_transcript)
        .expect("Verification should succeed");

    // Create a constraint system for boolean conversions
    let cs = ConstraintSystem::<Bn254Fr>::new_ref();

    // Build the circuit using boolean arrays
    let circuit = build_circuit(cs.clone(), schmivitz_proof.clone());

    let mut rng = ark_std::test_rng();
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

    let solidity_verifier = Groth16::<Bn254>::export(&vk);
    let output_dir = Path::new("foundry/src");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }
    let output_path = output_dir.join("vole_verifier.sol");
    fs::write(&output_path, solidity_verifier)?;
    println!("Solidity verifier generated at: {}", output_path.display());

    let public_input = vec![];

    let snark_proof = Groth16::prove(&pk, circuit, &mut rng)?;
    let is_valid = Groth16::verify(&vk, &public_input, &snark_proof)?;

    println!(
        "Verified SNARK proof with boolean arrays: {}",
        if is_valid { "VALID" } else { "INVALID" }
    );

    // Serialize the proof to JSON
    // We need to create a serializable representation of the proof
    #[derive(Serialize, Deserialize)]
    struct SerializableProof {
        a: [String; 2],
        b: [[String; 2]; 2],
        c: [String; 2],
    }

    // Helper function to extract hex value from Fp256 string
    fn extract_hex(s: String) -> String {
        // Extract the hex value between quotes and parentheses
        // Format is typically: Fp256 "(0x123...)"
        if let Some(start) = s.find('(') {
            if let Some(end) = s.find(')') {
                let hex = &s[start + 1..end];
                // Remove quotes if present
                return hex.trim_matches('"').to_string();
            }
        }
        // If parsing fails, return the original string
        s
    }

    // Convert the proof to our serializable format
    let serializable_proof = SerializableProof {
        a: [extract_hex(snark_proof.a.x.to_string()), extract_hex(snark_proof.a.y.to_string())],
        b: [
            [
                extract_hex(snark_proof.b.x.c0.to_string()),
                extract_hex(snark_proof.b.x.c1.to_string()),
            ],
            [
                extract_hex(snark_proof.b.y.c0.to_string()),
                extract_hex(snark_proof.b.y.c1.to_string()),
            ],
        ],
        c: [extract_hex(snark_proof.c.x.to_string()), extract_hex(snark_proof.c.y.to_string())],
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&serializable_proof)?;

    // Write to file
    fs::write("snark_proof.json", &json)?;
    println!("SNARK proof serialized to snark_proof.json");

    Ok(())
}
