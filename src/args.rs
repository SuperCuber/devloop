use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Devloop")]
pub(crate) struct Opt {
    #[structopt(default_value = "Devloop.toml")]
    pub(crate) config: PathBuf,
}

pub(crate) fn get_opts() -> Opt {
    Opt::from_args()
}
