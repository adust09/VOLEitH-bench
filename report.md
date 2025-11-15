
## TL;DR

- [Grant Proposal](https://hackmd.io/zPhb0H0kQMqNmw5m324wsQ)
- [Benchmark Result](https://github.com/adust09/VOLEitH-bench/tree/main/benches)
- [SNARK wrapper](https://github.com/adust09/swanky/tree/dev/schmivitz-snark)

In this report, based on the results of milestones 1 and 2,
we report on the implementation results and issues of the SNARK Wrapping (Groth16) approach of [VOLE itH](https://eprint.iacr.org/2023/996).
In Milestone 1, we considered multiple approaches as described below and ultimately evaluated the SNARK Wrapping approach in Milestone 2.  

The evaluation using SHA-256 that we had planned at the time of the grant application could not be implemented due to an explosive increase in the number of constraints in the SNARK phase. Similarly, we did not perform measurements in a mobile environment.

## 1. Feasibility analysis and key findings from Milestone 1 and 2

We considered three methods to make the Proof size of VOLE itH more compact and enable on-chain verification.

### 1.1 SNARK Wrapping(Groth16)

We started with a simple approach of wrapping VOLE itH verification logic in SNARK.

Achieved constant proof size of `1,055` bytes and gas cost of `208,967` gas regardless of circuit complexity, but identified critical constraint growth bottleneck of `16,640×n + 2,113,664` that makes complex circuits impractical.

The VOLE itH library is [schmivitz](https://github.com/GaloisInc/swanky/tree/dev/schmivitz), and we proved the VOLE itH verification logic with [arkworks](https://arkworks.rs/).Furthermore, we made it verifiable on EVM using [Solidity Verifier](https://github.com/Tetration-Lab/arkworks-solidity-verifier).
These implementations are as follows:

- [schmivitz-snark](https://github.com/adust09/swanky/tree/dev/schmivitz-snark): SNARK wrapper for VOLE itH in the Head verification
- [VOLEitH-bench](https://github.com/adust09/VOLEitH-bench): e2e benchmark using schmivitz-snark

### 1.2 Folding

We also considered an approach using Folding schemes in the verification logic.
VOLE itH verification logic is well structured. Using Folding schemes could potentially reduce proof size.  Details of this approach are already shown in [this post](https://hackmd.io/-WhvA3wCQoOjZ-feMTQvdQ), so we'll explain briefly here.

In VOLE itH verification logic, reconstructing the [GGM Tree](https://crypto.stanford.edu/pbc/notes/crypto/ggm.html) is costly.
![スクリーンショット 2025-06-05 11.57.29](https://hackmd.io/_uploads/SJOjvYCGgg.png)

Since this GGM Tree consists of PRG and Hash functions, we thought efficient reconstruction would be possible if these could be instantiated with NIVC.
However, we did not adopt this approach because [Sonobe](https://github.com/privacy-scaling-explorations/sonobe) does not support NIVC, and Schmivitz is an [insecure VOLE implementation](https://github.com/GaloisInc/swanky/blob/dev/schmivitz/src/vole/insecure.rs) that does not implement GGM Tree reconstruction part.

Implementing this approach in the future would be useful, so we leave it as future work.

### 1.3 Blob

Initially, we examined whether proofs could be stored in Blobs, but since EVMs cannot access Blobs, we abandoned this approach.

## 2. Comparative analysis of on-chain verifiers

We report the benchmark results obtained using SNARK Wrapping (Groth16).
We had planned to compare multiple approaches, but as mentioned above, we determined that Folding and Blob could not be evaluated at this point.

We use VOLE itH circuits composed only of ADD gates or AND gates. 

Machine specs: MacBook Air M1 16GB RAM, 8-core CPU

**VOLE phase Metrics**

| Metric                         | 100 ADD gate    | 100 AND gate    | 1000 ADD gate   | 1000 AND gate    |
| ------------------------------ | --------------- | --------------- | --------------- | ---------------- |
| Proof generation Time                   | 279.012µs       | 476.5µs         | 790.062µs       | 1.649608ms       |
| Proof verification Time              | 68.75µs         | 274.566µs       | 585.6µs         | 1.082441ms       |
| Proof Size                     | 21,361 bytes    | 42,491 bytes    | 21,319<br>bytes | 233,175<br>bytes |
| Communication Overhead         | 21,426 bytes    | 42,556 bytes    | 21,384 bytes    | 233,240 bytes    |
| VOLE Prover Computation Load   | 0.07%, 13.27 MB | 0.05%, 12.88 MB | 0.05%, 13.09 MB | 0.03%, 13.67 MB  |
| VOLE Verifier Computation Load | 0.04%, 15.19 MB | 0.10%, 14.94 MB | 0.05%, 15.53 MB | 0.04%, 16.22 MB  |

**SNARK phase Metrics**

| Metric                         | 100 ADD gate   | 100 AND gate       | 1000 ADD gate  | 1000 AND gate    |
| ------------------------------ | -------------- | ------------------ | -------------- | ---------------- |
| Proof generation Time                   | 272 ms         | 1,794 ms           | 324 ms         | 8,003 ms         |
| SNARK Prover Computation Load  | 0.05%, 91.11MB | 0.04%, 2,873.20 MB | 0.03%, 88.75MB | 0.05%, 966,22 MB |
| Number of constraints          | 86,080         | 3,471,680          | 86,080         | 33,942,080       |
| Proof Size                     | 1,055 bytes    | 1,055 bytes        | 1,055 bytes    | 1,055 bytes      |
| On-Chain Verification Gas Cost | 208,967 gas    | 208,967 gas        | 208,967 gas    | 208,967 gas      |

**Constraints per gate:**
- ADD gate: ~86 constraints per gate(1000 ADD)
- AND gate: ~34,000 constraints per gate(1000 AND)

These results revealed the following points:

- AND gates significantly increase proof time and VOLE proof size compared to ADD gates
- ADD gates have constant constraints while AND gates have constraints that increase proportionally to the number of gates
- SNARK proof generation time is longer than VOLE proof time and verification time, becoming the main bottleneck in end-to-end performance
- SNARK proof size and on-chain verification gas cost could be kept constant regardless of circuit size or gate type
- Overall proof time heavily depends on the SNARK phase, and the challenge is that constraints increase dramatically as more AND gates are included in the VOLE circuit.

In other words, the bottleneck is caused by the explosive growth of the constraint during the SNARK phase.
Complex circuits require a significant amount of proof generation time on the client side, making them impractical for realistic use cases without innovative solutions.

Our measured [VOLE itH benchmark for sha256](https://github.com/adust09/VOLEitH-bench?tab=readme-ov-file#vole-ith-bench-mark) showed 15x faster proof time than Circom implementation, but this advantage was lost due to constraint explosion in the SNARK phase. In fact, we attempted to verify the SHA-256 E2E benchmark but gave up because there were over `20,000` AND gates.

## 3. Insights into SNARK integration for VOLE itH

Here we consider the causes of explosive growth in constraint count during the SNARK phase presented in the previous chapter and potential solutions.

### 3.1 Constraints count

Let `n` = `extended_witness_length` (number of secret inputs + number of multiplication gates), then total complexity becomes `O(n)`.
First, we break down the implementation of [schmivitz-snark](https://github.com/adust09/swanky/tree/dev/schmivitz-snark) and identify where the constraints are increasing.

**Linear increasing constraints gadgets**

| Gadget                          | Constraint count |
| ------------------------------ | ---------------- |
| `compute_d_delta`              | 128n             |
| `compute_masked_witness`       | 256n             |
| `compute_validation_aggregate` | 16,512n          |
| Total linear terms             | ~16,640n         |

We can see that `compute_validation_aggregate` is dominant. Particularly, internal field multiplication operations (128²) are the main bottleneck.

**Constant constraints gadgets**

| Gadget | Constraint count |
|--------|------------------|
| `combine` | ~2,097,152 |
| `compute_actual_validation` | ~16,384 |
| `Final equality check` | ~128 |
| Total constant terms | ~2,113,664 |

Therefore, total constraint count = `16,640 × n + 2,113,664`

### 3.2 Field Mapping
Next, we explain the cause of the bottleneck in detail.
The biggest cause of explosive constraint growth is field mapping.
[schmivitz](https://github.com/GaloisInc/swanky/tree/dev/schmivitz) operates on the following binary fields:

- F2: GF(2) - only 0 and 1
- F8b: GF(2^8) - 8-bit binary field
- F64b: GF(2^64) - 64-bit binary field
- F128b: GF(2^128) - 128-bit binary field

However, Bn254Fr used in [arkworks](https://github.com/arkworks-rs/groth16) and binary fields are fundamentally different, making it difficult to directly express binary field operations in Bn254Fr.
Therefore, we represent binary fields as Boolean arrays as follows.

```Rust
// schmivitz-snark/src/constraints.rs
pub fn build_circuit(
    cs: ark_relations::r1cs::ConstraintSystemRef<Bn254Fr>,
    schmivitz_proof: Proof<InsecureVole>,
) -> VoleVerificationBoolean {
    // Convert binary field elements to boolean arrays

    // Convert witness commitment (F64b) to boolean arrays
    let witness_commitment_booleans: Vec<Vec<Boolean<Bn254Fr>>> = schmivitz_proof
        .witness_commitment
        .iter()
        .map(|value| f64b_to_boolean_array(cs.clone(), value).unwrap())
        .collect();

    // Convert witness challenges (F128b) to boolean arrays
    let witness_challenges_booleans: Vec<Vec<Boolean<Bn254Fr>>> = schmivitz_proof
        .witness_challenges
        .iter()
        .map(|value| f128b_to_boolean_array(cs.clone(), value).unwrap())
        .collect();
    ...
```

Thus, constraint count explodes in the SNARK phase because variables need to be treated as Boolean arrays rather than single field elements.

Constraint count particularly increases for processing related to AND gates.
This is fundamentally due to VOLE-based zk not being suitable for non-linear operations.

The following code snippet handles `witness_challenge` required for AND gate evaluation. Since 128×128 bit processing occurs here, $2^{14}$ constraints are needed per AND gate.

```Rust
//schmivitz-snark/src/gadgets/validation.rs
    // For each bit in challenge
    for (i, challenge_bit) in challenge.iter().enumerate() {
        if i >= 128 {
            break;
        }

        // For each bit in masked_witness
        for (j, masked_bit) in masked_witness.iter().enumerate() {
            if j >= 128 || i + j >= 128 {
                continue;
            }

            // Compute challenge_bit AND masked_bit
            let and_result = Boolean::and(challenge_bit, masked_bit)?;
            // XOR with the corresponding bit in product (at position i+j)
            product[i + j] = Boolean::xor(&product[i + j], &and_result)?;
        }
    }
```

`witness_challenge` is a variable specific to AND gates and is not needed for ADD gates.
In other words, `compute_validation_aggregate` is executed proportionally to the number of AND gates, causing constraint increase.

## 4. Identification of technical bottlenecks and solutions

Here we consider the identified bottlenecks and their solutions.
As mentioned in Chapter 3, explosive growth in constraint count due to Field Mapping is the biggest bottleneck. Also, GGM tree reconstruction, which we could not address in this project, needs to be considered.

### 4.1 Field Mapping

The [Mystique](https://eprint.iacr.org/2021/730) paradigm might provide hints for Field Mapping problems. Mystique is a VOLE-based zk specialized for non-linear functions in machine learning, focusing on efficient data conversion between $F_2$ and $F_p$

Furthermore, recent research shows that using [Lookup Tables based on Mystique](https://eprint.iacr.org/2025/507.pdf) can significantly reduce execution time and communication volume.

This research achieves approximately 61x to 130x speedup in execution time compared to Mystique for the following functions. Communication costs are also reduced by approximately 1.4x to 2.9x.

| Function                       | Protocol | Execution Time (seconds)  | Communication (MB) |
| ----------------------------- | -------- | ---------------------------------- | ------------------ |
| Exponential                   | Mystique with LUT     | 8.696                              | 99.020             |
|                               | Mystique | 1130.020                           | 291.435            |
| Division                      | Mystique with LUT     | 9.837                              | 110.684            |
|                               | Mystique | 617.690                            | 160.428            |
| Reciprocal Square Root        | Mystique with LUT     | 11.836                             | 147.903            |
|                               | Mystique | 824.639                            | 212.211            |

Adapting Mystique's construction methods and Lookup Tables to VOLE itH could potentially significantly reduce constraint count.

### 4.2 GGM Tree

In schmivitz, the most complex GGM Tree reconstruction process in the verification step is omitted. However, when SNARK Wrapping, this process is expected to further increase constraint count.

Fortunately, [GGM Tree optimization techniques](https://eprint.iacr.org/2024/490) have been proposed by the VOLE itH authors. Also, `FAESTER` has been proposed as an evolution of `FAEST`, improving signature size (Proof size) by about 10%. Signature time(Proof time) and verification time maintain equivalent or improved performance.
(Security level 128 bits, AMD Ryzen 9 7900X 12-core CPU, 128GB memory, Ubuntu 22.04)

| Scheme  | Version | Signature Size (bytes) | Signature Time (ms) | Verification Time (ms) |
| ------- | ------- | ---------------------- | ------------------- | ---------------------- |
| FAEST   | Slow| 50,063                  | 4.3813              | 4.1023                 |
|         | Fast| 63,363                  | 0.4043              | 0.3953                 |
| FAESTER | Slow| 45,943                  | 3.2823              | 4.4673                 |
|         | Fast| 60,523                  | 0.4333              | 0.6103                 |

>Note: Slow versions have larger signature sizes and longer signature/verification times. Fast versions have faster signature/verification times but larger signature sizes.

Furthermore, using the Folding scheme technique we devised in Milestone 1 could improve the constraint count required for GGM Tree reconstruction.

## 5. Future Research Directions

Based on our analysis, we have identified the following three priority research areas to address scalability issues in on-chain verification of VOLE itH:

**Priority 1: Field mapping optimization based on Mystique**
Applying the Mystique paradigm to efficiently convert F2 to Fp and combining it with lookup tables is the most promising short-term solution. Recent studies have shown performance improvements ranging from 61x to 130x, which could directly address our constraint explosion bottleneck.

**Priority 2: Optimization of GGM Tree Reconstruction**
Combining the optimized GGM tree reconstruction technique proposed in FAESTER with our folding scheme approach proposed in Milestone 1 could significantly reduce verification complexity in VOLE implementation in production environments.

**Priority 3: Alternative SNARK Systems**
Binius, which is suitable for binary field operations, could bring fundamental improvements. The use of Recursive SNARKs should also be considered.

## 6. Conclusion

In this project, we successfully demonstrated the feasibility of SNARK wrapping for verifying VOLE itH. We achieved a constant proof size and gas cost, regardless of circuit complexity.

However,  the incompatibility between the binary field used in VOLE itH and the prime field of the SNARK circuit requires a costly Boolean array representation, which makes computing complex circuits impractical. Although we achieved the target proof size compression, practical deployment requires addressing the field mapping overhead.

This can be done through approaches such as Mystique-based optimization, lookup tables, and GGM tree optimization techniques. Future research will focus on these areas.

## 7. References

- [Publicly Verifiable Zero-Knowledge and Post-Quantum Signatures From VOLE-in-the-Head
](https://eprint.iacr.org/2023/996)
- [Mystique: Efficient Conversions for Zero-Knowledge
Proofs with Applications to Machine Learning](https://eprint.iacr.org/2021/730.pdf)
- [FAEST: Algorithm Specifications Version 1.1](https://faest.info/faest-spec-v1.1.pdf)
- [Scalable Zero-knowledge Proofs for Non-linear Functions in Machine Learning](https://eprint.iacr.org/2025/507.pdf)
- [One Tree to Rule Them All: Optimizing GGM Trees and OWFs for Post-Quantum Signatures](https://eprint.iacr.org/2024/490)
- [Shorter VOLE-in-the-Head-based Signatures from Vector Semi-Commitment](https://eprint.iacr.org/2025/1077)
- https://github.com/GaloisInc/swanky
- https://github.com/CryptMatrix/ZKMath

