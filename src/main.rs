mod cli;
mod fs_utils;
mod generators;

use std::io;

use clap::Parser;

use cli::{Cli, Command, Kind};
use generators::{generate_component, generate_hook};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Gen { kind, name, css, props, docs, test, story } => match kind {
            Kind::Component => generate_component(&name, css, props, docs, test, story)?,
            Kind::Hook => generate_hook(&name)?,
        },
    }

    Ok(())
}
