mod cli;
mod executor;

use clap::Parser;

use cli::{Cli, Command as FerryCommand};
use executor::{install_deps, new_project};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        FerryCommand::Fetch => install_deps(),
        FerryCommand::New { project_name } => new_project(project_name),
    }
}
