// Import modules from our library
extern crate voleith_bench;

// Re-export modules that are only used in the binary
mod serialize;
mod snark;

// Use the modules from our library
use voleith_bench::voleith;

use clap::Parser;
use eyre::Ok;

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
    },
}

fn main() -> Result<(), eyre::Report> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Prove { algorithm }) => {
            let algorithm_type = algorithm.as_deref().unwrap_or("keccak_f");

            // Circuit paths and output prefix
            let (circuit_path, private_path);

            match algorithm_type {
                "keccak_f" => {
                    println!("Running prove with algorithm: {}", algorithm_type);
                    circuit_path = format!("circuits/{}/circuit.txt", algorithm_type);
                    private_path = format!("circuits/{}/private.txt", algorithm_type);
                }
                "sha256" => {
                    println!("Running prove with algorithm: {}", algorithm_type);
                    circuit_path = format!("circuits/{}/circuit.txt", algorithm_type);
                    private_path = format!("circuits/{}/private.txt", algorithm_type);
                }
                _ => {
                    return Err(eyre::eyre!("Unsupported algorithm: {}", algorithm_type));
                }
            }

            // Run proof with the selected circuit
            let result = voleith::prove(&circuit_path, &private_path)?;

            Ok(result)
        }

        None => Ok(()),
    }
}
