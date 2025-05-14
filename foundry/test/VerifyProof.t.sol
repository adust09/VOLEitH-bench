// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Verifier, Pairing} from "../src/vole_verifier.sol";
import {VerifyProofScript} from "../script/verify.s.sol";
import {Test, console} from "forge-std/Test.sol";

contract VerifyProofTest is Test {
    VerifyProofScript public verifyProofScript;

    function setUp() public {
        verifyProofScript = new VerifyProofScript();
    }

    function testVerifyProof() public {
        // Set the VERIFIER_ADDRESS environment variable
        address verifierAddress = vm.envAddress("VERIFIER_ADDRESS");
        vm.envString("VERIFIER_ADDRESS", vm.toString(verifierAddress));

        // Run the script's run function
        verifyProofScript.run();
    }
}
