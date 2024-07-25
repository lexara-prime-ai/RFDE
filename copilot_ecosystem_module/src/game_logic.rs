use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    version = "1.0",
    author = "Irfan M. Ghat",
    about = "A Marco Polo game."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[clap(version = "1.0", author = "Irfan M. Ghat")]
    Play { name: String },
}
