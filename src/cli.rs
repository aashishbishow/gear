use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    Fabricate {
        #[arg(short = 'n', long)]
        name: String,

        #[arg(short = 'l', long, default_value = "js")]
        lang: String,
    },
    Construct {
        #[arg(short = 'n', long)]
        name: String,
    },
    Assemble {
        #[arg(short = 'n', long)]
        name: String,

        #[arg(short = 'l', long, default_value = "js")]
        lang: String,
    },
    Ignite {
        #[arg(short = 'n', long)]
        name: String,
    },
    Blueprint {
        #[arg(short = 'n', long)]
        name: String,
    },
}
