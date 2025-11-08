# message_python_min_fixed_v3.py — Soroban demo con Python SDK 13.1.0
# Corrige la clave de lectura a "Message" y decodifica SCVal manualmente.

from time import sleep
import os
from typing import Optional

from stellar_sdk import (
    Keypair,
    Network,
    TransactionBuilder,
    SorobanServer,   # RPC Soroban
    StrKey,          # decode_contract("C...") -> bytes
    scval,           # helpers: to_symbol, to_string (sí existen en tu build)
    xdr as Xdr,
)
from stellar_sdk.operation import InvokeHostFunction

# ======= CONFIG =======
RPC_URL = "https://soroban-testnet.stellar.org"
CONTRACT_ID = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL"
USER_SECRET = os.getenv("USER_SECRET", "SDQK5C2WQ67VM4HQ3S3JAQ4XIJED7SJVTGKMDAVS7R4YCT7NJ34TLLKJ")
NETWORK_PASSPHRASE = Network.TESTNET_NETWORK_PASSPHRASE
BASE_FEE = 100
# ======================

server = SorobanServer(RPC_URL)
user = Keypair.from_secret(USER_SECRET)

# ---------- Utils ----------
def wait_tx(tx_hash: str):
    """Espera hasta SUCCESS o FAILED en la red."""
    while True:
        tx = server.get_transaction(tx_hash)
        if tx.status.name == "SUCCESS":
            return tx
        if tx.status.name == "FAILED":
            raise RuntimeError(f"Falló on-chain: {tx}")
        sleep(1.0)

def scval_to_string(val: Optional[Xdr.SCVal]) -> Optional[str]:
    """SCVal -> str (solo para strings/símbolos)."""
    if val is None:
        return None
    # SCV_STRING: bytes en .str
    if hasattr(val, "str") and val.str is not None:
        try:
            return val.str.decode("utf-8")
        except Exception:
            return None
    # SCV_SYMBOL: bytes en .sym
    if hasattr(val, "sym") and val.sym is not None:
        try:
            return val.sym.decode("utf-8")
        except Exception:
            return None
    return None

def make_host_fn(function_name: str, args_scval: list[Xdr.SCVal]) -> Xdr.HostFunction:
    """HostFunction INVOKE_CONTRACT (get_message / set_message)."""
    contract_id_bytes = StrKey.decode_contract(CONTRACT_ID)
    sc_addr = Xdr.SCAddress(
        type=Xdr.SCAddressType.SC_ADDRESS_TYPE_CONTRACT,
        contract_id=Xdr.Hash(contract_id_bytes),
    )
    invoke_args = Xdr.InvokeContractArgs(
        contract_address=sc_addr,
        function_name=Xdr.SCSymbol(function_name.encode("utf-8")),
        args=args_scval,
    )
    return Xdr.HostFunction(
        type=Xdr.HostFunctionType.HOST_FUNCTION_TYPE_INVOKE_CONTRACT,
        invoke_contract=invoke_args,
    )

# ---------- Demo ops ----------
# (Pega esta función en tu script en lugar de la otra)

def get_message():
    """
    LECTURA (gratis): simula la llamada a la función 'get_message' del contrato.
    Este método es más robusto porque usa la API pública del contrato.
    """
    print("🔍 get_message() (llamando a la función del contrato)")
    account = server.load_account(user.public_key)

    # Construimos la operación para llamar a la función get_message del contrato
    host_fn = make_host_fn("get_message", [])
    op = InvokeHostFunction(host_function=host_fn)

    tx = (
        TransactionBuilder(account, NETWORK_PASSPHRASE, BASE_FEE)
        .append_operation(op)
        .set_timeout(30)
        .build()
    )

    # Simulamos la transacción para obtener el resultado sin gastar comisión
    sim = server.simulate_transaction(tx)
    
    # Extraemos el valor de retorno de la simulación
    retval = extract_sim_retval(sim)
    
    # Convertimos el valor de retorno (SCVal) a un tipo de Python (string)
    message = scval.to_python(retval) if retval is not None else None
    print("📨 Mensaje actual:", message)
    return message

# Necesitarás también la función extract_sim_retval original:
def extract_sim_retval(sim):
    if hasattr(sim, "results") and sim.results:
        item = sim.results[0]
        for attr in ("retval", "retVal", "return_value"):
            if hasattr(item, attr):
                return getattr(item, attr)
    for attr in ("result", "retval", "retVal", "return_value"):
        if hasattr(sim, attr):
            obj = getattr(sim, attr)
            if hasattr(obj, "retval"):
                return getattr(obj, "retval")
            return obj
    return None

    prepared = server.prepare_transaction(tx)     # esto ayuda a footprints de lectura
    sim = server.simulate_transaction(prepared)

    retval = _extract_retval_from_sim(sim)
    message = _scval_to_string(retval)
    print("📨 Mensaje actual:", message)
    return message

def set_message(new_message: str):
    """ESCRITURA: preparar → firmar → enviar → esperar confirmación."""
    print(f'✏️ set_message("{new_message}")')
    account = server.load_account(user.public_key)

    arg = scval.to_string(new_message)   # Python str -> SCVal (helper del SDK)
    host_fn = make_host_fn("set_message", [arg])
    op = InvokeHostFunction(host_function=host_fn)

    tx = (
        TransactionBuilder(account, NETWORK_PASSPHRASE, BASE_FEE)
        .append_operation(op)
        .set_timeout(60)
        .build()
    )

    prepared = server.prepare_transaction(tx)
    prepared.sign(user)

    sent = server.send_transaction(prepared)

    # 13.1.0 reporta errores en error_result_xdr
    if getattr(sent, "error_result_xdr", None):
        raise RuntimeError(f"Rechazada al enviar (XDR): {sent.error_result_xdr}")

    wait_tx(sent.hash)
    print("✅ Confirmada. Hash:", sent.hash)

# ---------- Run ----------
if __name__ == "__main__":
    get_message()                         # leer antes
    set_message("Hola desde Python ✅")    # escribir
    get_message()                         # leer después
