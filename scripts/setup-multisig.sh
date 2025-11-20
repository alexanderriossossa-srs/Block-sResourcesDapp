#!/bin/bash
# Genera cuenta multisig nueva
stellar keys generate --global multisig --network testnet
PUBLIC_MULTISIG=$(stellar keys address multisig --network testnet)
echo "Nueva cuenta multisig: $PUBLIC_MULTISIG"

# Fondos con Friendbot
curl "https://friendbot.stellar.org?addr=$PUBLIC_MULTISIG"

# Config umbral y pesos (usa tx para set_options)
# Paso 1: Build tx para set threshold (master=2, med=2, etc.)
stellar transaction build \
  --source $PUBLIC_MULTISIG \
  --network testnet \
  set_options \
  --master-weight 2 \
  --low-threshold 1 \
  --med-threshold 2 \
  --high-threshold 2 \
  --signer <PUB_A> ed25519_public_key 1 \  # Peso 1 para A
  --signer <PUB_B> ed25519_public_key 1   # Peso 1 para B
  --base-fee 100 \
  > set_options_tx.json

# Paso 2: Firma y submit (requiere firma master inicial)
stellar transaction sign --source $PUBLIC_MULTISIG --network testnet --base64 set_options_tx.json --signer <SECRET_MULTISIG>
stellar transaction submit --network testnet --base64 set_options_tx.json

echo "Multisig configurado. Umbral: 2 firmas."
