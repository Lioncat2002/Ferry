mod cli;
mod commands;

use clap::Parser;

use cli::{Cli, Command as FerryCommand};
use commands::{run, install_deps, new_project, doc_gen};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        FerryCommand::Run => run::run_program(),
        FerryCommand::Fetch => install_deps::install_deps(),
        FerryCommand::New { project_name } =>new_project::new_project(project_name),
        FerryCommand::Doc { path } => doc_gen::generate_docs(path),
    }
}
