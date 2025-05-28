use ark_bn254::Bn254;
use ark_groth16::Proof as GrothProof;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;

/// Serializable representation of a Groth16 proof
#[derive(Serialize, Deserialize)]
pub struct SerializableProof {
    pub a: [String; 2],
    pub b: [[String; 2]; 2],
    pub c: [String; 2],
}

/// Helper function to extract hex value from Fp256 string
pub fn extract_hex(s: String) -> String {
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

/// Convert a Groth16 proof to a serializable format
pub fn _convert_to_serializable(snark_proof: &GrothProof<Bn254>) -> SerializableProof {
    SerializableProof {
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
    }
}

/// Serialize a proof to JSON and write it to a file
pub fn serialize_and_save_proof(snark_proof: &GrothProof<Bn254>, output_path: &str) -> Result<()> {
    // Convert the proof to our serializable format
    let serializable_proof = _convert_to_serializable(snark_proof);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&serializable_proof)?;

    // Write to file
    fs::write(output_path, &json)?;

    Ok(())
}
