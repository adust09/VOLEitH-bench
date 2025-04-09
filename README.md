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
| Proof Generation Time    | 99 ms         | 182 ms        |
| Proof Verification Time  | 51 ms         | 77 ms         |
| Proof Size               | 4.927655 MB   | 8.416492 MB   |
| Communication Overhead   | 4.927720 MB   | 8.416557 MB   |
| Prover Computation Load  | 0.12% CPU, 99.44 MB | 0.03% CPU, 147 MB |
| Verifier Computation Load| 0.14% CPU, 99.33 MB | 0.03% CPU, 151.34 MB |

You can run the benchmarks yourself using the following scripts:

```bash
cargo bench --bench {benchmark_name}
```
