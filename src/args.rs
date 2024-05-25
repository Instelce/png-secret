use clap::Parser;

use crate::Result;
use crate::commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Commands>

}
