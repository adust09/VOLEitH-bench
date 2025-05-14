use eyre::{Result, WrapErr};
use merlin::Transcript;
use rand::thread_rng;
use schmivitz::{insecure::InsecureVole, Proof};
use std::{fs, io::Cursor, path::Path};

pub fn prove(circuit_path_str: &str, private_input_path_str: &str) -> Result<()> {
    // Read circuit and input files
    let circuit_path = Path::new(circuit_path_str);
    let circuit_bytes = fs::read_to_string(circuit_path)
        .wrap_err_with(|| format!("Failed to read circuit file at {:?}", circuit_path))?;
    let circuit_bytes_slice = circuit_bytes.as_bytes();

    let private_input_path = Path::new(private_input_path_str);
    if !private_input_path.exists() {
        return Err(eyre::eyre!("Private input file does not exist at {:?}", private_input_path));
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
    println!("VOLE proof generation successful!");

    // Reset circuit cursor for verification
    let circuit = &mut Cursor::new(circuit_bytes_slice);

    // Create a new transcript for verification
    let mut verification_transcript = create_transcript();
    // Verify the proof
    proof
        .verify(circuit, &mut verification_transcript)
        .wrap_err("VOLE proof verification failed")?;

    println!("VOLE proof verification successful!");
    Ok(())
}

fn create_transcript() -> Transcript {
    Transcript::new(b"basic happy test transcript")
}
