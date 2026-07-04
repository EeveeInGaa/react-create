mod cli;
mod fs_utils;
mod generators;

use std::io;

use clap::Parser;

use cli::{Cli, Command, Kind};
use generators::{
    generate_component,
    generate_hook,
    generate_interface,
    generate_enum,
    generate_type,
    generate_function
};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Gen {
            kind,
            name,
            css,
            props,
            docs,
            test,
            story,
            path
        } => {
            let base_path = path.unwrap_or_else(|| match kind {
                Kind::Component => "src/components".into(),
                Kind::Hook => "src/hooks".into(),
                Kind::Interface => "src/utils/interfaces".into(),
                Kind::Enum => "src/utils/enums".into(),
                Kind::Type => "src/utils/types".into(),
                Kind::Function => "src/utils/functions".into(),
            });

            match kind {
                Kind::Component => {
                    generate_component(&name, &base_path, css, props, docs, test, story)?
                }
                Kind::Hook => generate_hook(&name)?,
                Kind::Interface => generate_interface(&name, &base_path)?,
                Kind::Enum => generate_enum(&name, &base_path)?,
                Kind::Type => generate_type(&name, &base_path)?,
                Kind::Function => generate_function(&name, &base_path)?,
            }
        }
    }

    Ok(())
}
