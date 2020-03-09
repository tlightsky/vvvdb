
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    query: String,
}

#[get("/")]
async fn index(data: web::Query<Query>) -> String {
    format!("Your query: {}", data.query)
}

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};

pub async fn start_http_server(http_port: String) -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .workers(16)
        .bind( format!("127.0.0.1:{}", http_port))?
        .run()
        .await
}
