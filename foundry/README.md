# Foundry for solidity verifier

## Deploy solidity verifier

1. Start anvil

```shell
$ anvil
```

2. Set your private key as an environment variable:

Open another shell and run this script, you can find private key in anvil window.private key whould appear in the anvil window when you start it in step 1.

```shell
$ export PRIVATE_KEY=<your_private_key>
```

3. Deploy the VOLE Verifier contract
The script will deploy the Verifier contract and output its address. Save this address as you'll need it for proof verification.

```shell
$ forge script script/deploy.s.sol:VOLEVerifierScript --rpc-url 127.0.0.1:8545 --broadcast
```

## Verify proof

To verify a SNARK proof using the verification script

1. Make sure you have a `snark_proof.json` file in the `script` directory containing the proof data in the following format:

```json
{
    "a": ["<aX>", "<aY>"],
    "b": [["<bX0>", "<bX1>"], ["<bY0>", "<bY1>"]],
    "c": ["<cX>", "<cY>"]
}
```

2. Set the `VERIFIER_ADDRESS` environment variable to the address of your deployed Verifier contract:

```shell
$ export VERIFIER_ADDRESS=<your_verifier_contract_address>
```

3. Run the verification script:

```shell
$ forge script script/verify.s.sol:VerifyProofScript --rpc-url 127.0.0.1:8545
```

The script will read the proof from `snark_proof.json`, parse it, and verify it using the deployed Verifier contract. The verification result will be printed to the console.

## Bench gas usage

You can get gas usage

```shell
$ forge test --gas-report -vvvv
```
