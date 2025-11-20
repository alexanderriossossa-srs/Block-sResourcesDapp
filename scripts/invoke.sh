#!/bin/bash
stellar contract invoke \
  --source <PUBLIC_MULTISIG> \
  --network testnet \
  --id <CONTRACT_ID> \
  log_trace \
  -- b"RECURSO_001" <PUB_A> <PUB_RECEIVER> '{ "tipo": "agua", "cantidad": "100L" }' \
  --signer <SECRET_A> --signer <SECRET_B>  # Dos firmas
