# Project Overview

This is a Rust project for benchmarking the "VOLE in the Head" (VOLEitH) zero-knowledge proof system. The primary goal is to evaluate the feasibility and performance of on-chain verification for VOLEitH proofs. The project is structured as a Rust library and a command-line interface (CLI) application. It uses various cryptographic libraries, including `ark-groth16`, `ark-bn254`, and a custom `schmivitz` library, to implement and benchmark the proof system.

The project also includes a benchmarking suite using `criterion` to measure performance metrics such as proof generation time, verification time, proof size, and computational load (CPU and memory usage).

# Building and Running

## Building the Project

To build the project, use the standard Cargo command:

```bash
cargo build --release
```

## Running the CLI

The project includes a CLI for generating proofs. The following command can be used to generate a proof using the `keccak_f` algorithm:

```bash
cargo run --release -- prove --algorithm keccak_f
```

## Running the Benchmarks

To run the performance benchmarks, use the following command:

```bash
cargo bench
```

The benchmark results, including detailed reports, will be available in the `target/criterion` directory.

# Development Conventions

## Code Style

The project follows standard Rust conventions. Code is formatted using `rustfmt`.

## Testing

The project uses a combination of unit tests and integration tests. The core logic is tested through the benchmarking suite in the `benches` directory.

## Dependencies

The project's dependencies are managed using `Cargo.toml`. Key dependencies include:

-   `schmivitz` and `schmivitz-snark`: Core libraries for VOLEitH proofs.
-   `arkworks` libraries (`ark-groth16`, `ark-bn254`, etc.): For zero-knowledge proof implementations.
-   `criterion`: For performance benchmarking.
-   `clap`: For the command-line interface.

# Paper Writing Guidelines

This section outlines the guidelines for writing the research paper located in the `paper/` directory.

## Target Publication

The paper is to be submitted to the **SCIS (Symposium on Cryptography and Information Security)**, a Japanese security conference.

## Target Audience

The intended audience is individuals who are **not deeply familiar with Ethereum or zero-knowledge proofs**.

## Writing Style and Structure

-   The paper must be structured as a formal academic paper.
-   The language and structure should be tailored to clearly convey the significance of this research to the target audience.
-   Use logical explanations, mathematical formulas, and diagrams to explain complex concepts.
-   The paper should be written in Japanese.
