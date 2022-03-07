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
    Run,
    /// Create new Ferry project
    New {
        /// Project name
        #[clap(value_name = "NAME")]
        project_name: String,
    },
    /// Generate documentation of a particular file
    Doc{
        #[clap(value_name="Path to file")]
        path:String,
    }
}
