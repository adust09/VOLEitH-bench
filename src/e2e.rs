use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSystem;
use ark_snark::SNARK;
use arkworks_solidity_verifier::SolidityVerifier;
use eyre::{Result, WrapErr};
use merlin::Transcript;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use schmivitz_snark::build_circuit;
use std::{fs, io::Cursor, path::Path};

// The serialize module is now part of the same crate
use crate::serialize;

pub fn e2e(
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
    .wrap_err("Failed to generate VOLE proof")?;
    println!("VOLE proof generation successful!");

    let mut verification_transcript = create_transcript();
    proof
        .verify(circuit, &mut verification_transcript)
        .wrap_err("VOLE proof verification failed")?;
    println!("VOLE proof verification successful!");

    // Create a constraint system for boolean conversions
    let cs = ConstraintSystem::<Bn254Fr>::new_ref();

    // Build the circuit using boolean arrays
    let circuit = build_circuit(cs.clone(), proof.clone());

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
    // Use the serialize module to save the proof
    serialize::serialize_and_save_proof(&snark_proof, proof_output_path)?;
    Ok(())
}

fn create_transcript() -> Transcript {
    Transcript::new(b"basic happy test transcript")
}
