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
    ///Run the python program
    Run,
    /// Create new Ferry project
    New {
        /// Project name
        #[clap(value_name = "Name")]
        project_name: String,
    },
    /// Generate documentation of a particular file
    Doc{
        /// Path to the file
        #[clap(value_name="Path")]
        path:String,
    }
}
