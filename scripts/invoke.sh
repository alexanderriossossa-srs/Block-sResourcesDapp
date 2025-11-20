#!/bin/bash
# Invoke log_trace
soroban contract invoke \
  --source <PUBLIC_MULTISIG> \
  --network testnet \
  --wasm target/wasm32-unknown-unknown/release/block_resources.wasm \
  log_trace \
  -- <RESOURCE_ID> <FROM> <TO> '{ "tipo": "agua", "cantidad": "100L" }' \
  --signer <SECRET_A> --signer <SECRET_B>  # Dos firmas
