// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Verifier, Pairing} from "../src/verifier.sol";
import {VerifyProofScript} from "../script/verify.s.sol";
import {Test, console} from "forge-std/Test.sol";

contract VerifyProofTest is Test {
    VerifyProofScript public verifyProofScript;
    Verifier public verifier;
    address public verifierAddress;

    function setUp() public {
        verifier = new Verifier();
        verifierAddress = address(verifier);
        verifyProofScript = new VerifyProofScript();
    }

    function testVerifyProof() public {
        vm.setEnv("VERIFIER_ADDRESS", vm.toString(verifierAddress));
        verifyProofScript.run();        
        console2.log("Gas used:", gasleft());
    }
}
