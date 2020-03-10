use super::http_handler::start_http_server;
use crate::Opt;

pub async fn start_servers(opt: Opt) -> std::io::Result<()> {
    start_http_server(opt.http_port).await
}
