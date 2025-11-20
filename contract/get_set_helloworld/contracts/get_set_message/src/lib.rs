#![no_std]

use soroban_sdk::{contract, contractimpl, contracterror, Env, Address, Symbol, Bytes, Vec, Map, symbol_short, invoke};

#[contracterror]
#[derive(Copy)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    NotInitialized = 2,
}

const ADMINS_KEY: Symbol = symbol_short!("admins");
const RECEIVER_KEY: Symbol = symbol_short!("receiver");

#[contract]
pub struct BlocksResources;

#[contractimpl]
impl BlocksResources {
    /// Inicializa el contrato con admins (multisig implícito en tx) y receptor.
    pub fn initialize(env: Env, admin1: Address, admin2: Address, receiver: Address) {
        if env.storage().instance().has(&RECEIVER_KEY) {
            panic!("Ya inicializado");
        }
        let mut admins: Vec<Address> = Vec::new(&env);
        admins.push(admin1);
        admins.push(admin2);
        env.storage().instance().set(&ADMINS_KEY, admins);
        env.storage().instance().set(&RECEIVER_KEY, receiver);
        // Evento de inicialización
        (Symbol::short("init"), admin1, admin2, receiver).emit(&env);
    }

    /// Registra un evento de trazabilidad para un recurso ambiental.
    /// Requiere auth de admin (verificado en tx multisig).
    pub fn log_trace(
        env: Env,
        resource_id: Bytes,  // ID del recurso, e.g., "RECURSO_001"
        from: Address,       // Origen
        to: Address,         // Destino
        metadata: Map<Symbol, Bytes>,  // Metadata, e.g., { "tipo": "agua", "cantidad": "100L" }
    ) {
        let admins: Vec<Address> = env.storage().instance().get(&ADMINS_KEY).unwrap_or_else(|| panic!("No inicializado"));
        let caller = env.invoker();
        if !admins.contains(&caller) {
            panic_error!(env, Error::NotAuthorized);
        }
        // Emitir evento para trazabilidad (visible en explorer)
        (
            Symbol::short("trace"),
            resource_id,
            from,
            to,
            env.block().timestamp(),
            metadata,
        ).emit(&env);
    }

    /// Simula liberación de fondos: emite evento y permite tx con pago a receptor.
    /// El pago real se hace vía extensión de tx (no en contrato).
    pub fn release_funds(env: Env, resource_id: Bytes, amount: i128) {  // amount en stroops (1 XLM = 10^7 stroops)
        let receiver: Address = env.storage().instance().get(&RECEIVER_KEY).unwrap_or_else(|| panic!("No inicializado"));
        let admins: Vec<Address> = env.storage().instance().get(&ADMINS_KEY).unwrap_or_else(|| panic!("No inicializado"));
        let caller = env.invoker();
        if !admins.contains(&caller) {
            panic_error!(env, Error::NotAuthorized);
        }
        // Emitir evento de liberación
        (
            Symbol::short("release"),
            resource_id,
            amount,
            receiver,
            env.block().timestamp(),
        ).emit(&env);
        // Nota: Para transferir fondos reales, usa --payment en soroban tx invoke.
    }
}
