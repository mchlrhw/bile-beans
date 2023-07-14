use axum::{
    routing::{get, post},
    Router,
};

async fn mine() {
    todo!()
}

async fn new_transaction() {
    todo!()
}

async fn full_chain() {
    todo!()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/mine", get(mine))
        .route("/transaction/new", post(new_transaction))
        .route("/chain", get(full_chain));

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
