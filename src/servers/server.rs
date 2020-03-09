use crate::opt::Opt;
use super::http_handler::start_http_server;

pub async fn start_servers(opt: Opt) -> std::io::Result<()> {
    start_http_server(opt.http_port).await
}
