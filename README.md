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
| Communication Overhead   | Total amount of data exchanged between prover and verifier                                 | Bytes               | Capture communication logs and measure the total data exchanged                               |
| On-Chain Verification Gas Cost| Cost of verifying the proof on-chain in gas                                           | Gas units           | Measure gas cost using a blockchain environment (e.g., Ethereum)                             |

##  VOLE itH bench mark

Results on a test machine (Apple M1 with 16GB memory)
You can run the benchmarks by yourself using the following scripts
```bash
cargo bench --bench voleith
```

| Metric                   | sha256        | keccak_f      |
|--------------------------|---------------|---------------|
| Proof Generation Time    | 95 ms         | 143 ms        |
| Proof Verification Time  | 51 ms         | 74 ms         |
| Proof Size               | 4,927,342 B   | 8,416,569 B   |
| Communication Overhead   | 4,927,407 B   | 8,416,634 B   |
| Prover Computation Load  | 0.02% CPU, 118.23 MB | 0.04% CPU, 154.14 MB |
| Verifier Computation Load| 0.04% CPU, 138.89 MB | 0.04% CPU, 158.1 MB |

- Compared to the results of the [sha256 Circom implementation](https://eprint.iacr.org/2023/681.pdf), the Proof time performance is 15.5 times better. However, the Proof size is more than 6000 times larger.
- `sha256` and `keccak_f` are the circuits implemented at [Bristol Fashion](https://github.com/GaloisInc/swanky/tree/dev/bristol-fashion/circuits) transpiled to SIEVE IR and compiled by [Schmivitz](https://github.com/adust09/swanky/tree/dev/schmivitz). 
  - Currently we are copying the generated circuits to this project.

- If you want to benchmark any other circuits implemented in Bristol Fashion, you can transpile Bristol Fashion circuit to SIEVE IR circuit by [bristol2sieve](https://github.com/adust09/swanky/tree/dev/bristol2sieve)
  - Private Input files are not generated, so prepare them by yourself.

## E2E Benchmark Results

This is the E2E benchmark with 10 and 1000 boolean gates.

**VOLE phase Metrics**

| Metric                         | 100 add gate    | 100 and gate    | 1000 add gate   | 1000 and gate    |
| ------------------------------ | --------------- | --------------- | --------------- | ---------------- |
| Proving Time                   | 279.012µs       | 476.5µs         | 790.062µs       | 1.649608ms       |
| Verification Time              | 68.75µs         | 274.566µs       | 585.6µs         | 1.082441ms       |
| Proof Size                     | 21,361 bytes    | 42,491 bytes    | 21,319<br>bytes | 233,175<br>bytes |
| Communication Overhead         | 21,426 bytes    | 42,556 bytes    | 21,384 bytes    | 233,240 bytes    |
| Circuit Size                   | 2,971 bytes     | 2,971 bytes     | 30,876 bytes    | 30,876 bytes     |
| VOLE Prover Computation Load   | 0.07%, 13.27 MB | 0.05%, 12.88 MB | 0.05%, 13.09 MB | 0.03%, 13.67 MB  |
| VOLE Verifier Computation Load | 0.04%, 15.19 MB | 0.10%, 14.94 MB | 0.05%, 15.53 MB | 0.04%, 16.22 MB  |

**SNARK phase Metrics**

| Metric                         | 100 add gate   | 100 and gate       | 1000 add gate  | 1000 and gate    |
| ------------------------------ | -------------- | ------------------ | -------------- | ---------------- |
| Proving Time                   | 272 ms         | 1,794 ms           | 324 ms         | 8,003 ms         |
| SNARK Prover Computation Load  | 0.05%, 91.11MB | 0.04%, 2,873.20 MB | 0.03%, 88.75MB | 0.05%, 966,22 MB |
| Number of constraints          | 86,080         | 3,471,680          | 86,080         | 33,942,080       |
| Proof Size                     | 1,055 bytes    | 1,055 bytes        | 1,055 bytes    | 1,055 bytes      |
| On-Chain Verification Gas Cost | 208,967 gas    | 208,967 gas        | 208,967 gas    | 208,967 gas      |



As the number of gates increases, the proof time also increases, but due to the characteristics of Groth16, the proof size remains constant and gas cost is also kept constant.

Unfortunately, representing a circuit like sha256 in SIEVE IR would require more than 7M constraints (it's close to $2^{23}$ ptau file) in the current implementation.This is not realistic for client-side use, which would take too long to prove.
