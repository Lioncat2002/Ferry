mod cli;
mod executor;

use clap::Parser;

use cli::{Cli, Command as FerryCommand};
use executor::{generate_docs, install_deps, new_project, run_program};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        FerryCommand::Run => run_program(),
        FerryCommand::Fetch => install_deps(),
        FerryCommand::New { project_name } => new_project(project_name),
        FerryCommand::Doc { path } => generate_docs(path),
    }
}
