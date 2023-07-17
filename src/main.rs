use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use bile_beans::{proof_of_work, Block, Blockchain, Transaction};
use serde::Serialize;
use std::sync::{Arc, Mutex};

struct Node {
    blockchain: Blockchain,
    id: String,
}

impl Node {
    fn new() -> Self {
        Self {
            blockchain: Default::default(),
            id: uuid::Uuid::new_v4().to_string().replace('-', ""),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

type NodeState = Arc<Mutex<Node>>;

async fn mine(State(node): State<NodeState>) -> Json<Block> {
    let mut node = node.lock().unwrap();
    let last_proof = node.blockchain.last_proof();
    let proof = proof_of_work(last_proof);

    let transaction = Transaction::new("0".to_string(), node.id.clone(), 1);
    node.blockchain.new_transaction(transaction);

    Json(node.blockchain.new_block(proof))
}

async fn new_transaction(
    State(node): State<NodeState>,
    Json(transaction): Json<Transaction>,
) -> (StatusCode, String) {
    let index = node.lock().unwrap().blockchain.new_transaction(transaction);

    (
        StatusCode::CREATED,
        format!("Transaction will be added to Block {index}"),
    )
}

#[derive(Serialize)]
struct FullChain {
    chain: Vec<Block>,
    length: usize,
}

async fn full_chain(State(node): State<NodeState>) -> Json<FullChain> {
    let chain = node.lock().unwrap().blockchain.chain();
    let length = chain.len();

    Json(FullChain { chain, length })
}

#[tokio::main]
async fn main() {
    let node: NodeState = Default::default();

    let app = Router::new()
        .route("/mine", get(mine))
        .route("/transaction/new", post(new_transaction))
        .route("/chain", get(full_chain))
        .with_state(node);

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
