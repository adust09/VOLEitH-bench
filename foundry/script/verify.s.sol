// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Verifier, Pairing} from "../src/verifier.sol";

contract VerifyProofScript is Script {
    function setUp() public {}

    function run() public {
        // Read the proof from the JSON file
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/proof.json");
        string memory json = vm.readFile(path);
        
        // Parse a
        string memory aX = vm.parseJsonString(json, ".a[0]");
        string memory aY = vm.parseJsonString(json, ".a[1]");
        
        // Parse b
        string memory bX0 = vm.parseJsonString(json, ".b[0][0]");
        string memory bX1 = vm.parseJsonString(json, ".b[0][1]");
        string memory bY0 = vm.parseJsonString(json, ".b[1][0]");
        string memory bY1 = vm.parseJsonString(json, ".b[1][1]");
        
        // Parse c
        string memory cX = vm.parseJsonString(json, ".c[0]");
        string memory cY = vm.parseJsonString(json, ".c[1]");
        
        // Convert to uint256 - add 0x prefix for hex parsing
        uint256 aXValue = vm.parseUint(string.concat("0x", aX));
        uint256 aYValue = vm.parseUint(string.concat("0x", aY));
        uint256 bX0Value = vm.parseUint(string.concat("0x", bX0));
        uint256 bX1Value = vm.parseUint(string.concat("0x", bX1));
        uint256 bY0Value = vm.parseUint(string.concat("0x", bY0));
        uint256 bY1Value = vm.parseUint(string.concat("0x", bY1));
        uint256 cXValue = vm.parseUint(string.concat("0x", cX));
        uint256 cYValue = vm.parseUint(string.concat("0x", cY));
        
        // Get the deployed Verifier contract address
        address verifierAddress = vm.envAddress("VERIFIER_ADDRESS");
        Verifier verifier = Verifier(verifierAddress);
        
        // Create the proof struct
        Pairing.G1Point memory a = Pairing.G1Point(aXValue, aYValue);
        Pairing.G2Point memory b = Pairing.G2Point(
            [bX0Value, bX1Value],
            [bY0Value, bY1Value]
        );
        Pairing.G1Point memory c = Pairing.G1Point(cXValue, cYValue);
        
        Verifier.Proof memory proof = Verifier.Proof(a, b, c);
        
        // Verify the proof
        vm.startBroadcast();
        bool result = verifier.verifyTx(proof);
        vm.stopBroadcast();
        
        console2.log("Verification result:", result);
    }
}
