// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {Script, console} from "forge-std/Script.sol";
import {BlockchainEmail} from "../src/BlockchainEmail.sol";

contract DeployBlockchainEmail is Script {
    constructor() {}

    function run() external returns (BlockchainEmail) {
        vm.startBroadcast();
        BlockchainEmail blockchainEmail = new BlockchainEmail();
        vm.stopBroadcast();

        return blockchainEmail;
    }
}
