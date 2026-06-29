use clap::{Parser, Subcommand, ValueEnum};

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
        #[arg(value_enum)]
        kind: Kind,

        name: String,

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
}

#[derive(Clone, ValueEnum)]
pub enum Kind {
    #[value(alias = "c")]
    Component,

    #[value(alias = "h")]
    Hook,
}