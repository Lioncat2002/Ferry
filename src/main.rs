mod cli;
mod executor;

use clap::Parser;

use cli::{Cli, Command as FerryCommand};
use executor::{install_deps, new_project, run_program, generate_docs};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        FerryCommand::Run=>run_program(),
        FerryCommand::Fetch => install_deps(),
        FerryCommand::New { project_name } => new_project(project_name),
        FerryCommand::Doc{path}=>generate_docs(path),
    }
}
