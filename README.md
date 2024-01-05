# Solana Wrapping Contract

This Rust project contains a Solana smart contract for wrapping and unwrapping SOL into WSOL (Wrapped SOL). It's built using the Anchor framework.

## Project Structure

- `lib.rs`: The main entry point for the smart contract. It includes the `wrap` and `unwrap` functions for handling SOL wrapping and unwrapping.
- `errors.rs`: Contains custom error types for the smart contract.
- `Cargo.toml`: The configuration file for the Rust project, specifying dependencies and other metadata.

## Features

- **Wrap**: Converts SOL to WSOL for transactions. If the amount of WSOL user wants to get is already present in the user WSOL token account, nothing happens, otherwise the program converts as much SOL to WSOl as needed to reach the amount of WSOL required by the user.
- **Unwrap**: Converts WSOL back to SOL, removing the WSOL tokens from the user's account and closing the user WSOL account.

## Requirements

- Rust programming environment.
- Anchor framework installed.
- Solana CLI tools.

## Building and Deploying the Project

To build the project, navigate to the project directory and run the following commands:

```bash
anchor build
anchor deploy
