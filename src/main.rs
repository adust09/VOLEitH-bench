mod prove;

use crate::prove::snark;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Prove {
        #[arg(short, long)]
        algorithm: Option<String>,

        #[arg(short, long)]
        field: Option<String>,

        #[arg(short, long)]
        circuit: Option<String>,
    },
}

fn main() -> () {
    let _ = snark();
    // let cli = Cli::parse();

    // match &cli.command {
    //     Some(Commands::Prove { algorithm, field, circuit }) => {
    //         let algorithm_type = algorithm.as_deref().unwrap_or("poseidon");
    //         let field_type = field.as_deref().unwrap_or("f2");

    //         // Circuit paths and output prefix
    //         let (circuit_path, private_path, public_path, output_prefix);

    //         match algorithm_type {
    //             "poseidon" => {
    //                 let circuit_type = circuit.as_deref().unwrap_or("single");
    //                 println!(
    //                     "Running prove with algorithm: {}, field: {}, circuit: {}",
    //                     algorithm_type, field_type, circuit_type
    //                 );

    //                 circuit_path = format!(
    //                     "circuits/{}/{}/{}/circuit.txt",
    //                     algorithm_type, field_type, circuit_type
    //                 );
    //                 private_path = format!(
    //                     "circuits/{}/{}/{}/private.txt",
    //                     algorithm_type, field_type, circuit_type
    //                 );
    //                 public_path = format!(
    //                     "circuits/{}/{}/{}/public.txt",
    //                     algorithm_type, field_type, circuit_type
    //                 );
    //                 output_prefix = format!("{}_{}_{}", algorithm_type, field_type, circuit_type);
    //             }
    //             "keccak_f" => {
    //                 println!(
    //                     "Running prove with algorithm: {}, field: {}",
    //                     algorithm_type, field_type
    //                 );
    //                 circuit_path =
    //                     format!("circuits/{}/{}/circuit.txt", algorithm_type, field_type);
    //                 private_path =
    //                     format!("circuits/{}/{}/private.txt", algorithm_type, field_type);
    //                 public_path = format!("circuits/{}/{}/public.txt", algorithm_type, field_type);
    //                 output_prefix = format!("{}_{}", algorithm_type, field_type);
    //             }
    //             _ => {
    //                 return Err(eyre::eyre!("Unsupported algorithm: {}", algorithm_type));
    //             }
    //         }

    //         // Create output path
    //         let output_path = format!("results/proofs/{}.json", output_prefix);

    //         // Run proof with the selected circuit
    //         let result = prove::prove(&circuit_path, &private_path, &public_path, &output_path)?;

    //         Ok(result)
    //     }
    //     None => {
    //         // Default behavior with improved help message
    //         println!(
    //             "No command specified. Using default: 'prove' with poseidon algorithm, f2 field and single circuit."
    //         );
    //         println!("Available options:");
    //         println!("  --algorithm: poseidon or keccak_f (default: poseidon)");
    //         println!("  --field: f2 or f64 (default: f2)");
    //         println!("  --circuit: single or hash_chain_10/100 (default: single, only applicable for poseidon)");
    //         println!("");
    //         println!("Examples:");
    //         println!("  cargo run -- prove --algorithm poseidon --field f2 --circuit hash_chain_10   # Run Poseidon hash chain with F2");
    //         println!("  cargo run -- prove --algorithm poseidon --field f64                          # Run single Poseidon with F64");
    //         println!("  cargo run -- prove --algorithm keccak_f --field f2                           # Run Keccak-F with F2");
    //         println!("  cargo run -- prove --algorithm keccak_f --field f64                          # Run Keccak-F with F64");
    //         println!("");

    //         // Same default as if called explicitly
    //         let algorithm_type = "poseidon";
    //         let field_type = "f2";
    //         let circuit_type = "single";

    //         // Use the same path construction logic as in the explicit command
    //         let (circuit_path, private_path, public_path, output_prefix) = (
    //             format!("circuits/poseidon/{}/{}/circuit.txt", field_type, circuit_type),
    //             format!("circuits/poseidon/{}/{}/private.txt", field_type, circuit_type),
    //             format!("circuits/poseidon/{}/{}/public.txt", field_type, circuit_type),
    //             format!("poseidon_{}_{}", field_type, circuit_type),
    //         );

    //         let output_path = format!("results/proofs/{}.json", output_prefix);

    //         println!(
    //             "Running prove with algorithm: {}, field: {}, circuit: {}",
    //             algorithm_type, field_type, circuit_type
    //         );
    //         let result = prove::prove(&circuit_path, &private_path, &public_path, &output_path)?;
    //         Ok(result)
    //     }
    // }
}
