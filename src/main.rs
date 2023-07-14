use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use bile_beans::{Blockchain, Transaction};
use std::sync::{Arc, Mutex};

struct Node {
    blockchain: Blockchain,
    _id: String,
}

impl Node {
    fn new() -> Self {
        Self {
            blockchain: Default::default(),
            _id: uuid::Uuid::new_v4().to_string().replace('-', ""),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

type NodeState = Arc<Mutex<Node>>;

async fn mine() {
    todo!()
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

async fn full_chain() {
    todo!()
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
