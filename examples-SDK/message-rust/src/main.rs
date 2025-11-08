use soroban_client::account::AccountBehavior;
use soroban_client::contract;
use soroban_client::contract::ContractBehavior;
use soroban_client::keypair::KeypairBehavior;
use soroban_client::network::{NetworkPassphrase, Networks};
use soroban_client::soroban_rpc::{SendTransactionStatus, TransactionStatus};
use soroban_client::transaction::Account;
use soroban_client::transaction::TransactionBehavior;
use soroban_client::transaction_builder::TransactionBuilder;
use soroban_client::transaction_builder::TransactionBuilderBehavior;
use soroban_client::transaction_builder::TIMEOUT_INFINITE;
use soroban_client::EventFilter;
use soroban_client::Options;
use soroban_client::{keypair::Keypair, Server};
use stellar_xdr::{ScVal, String as XdrString, Limits, WriteXdr};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

// ========= CONFIGURA TUS DATOS =========
const CONTRACT_ID: &str = "CAJN25XAZLTZEVS7ZFLNZ3HWREJRQHKUU265CK67ED2ASJ22TDQ5Y4PL";
// =======================================

/// Lee el mensaje del contrato simulando la llamada a `get_message`.
async fn read_message(
    server: &Server,
    source_account: &Rc<RefCell<Account>>,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n🔍 read_message() (desde Rust con soroban-client)");

    let contract = contract::Contracts::new(CONTRACT_ID)?;

    // 1. Construir la transacción para llamar a la función
    let read_tx = TransactionBuilder::new(source_account.clone(), Networks::testnet(), None)
        .fee(1000000_u32)
        .add_operation(contract.call("get_message", None)) // get_message no tiene argumentos
        .set_timeout(TIMEOUT_INFINITE)?
        .build();

    // 2. Simular la transacción para obtener el resultado sin enviarla
    let sim_result = server.simulate_transaction(read_tx).await?;

    // 3. Extraer el valor de retorno del resultado de la simulación
    if sim_result.results.is_empty() {
        return Err("La simulación no devolvió resultados".into());
    }
    let retval = &sim_result.results[0].retval;

    // 4. Decodificar el ScVal a un tipo de Rust (String)
    // El ScVal para un string tiene la forma ScVal::String(XdrString)
    if let ScVal::String(xdr_string) = retval {
        let message = String::from_utf8(xdr_string.to_vec())?;
        println!("📨 Mensaje actual: {}", message);
        Ok(message)
    } else {
        Err("El valor de retorno no era un string".into())
    }
}

/// Escribe un nuevo mensaje en el contrato llamando a `set_message`.
async fn write_message(
    server: &Server,
    source_account: &Rc<RefCell<Account>>,
    keypair: &Keypair,
    new_message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!('\n✏️ write_message("{}") (desde Rust con soroban-client)', new_message);

    let contract = contract::Contracts::new(CONTRACT_ID)?;

    // 1. Crear el argumento ScVal para el nuevo mensaje
    let xdr_string = XdrString::try_from(new_message)?;
    let message_arg = ScVal::String(xdr_string);

    // 2. Construir la transacción para llamar a la función con el argumento
    let mut write_tx =
        TransactionBuilder::new(source_account.clone(), Networks::testnet(), None)
            .fee(1000000_u32)
            .add_operation(contract.call("set_message", Some(vec![message_arg]))) // Pasamos el argumento
            .set_timeout(TIMEOUT_INFINITE)?
            .build();

    // 3. Preparar la transacción (añade footprint, etc.)
    write_tx = server.prepare_transaction(write_tx).await?;

    // 4. Firmar la transacción
    write_tx.sign(&[keypair.clone()]);

    // 5. Enviar la transacción y esperar la confirmación (reutilizamos tu lógica de polling)
    match server.send_transaction(write_tx).await {
        Ok(response) => {
            println!("✅ Transacción enviada con éxito");
            println!("Hash de la transacción: {}", response.hash);

            let hash = response.hash.clone();
            if response.status == SendTransactionStatus::Error {
                dbg!(&response);
                return Err("Error en el envío de la transacción".into());
            }

            loop {
                let response = server.get_transaction(&hash).await;
                if let Ok(tx_result) = response {
                    match tx_result.status {
                        TransactionStatus::Success => {
                            println!("✅ ¡Transacción confirmada con éxito!");
                            break;
                        }
                        TransactionStatus::NotFound => {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                        TransactionStatus::Failed => {
                            eprintln!("❌ La transacción falló.");
                            if let Some(result) = tx_result.to_result() {
                                eprintln!("Motivo: {:?}", result);
                            }
                            return Err("La transacción falló en la red".into());
                        }
                    }
                } else {
                    eprintln!("Error al obtener el estado de la transacción: {:?}", response);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Fallo al enviar la transacción: {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Iniciando Demo del Contrato de Mensajes (soroban-client v-next) ---");

    // Inicializar la conexión al servidor
    let server = Server::new(
        "https://soroban-testnet.stellar.org",
        Options {
            timeout: 1000,
            ..Default::default()
        },
    )?;

    // Configurar la cuenta de origen
    let source_keypair = Keypair::random()?;
    let source_public_key = &source_keypair.public_key();

    // Obtener información de la cuenta del servidor (con airdrop)
    let account_data = server.request_airdrop(source_public_key).await?;
    let source_account = Rc::new(RefCell::new(
        Account::new(source_public_key, &account_data.sequence_number())?,
    ));

    // 1. Leer el mensaje inicial
    read_message(&server, &source_account).await?;

    // 2. Escribir un nuevo mensaje
    write_message(&server, &source_account, &source_keypair, "Hola desde Rust 🦀").await?;

    // 3. Leer el mensaje de nuevo para verificar el cambio
    read_message(&server, &source_account).await?;

    println!("\n--- Demo Finalizada ---");
    Ok(())
}