pub(crate) mod api;
pub(crate) mod block;
pub(crate) mod libs;
pub(crate) mod miner;
pub(crate) mod state;
pub(crate) mod tx;
use axum::{Router, routing::get};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mut rng = OsRng;

    // В качестве исходных данных создаём три кошелька
    let alice_signing_key: SigningKey = SigningKey::generate(&mut rng);
    let alice_address: tx::Address = alice_signing_key.verifying_key();

    let bob_signing_key: SigningKey = SigningKey::generate(&mut rng);
    let bob_address: tx::Address = bob_signing_key.verifying_key();

    let charlie_signing_key: SigningKey = SigningKey::generate(&mut rng);
    let charlie_address: tx::Address = bob_signing_key.verifying_key();

    // Выполняем несколько транзакций
    let tx1 = tx::Tx::sign(alice_address, bob_address, 100, alice_signing_key);
    let tx2 = tx::Tx::sign(bob_address, charlie_address, 15, charlie_signing_key);

    // Добавляем их в mempool
    let miner = miner::Miner::new(1_000_000);
    let mut blockchain = block::Blockchain::new();
    let mut mempool = state::Mempool::new();
    mempool.push(tx1);
    mempool.push(tx2);
    let block = miner
        .mine_block(blockchain.blocks.last().unwrap(), &mempool.transactions)
        .unwrap();
    let _ = blockchain.add_block(block);

    // println!("Blockchain state: {:?}", blockchain);

    let shared_state = Arc::new(state::AppState {
        blockchain: Mutex::new(blockchain),
        pool: Mutex::new(mempool),
    });

    // Приложение предоставляет REST API,
    // через которое клиенты и майнеры могут взаимодействовать с блокчейном.
    let app = Router::new()
        .route("/", get(api::index))
        .route("/blocks", get(api::get_blocks).post(api::post_blocks))
        .route("/txs", get(api::get_txs).post(api::post_txs))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
