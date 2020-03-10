
use structopt::StructOpt;
use vvvdb::{server, Opt};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to vvvdb!");
    let opt = Opt::from_args();
    println!("http_port is {:#?}", opt.http_port);
    server::start_servers(opt).await
}
