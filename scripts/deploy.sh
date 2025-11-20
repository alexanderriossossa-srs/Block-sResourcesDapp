#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/block_resources.wasm \
  --network testnet \
  --source <TU_SECRET_KEY_MULTISIG> \
  --permission-set-name admin  # Opcional para permisos
echo "Contrato desplegado. ID: $(soroban contract id --network testnet --source <TU_SECRET_KEY_MULTISIG> --wasm target/wasm32-unknown-unknown/release/block_resources.wasm)"
