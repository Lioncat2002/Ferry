use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Install project dependencies
    Fetch,

    /// Create new Ferry project
    New {
        /// Project name
        #[clap(value_name = "NAME")]
        project_name: String,
    },
}
