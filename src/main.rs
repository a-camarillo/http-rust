use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};
use image::{io::Reader as ImageReader, ImageError};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(handler));

    // run app on port 3000
    let _listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
     Ok(listener) => axum::serve(listener, app).await.unwrap(),
     Err(e) => println!("Error occurred: {}", e),
    };
}

async fn handler() -> Result<Vec<u8>, String> {
    match root().await {
        Ok(i) => Ok(i),
        Err(e) => Err(e.to_string()),
    }
}

async fn root() -> Result<Vec<u8>, ImageError> {
    let bytes = ImageReader::open("../images/gritty.jpg")?;
    let decoded = bytes.decode()?.into_bytes();
    Ok(decoded)
}
