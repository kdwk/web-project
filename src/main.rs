use axum::{Router, response::IntoResponse, routing::get};

// #[tokio::main]
#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/", get(hello_world).into());
    Ok(app.into())
    // let addr = SocketAddr::new([0,0,0,0].into(), 3000);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

async fn hello_world() -> impl IntoResponse { // allow return of anything that can be turned into a response (anything that implements IntoResponse trait)
    "Hello World"
}

async fn test() -> impl IntoResponse {

}