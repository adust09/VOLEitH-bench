# Project Details

This project investigates the feasibility of on-chain verification for VOLE in the Head (VOLE itH), a publicly verifiable and efficient variant of VOLE-based zero-knowledge (ZK) proofs. While VOLE-based ZK systems significantly reduce computational complexity for provers, the challenge lies in implementing cost-effective and scalable on-chain verification.

VOLE-based ZK systems are typically designed for efficient prover but are not yet widely implemented for on-chain verification due to challenges such as proof size, communication costs, and gas costs. VOLE itH introduces public verifiability, making it a promising candidate for practical zk schemes for on-chain applications via client-side provers. However, the exact costs and technical bottlenecks of integrating VOLE itH into a public blockchain, such as Ethereum, remain unclear.

## Measurement matrix

| Metric                   | Description                                                                                 | Unit                | Example Measurement Method                                                                     |
|--------------------------|---------------------------------------------------------------------------------------------|---------------------|-----------------------------------------------------------------------------------------------|
| Proof Generation Time    | Time required for the prover to generate the proof                                         | Milliseconds (ms)   | Measure the execution time of the proof generation process                                    |
| Proof Verification Time  | Time required for the verifier to verify the proof                                         | Milliseconds (ms)   | Measure the execution time of the proof verification process                                  |
| Proof Size               | Size of the generated proof data                                                           | Bytes               | Measure the size of the proof after generation                                                |
| Prover Computation Load  | Computational cost for the prover (e.g., memory usage, CPU usage)                          | CPU Load (%), MB    | Monitor resource usage during the proof generation process                                    |
| Verifier Computation Load| Computational cost for the verifier (e.g., memory usage, CPU usage)                        | CPU Load (%), MB    | Monitor resource usage during the proof verification process                                  |
| Setup Time               | Time required for the initial system setup (if applicable)                                 | Seconds (s)         | Measure the execution time of the setup process                                               |
| Communication Overhead   | Total amount of data exchanged between prover and verifier                                 | Bytes               | Capture communication logs and measure the total data exchanged                               |
| On-Chain Verification Gas Cost| Cost of verifying the proof on-chain in gas                                           | Gas units           | Measure gas cost using a blockchain environment (e.g., Ethereum)                             |

The target of this calculation is the Proof of Hash chain.
The following two methods are used for onchain-verification.

- SNARK verification + Solidity Verifier
- Smart Contract

## Benchmark

The following benchmark results were obtained on a test machine using the VOLEitH implementation.

- Apple M1 with 16GB memory
- [VOLE itH Parameters](https://github.com/adust09/swanky/blob/dev/schmivitz/src/parameters.rs)

| Metric                   | sha256        | keccak_f      |
|--------------------------|---------------|---------------|
| Proof Generation Time    | 95 ms         | 143 ms        |
| Proof Verification Time  | 51 ms         | 74 ms         |
| Proof Size               | 4,927,342 B   | 8,416,569 B   |
| Communication Overhead   | 4,927,407 B   | 8,416,634 B   |
| Prover Computation Load  | 0.02% CPU, 118.23 MB | 0.04% CPU, 154.14 MB |
| Verifier Computation Load| 0.04% CPU, 138.89 MB | 0.04% CPU, 158.1 MB |

> Compared to the results of the [sha256 Circom implementation](https://eprint.iacr.org/2023/681.pdf), the Proof time performance is 15.5 times better. However, the Proof size is more than 6000 times larger.

You can run the benchmarks yourself using the following scripts:

```bash
cargo bench --bench {benchmark_name}
```
sha256 and keccak_f are the circuits implemented at [Bristol Fashion](https://github.com/GaloisInc/swanky/tree/dev/bristol-fashion/circuits) transpiled to SIEVE IR and compiled by Schmivitz. If you want to benchmark the circuits implemented in Bristol Fashion, please use [this](https://github.com/adust09/swanky/tree/dev/bristol2sieve) as a reference for your transpile.
Currently we are copying the generated circuits to this project.Private Input and Public Input files are not generated, so prepare them yourself.

## Sample Benchmark Results

| Metric | Value |
|--------|-------|
| **VOLE Proof Metrics** | |
| Generation Time | 0 ms (0 ns) |
| Verification Time | 0 ms (0 ns) |
| Proof Size | 22,387 bytes |
| Communication Overhead | 22,452 bytes |
| **Circuit Size** | 356 bytes |
| **VOLE Prover Computation Load** | |
| CPU Usage | 0.03% |
| Memory Usage | 12.22 MB |
| **VOLE Verifier Computation Load** | |
| CPU Usage | 0.08% |
| Memory Usage | 14.27 MB |
| **SNARK Prover Computation Load** | |
| CPU Usage | 0.03% |
| Memory Usage | 231.06 MB |
| Proof Generation Time | 135 ms |
| Proof Size | 1,055 bytes |
| **Gas Consumption** | 208,967 gas (mostly from elliptic curve pairing operations: 181,000 gas) |
