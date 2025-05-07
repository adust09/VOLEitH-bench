mod e2e;
mod serialize;
mod snark;
mod voleith;

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

        #[arg(short, long)]
        field: Option<String>,

        #[arg(short, long)]
        circuit: Option<String>,
    },
    E2E {
        #[arg(short, long)]
        algorithm: Option<String>,

        #[arg(short, long)]
        field: Option<String>,

        #[arg(short, long)]
        circuit: Option<String>,
    },
}

fn main() -> Result<(), eyre::Report> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Prove { algorithm, field, circuit }) => {
            let algorithm_type = algorithm.as_deref().unwrap_or("poseidon");
            let field_type = field.as_deref().unwrap_or("f2");

            // Circuit paths and output prefix
            let (circuit_path, private_path, public_path, output_prefix);

            match algorithm_type {
                "poseidon" => {
                    let circuit_type = circuit.as_deref().unwrap_or("single");
                    println!(
                        "Running prove with algorithm: {}, field: {}, circuit: {}",
                        algorithm_type, field_type, circuit_type
                    );

                    circuit_path = format!(
                        "circuits/{}/{}/{}/circuit.txt",
                        algorithm_type, field_type, circuit_type
                    );
                    private_path = format!(
                        "circuits/{}/{}/{}/private.txt",
                        algorithm_type, field_type, circuit_type
                    );
                    public_path = format!(
                        "circuits/{}/{}/{}/public.txt",
                        algorithm_type, field_type, circuit_type
                    );
                    output_prefix = format!("{}_{}_{}", algorithm_type, field_type, circuit_type);
                }
                "keccak_f" => {
                    println!(
                        "Running prove with algorithm: {}, field: {}",
                        algorithm_type, field_type
                    );
                    circuit_path =
                        format!("circuits/{}/{}/circuit.txt", algorithm_type, field_type);
                    private_path =
                        format!("circuits/{}/{}/private.txt", algorithm_type, field_type);
                    public_path = format!("circuits/{}/{}/public.txt", algorithm_type, field_type);
                    output_prefix = format!("{}_{}", algorithm_type, field_type);
                }
                _ => {
                    return Err(eyre::eyre!("Unsupported algorithm: {}", algorithm_type));
                }
            }

            // Create output path
            let output_path = format!("results/proofs/{}.json", output_prefix);

            // Run proof with the selected circuit
            let result = voleith::prove(&circuit_path, &private_path, &public_path, &output_path)?;

            Ok(result)
        }
        Some(Commands::E2E { algorithm, field, circuit }) => {
            let algorithm_type = algorithm.as_deref().unwrap_or("poseidon");
            let field_type = field.as_deref().unwrap_or("f2");

            // Circuit paths and output prefix
            let (circuit_path, private_path, public_path, output_prefix);

            match algorithm_type {
                "poseidon" => {
                    let circuit_type = circuit.as_deref().unwrap_or("single");
                    println!(
                        "Running prove with algorithm: {}, field: {}, circuit: {}",
                        algorithm_type, field_type, circuit_type
                    );

                    circuit_path = format!(
                        "circuits/{}/{}/{}/circuit.txt",
                        algorithm_type, field_type, circuit_type
                    );
                    private_path = format!(
                        "circuits/{}/{}/{}/private.txt",
                        algorithm_type, field_type, circuit_type
                    );
                    public_path = format!(
                        "circuits/{}/{}/{}/public.txt",
                        algorithm_type, field_type, circuit_type
                    );
                    output_prefix = format!("{}_{}_{}", algorithm_type, field_type, circuit_type);
                }
                "keccak_f" => {
                    println!(
                        "Running prove with algorithm: {}, field: {}",
                        algorithm_type, field_type
                    );
                    circuit_path =
                        format!("circuits/{}/{}/circuit.txt", algorithm_type, field_type);
                    private_path =
                        format!("circuits/{}/{}/private.txt", algorithm_type, field_type);
                    public_path = format!("circuits/{}/{}/public.txt", algorithm_type, field_type);
                    output_prefix = format!("{}_{}", algorithm_type, field_type);
                }
                _ => {
                    return Err(eyre::eyre!("Unsupported algorithm: {}", algorithm_type));
                }
            }

            // Create output path
            let output_path = format!("results/proofs/{}.json", output_prefix);

            // Run proof with the selected circuit
            let result = e2e::e2e(&circuit_path, &private_path, &public_path, &output_path)?;

            Ok(result)
        }
        None => Ok(()),
    }
}
