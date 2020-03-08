mod err;
mod opt;

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use structopt::StructOpt;
use serde::Deserialize;
use self::opt::Opt;

#[derive(Deserialize)]
struct Query {
    query: String,
}

#[get("/")]
async fn index(data: web::Query<Query>) -> String {
    format!("Your query: {}", data.query)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to vvvdb!");
    start_http_server().await
}


async fn start_http_server() -> std::io::Result<()> {
    let opt = Opt::from_args();
    println!("http_port is {:#?}", opt.http_port);

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .workers(16)
        .bind( format!("127.0.0.1:{}", opt.http_port))?
        .run()
        .await
}
