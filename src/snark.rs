use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
use ark_snark::SNARK;
use arkworks_solidity_verifier::SolidityVerifier;
use eyre::Result;
use schmivitz::{insecure::InsecureVole, Proof};
use schmivitz_snark::build_circuit;
use std::{fs, path::Path};
use tracing_subscriber::layer::SubscriberExt;

// The serialize module is now part of the same crate
use crate::serialize;

pub fn _prove(vole_proof: Proof<InsecureVole>, proof_output_path: &str) -> Result<()> {
    let mut layer = ConstraintLayer::default();
    layer.mode = TracingMode::OnlyConstraints;
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    let _guard = tracing::subscriber::set_default(subscriber);

    // Create a constraint system for boolean conversions
    let cs = ConstraintSystem::<Bn254Fr>::new_ref();

    // Build the circuit using boolean arrays
    let circuit = build_circuit(cs.clone(), vole_proof.clone());

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
