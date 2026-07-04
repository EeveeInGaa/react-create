use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rea")]
#[command(version)]
#[command(about = "Generate React files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(alias = "g")]
    Gen {
        #[command(subcommand)]
        generator: Generator,
    },
}

#[derive(Args)]
pub struct GeneratorArgs {
    pub name: String,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Generator {
    #[command(alias = "c")]
    Component {
        #[command(flatten)]
        common: GeneratorArgs,

        #[arg(long)]
        css: bool,

        #[arg(long)]
        props: bool,

        #[arg(long)]
        docs: bool,

        #[arg(long)]
        test: bool,

        #[arg(long)]
        story: bool,
    },

    #[command(alias = "h")]
    Hook {
        #[command(flatten)]
        common: GeneratorArgs,
    },

    #[command(alias = "i")]
    Interface {
        #[command(flatten)]
        common: GeneratorArgs,
    },

    #[command(alias = "e")]
    Enum {
        #[command(flatten)]
        common: GeneratorArgs,
    },

    #[command(alias = "t")]
    Type {
        #[command(flatten)]
        common: GeneratorArgs,
    },

    #[command(alias = "fn")]
    Function {
        #[command(flatten)]
        common: GeneratorArgs,
    },
}