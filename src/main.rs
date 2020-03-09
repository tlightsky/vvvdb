mod err;
mod opt;
mod servers;

use structopt::StructOpt;
use self::opt::Opt;
use servers::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to vvvdb!");
    let opt = Opt::from_args();
    println!("http_port is {:#?}", opt.http_port);
    server::start_servers(opt).await
}
