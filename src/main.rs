use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use bile_beans::{proof_of_work, Block, Blockchain, Transaction};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

struct Node {
    blockchain: Blockchain,
    id: String,
    neighbours: HashSet<SocketAddr>,
}

impl Node {
    fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string().replace('-', ""),
            blockchain: Default::default(),
            neighbours: Default::default(),
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

#[derive(Serialize, Deserialize)]
struct FullChain {
    chain: Vec<Block>,
    length: usize,
}

async fn full_chain(State(node): State<NodeState>) -> Json<FullChain> {
    let chain = node.lock().unwrap().blockchain.chain();
    let length = chain.len();

    Json(FullChain { chain, length })
}

#[derive(Deserialize)]
struct Nodes {
    nodes: Vec<SocketAddr>,
}

#[derive(Serialize)]
struct TotalNodes {
    total_nodes: HashSet<SocketAddr>,
}

async fn register_node(
    State(node): State<NodeState>,
    Json(nodes): Json<Nodes>,
) -> (StatusCode, Json<TotalNodes>) {
    let mut node = node.lock().unwrap();
    node.neighbours.extend(&nodes.nodes);

    let resp_body = Json(TotalNodes {
        total_nodes: node.neighbours.clone(),
    });

    (StatusCode::CREATED, resp_body)
}

async fn consensus(State(node): State<NodeState>) -> Json<FullChain> {
    let mut chains = vec![];
    let neighbours = node.lock().unwrap().neighbours.clone();
    for neighbour in neighbours {
        let FullChain { chain, .. }: FullChain = reqwest::get(format!("http://{neighbour}/chain"))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        chains.push(chain);
    }

    let chain = node.lock().unwrap().blockchain.resolve_conflicts(&chains);
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
        .route("/node/register", post(register_node))
        .route("/node/resolve", get(consensus))
        .with_state(node);

    let port = std::env::args().nth(1).unwrap();
    let addr = format!("127.0.0.1:{port}");

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
