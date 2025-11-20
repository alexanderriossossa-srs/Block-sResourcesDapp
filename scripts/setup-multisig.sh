#!/bin/bash
# Asume cuenta nueva: genera con soroban keys generate
# Fondos: curl "https://friendbot.stellar.org?addr=<PUBLIC_MULTISIG>"
soroban keys weights --ed25519 <SECRET_A> 1  # Peso 1 para A
soroban keys weights --ed25519 <SECRET_B> 1  # Peso 1 para B
soroban keys threshold --master 2 --low 1 --med 2 --high 2  # Umbral 2 para tx
# Aplica a cuenta: soroban transaction submit --source <PUBLIC_MULTISIG> --base-asset XLM --network testnet <TX_SET_OPTIONS>
