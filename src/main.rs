mod cli;
mod fs_utils;
mod generators;

use std::io;

use clap::Parser;

use cli::{Cli, Command, Generator};
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
        Command::Gen { generator } => match generator {
            Generator::Component {
                common,
                css,
                props,
                docs,
                test,
                story,
            } => {
                let base_path = common.path.unwrap_or_else(|| "src/components".into());
                generate_component(&common.name, &base_path, css, props, docs, test, story)?;
            }
            Generator::Hook { common } => {
                let base_path = common.path.unwrap_or_else(|| "src/hooks".into());
                generate_hook(&common.name, &base_path)?;
            }
            Generator::Interface { common } => {
                let base_path = common
                    .path
                    .unwrap_or_else(|| "src/interfaces".into());
                generate_interface(&common.name, &base_path)?;
            }
            Generator::Enum { common } => {
                let base_path = common.path.unwrap_or_else(|| "src/enums".into());
                generate_enum(&common.name, &base_path)?;
            }
            Generator::Type { common } => {
                let base_path = common.path.unwrap_or_else(|| "src/types".into());
                generate_type(&common.name, &base_path)?;
            }
            Generator::Function { common } => {
                let base_path = common
                    .path
                    .unwrap_or_else(|| "src/functions".into());
                generate_function(&common.name, &base_path)?;
            }
        },
    }

    Ok(())
}
