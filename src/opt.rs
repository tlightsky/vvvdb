use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(help = "HTTP port", default_value = "9000")]
    pub http_port: String,
}
