# Solana Escrow Smart Contract

[![Solana](https://img.shields.io/badge/Solana-Blockchain-blueviolet)](https://solana.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Overview

**Pinocchio Escrow** is a highly optimized, native Solana smart contract implementing a secure and trustless token-based escrow mechanism. Built with the lightweight Pinocchio SDK, it facilitates atomic, peer-to-peer token exchanges utilizing on-chain Program Derived Address (PDA) vaults, ensuring that transactions occur reliably without the need for centralized intermediaries.

## Features

- **Trustless Execution**: Facilitates secure SPL token swaps between two independent parties without custodian risk.
- **PDA-Secured Vaults**: Employs Program Derived Addresses to programmatically lock and manage funds with cryptographic certainty.
- **Fail-Safe Refund Mechanism**: Allows the original maker to securely reclaim their deposited assets if an offer remains unfulfilled.
- **Atomic Swaps**: Guarantees that either both parties successfully exchange their targeted tokens, or the entire transaction securely reverts.
- **Highly Optimized Architecture**: Developed in Rust directly on the Solana ABI using the zero-dependency Pinocchio SDK for maximum performance and minimal compute unit consumption.

## How It Works

1. **Make**: The initiating party (Maker) drafts an escrow offer by depositing their SPL tokens into a securely derived PDA vault while specifying their desired token and quantity in return.
2. **Take**: A responding party (Taker) fulfills the predefined conditions by supplying the requested tokens, triggering an atomic, two-way transfer of assets.
3. **Refund**: In the event the offer is no longer desired, the Maker retains the absolute authority to retrieve their deposited assets and safely close the escrow account.

## Account Structure

- **Maker**: The principal entity initiating and funding the initial escrow offer.
- **Taker**: The counterparty executing and finalizing the atomic trade conditions.
- **Vault**: A secure, isolated Token Account derived via PDA that holds the Maker's tokens during the escrow period.
- **Escrow**: A dedicated state account preserving all immutable trade parameters, expectations, and deterministic seeds.

## Instructions

- `Make`: Initializes the escrow state and cryptographically secures the Maker's tokens within the Vault.
- `Take`: Validates the Taker's assets and executes the infallible atomic swap.
- `Refund`: Reverts the pending escrow securely, returning the Maker's initial deposit and recovering network lamports.

## Usage

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable environment)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18.x or newer recommended)

### Build

Compile the program using the standard Solana BPF/SBF toolchain:
```sh
cargo build-sbf
```

### Test

Execute the integrated binary test suite:
```sh
cargo test-sbf
```

### Deploy

Deploy the compiled shared object to your targeted Solana cluster:
```sh
solana program deploy target/deploy/pinocchio_escrow.so
```

## Example Workflow

1. **Initiation**: The Maker constructs and dispatches a `Make` instruction to broadcast their targeted offer to the network.
2. **Execution**: A designated or arbitrary Taker resolves the trade by submitting a valid `Take` instruction with matching parameters.
3. **Cancellation**: If market conditions change or the offer goes unfulfilled, the Maker invokes `Refund` to securely abort the trade and reclaim capital.

## License

This project is distributed under the [MIT License](LICENSE).