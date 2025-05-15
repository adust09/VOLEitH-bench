// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Verifier} from "../src/verifier.sol";

contract VOLEVerifierScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy the Verifier contract
        Verifier verifier = new Verifier();
        console2.log("VOLE Verifier deployed at:", address(verifier));
        
        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
