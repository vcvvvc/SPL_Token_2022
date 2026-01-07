# SPL_Token_2022

[English](README.md) | [中文](README.zh.md)

This project provides Rust utilities for creating and managing SPL tokens on the Solana blockchain using the Token 2022 program.

## Features

- Create new SPL tokens with metadata
- Initialize token accounts (ATA)
- Mint tokens
- Revoke various authorities (freeze, mint, metadata update, metadata pointer) for a given token address

## Usage

1. Set up your `.env` file with the required environment variables:
   - `RPC_URL`: Solana RPC URL
   - `WALLET_PATH`: Path to your Solana wallet file
   - `METADATA_URI`: URI for the token metadata (required for creating new tokens)
   - `TOKEN_ADDRESS`: Address of the token to revoke authorities (required for revoking authorities)

2. Run `cargo run --bin create_token` to create a new SPL token.
3. Run `cargo run --bin revoke_auth` to revoke authorities for an existing token.

## Dependencies

- `solana-client`
- `solana-sdk`
- `spl-token-2022-interface`
- `spl-token-metadata-interface`
- `dotenv`

## Notes

This project uses the Solana Token 2022 program for creating and managing tokens. It includes examples for creating new tokens with metadata and revoking various authorities.