use crate::{block::Block, state::AppState, tx::Tx};
use axum::{Json, extract::State};
use serde_json::Value;
use std::sync::Arc;

pub async fn get_txs(State(state): State<Arc<AppState>>) -> Json<Value> {
    let pool = state.pool.lock().await;
    let txs = pool.get_txs();
    let serialized = serde_json::to_value(txs).unwrap();
    Json(serialized)
}

pub async fn post_txs(State(state): State<Arc<AppState>>, tx_json: Json<Tx>) -> Json<Value> {
    let tx = tx_json.0;
    let mut pool = state.pool.lock().await;
    pool.push(tx);

    let txs = pool.get_txs();
    let serialized = serde_json::to_value(txs).unwrap();
    Json(serialized)
}

pub async fn get_blocks(State(state): State<Arc<AppState>>) -> Json<Value> {
    let blockchain = &state.blockchain.lock().await;
    let blocks = blockchain.get_blocks();
    let serialized = serde_json::to_value(blocks).unwrap();
    Json(serialized)
}

pub async fn post_blocks(
    State(state): State<Arc<AppState>>,
    block_json: Json<Block>,
) -> Json<Value> {
    let block = block_json.0;
    let mut blockchain = state.blockchain.lock().await;
    let _ = blockchain.add_block(block);

    let blocks = blockchain.get_blocks();
    let serialized = serde_json::to_value(blocks).unwrap();
    Json(serialized)
}

pub async fn index() -> &'static str {
    "Blockchain is live!"
}
