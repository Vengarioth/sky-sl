use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(about = "Parse a single file")]
    Parse {
        #[structopt(name = "FILE")]
        file: PathBuf,

        #[structopt(name = "OUTPUT")]
        output: PathBuf,
    },
}

impl Command {
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}
