#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/block_resources.wasm \
  --source <TU_PUBLIC_MULTISIG> \
  --network testnet
echo "Contrato desplegado. ID: $(stellar contract id --network testnet --source <TU_PUBLIC_MULTISIG> --wasm target/wasm32-unknown-unknown/release/block_resources.wasm)"
