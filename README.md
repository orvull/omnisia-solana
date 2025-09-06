# Solana Lottery Examples

This repository contains two sample lottery programs for the Solana blockchain:

- `anchor-lottery` – implementation using the [Anchor](https://github.com/coral-xyz/anchor) framework.
- `native-lottery` – implementation using only the core `solana-program` SDK.

Each version lets players buy tickets and allows an authority to draw a winner who receives the accumulated pot.

## Running on WSL2 Ubuntu

These steps set up a fresh WSL2 Ubuntu installation for Solana development:

1. **Install WSL2 and Ubuntu (from Windows PowerShell):**
   ```bash
   wsl --install Ubuntu
   ```
2. **Update packages and install build tools:**
   ```bash
   sudo apt update && sudo apt upgrade -y
   sudo apt install -y build-essential pkg-config libssl-dev
   ```
3. **Install Rust toolchain:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
4. **Install the Solana CLI:**
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.15/install)"
   echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```
5. **Install Anchor CLI (optional for Anchor version):**
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
   ```

## Building the programs

### Anchor version
```bash
cd anchor-lottery
cargo test       # compiles the program
anchor build     # build using Anchor (requires Anchor CLI)
```

### Native version
```bash
cd native-lottery
cargo test
```

Deploying these programs to a cluster requires the Solana toolchain (e.g. `solana-test-validator` and `solana program deploy`). Refer to the Solana and Anchor documentation for detailed deployment instructions.

