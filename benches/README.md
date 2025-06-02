
## VOLE itH bench mark

Before experimenting with SNARK Proof, we measured the efficiency of VOLE itH by conducting benchmarks on keccak_f and sha256.

Machine specifications:Apple M1 with 16GB memory

```bash
cargo bench --bench voleith
```

> ⚠️This command is not work now, should be fixed

| Metric                   | sha256        | keccak_f      |
|--------------------------|---------------|---------------|
| Proof Generation Time    | 95 ms         | 143 ms        |
| Proof Proof Verification Time  | 51 ms         | 74 ms         |
| Proof Size               | 4,927,342 B   | 8,416,569 B   |
| Communication Overhead   | 4,927,407 B   | 8,416,634 B   |
| Prover Computation Load  | 0.02% CPU, 118.23 MB | 0.04% CPU, 154.14 MB |
| Verifier Computation Load| 0.04% CPU, 138.89 MB | 0.04% CPU, 158.1 MB |

- Compared to the results of the [sha256 Circom implementation](https://eprint.iacr.org/2023/681.pdf), the Proof time performance is 15.5 times better. However, the Proof size is more than 6000 times larger.
- `sha256` and `keccak_f` are the circuits implemented at [Bristol Fashion](https://github.com/GaloisInc/swanky/tree/dev/bristol-fashion/circuits) transpiled to SIEVE IR and compiled by [Schmivitz](https://github.com/adust09/swanky/tree/dev/schmivitz). Currently we are copy and paste the generated circuits to this project.

- If you want to benchmark any other circuits implemented in Bristol Fashion, you can transpile Bristol Fashion circuit to SIEVE IR circuit by [bristol2sieve](https://github.com/adust09/swanky/tree/dev/bristol2sieve). Private Input files are not generated, so prepare them by yourself.

## E2E Benchmark Results

Next, we measured each point from end to end until the generation of VOLE itH Proof and SNARK Proof.

Machine specifications:Apple M1 with 16GB memory
```bash
cargo bench --bench e2e
```

The Solidity file is generated automatically, but deployment and verification must be performed manually. For details, refer to [/foundry/README.md](https://github.com/adust09/VOLEitH-bench/blob/main/foundry/README.md). This is the E2E benchmark with 10 and 1000 boolean gates.

**VOLE phase Metrics**

| Metric                         | 100 add gate    | 100 and gate    | 1000 add gate   | 1000 and gate    |
| ------------------------------ | --------------- | --------------- | --------------- | ---------------- |
| Proof Generation Time                   | 279.012µs       | 476.5µs         | 790.062µs       | 1.649608ms       |
| Proof Verification Time              | 68.75µs         | 274.566µs       | 585.6µs         | 1.082441ms       |
| Proof Size                     | 21,361 bytes    | 42,491 bytes    | 21,319<br>bytes | 233,175<br>bytes |
| Communication Overhead         | 21,426 bytes    | 42,556 bytes    | 21,384 bytes    | 233,240 bytes    |
| Circuit Size                   | 2,971 bytes     | 2,971 bytes     | 30,876 bytes    | 30,876 bytes     |
| VOLE Prover Computation Load   | 0.07%, 13.27 MB | 0.05%, 12.88 MB | 0.05%, 13.09 MB | 0.03%, 13.67 MB  |
| VOLE Verifier Computation Load | 0.04%, 15.19 MB | 0.10%, 14.94 MB | 0.05%, 15.53 MB | 0.04%, 16.22 MB  |

**SNARK phase Metrics**

| Metric                         | 100 add gate   | 100 and gate       | 1000 add gate  | 1000 and gate    |
| ------------------------------ | -------------- | ------------------ | -------------- | ---------------- |
| Proof Generation Time                   | 272 ms         | 1,794 ms           | 324 ms         | 8,003 ms         |
| SNARK Prover Computation Load  | 0.05%, 91.11MB | 0.04%, 2,873.20 MB | 0.03%, 88.75MB | 0.05%, 966,22 MB |
| Number of constraints          | 86,080         | 3,471,680          | 86,080         | 33,942,080       |
| Proof Size                     | 1,055 bytes    | 1,055 bytes        | 1,055 bytes    | 1,055 bytes      |
| On-Chain Verification Gas Cost | 208,967 gas    | 208,967 gas        | 208,967 gas    | 208,967 gas      |
