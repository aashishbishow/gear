use clap::Subcommand;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Fabricate {
        #[command(subcommand)]
        subcommand:Option<FabricateSubCommands>,
    },
    Construct {
        #[command(subcommand)]
        subcommand:Option<ConstructSubCommands>,
    },
    Assemble {
        #[command(subcommand)]
        subcommand:Option<AssembleSubCommands>,
    },
    Ignite {
        #[command(subcommand)]
        subcommand:Option<IgniteSubCommands>,
    },
    Blueprint {
        #[command(subcommand)]
        subcommand:Option<BlueprintSubCommands>,
    },
}

#[derive(Subcommand, Debug)]
pub enum AssembleSubCommands {
    ReactVite {
        #[arg(short = 'n', long)]
        name: String,

        #[arg(short = 'l', long, default_value = "js")]
        lang: String,
    },
    TailwindCSS {
        #[arg(short = 'n', long)]
        name: String,

        #[arg(short = 'l', long, default_value = "js")]
        lang: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum FabricateSubCommands {
    Fabricate {
        #[arg(short = 'n', long)]
        name: String,
    
        #[arg(short = 'l', long, default_value = "js")]
        lang: String,
    
        #[arg(short = 'f', long, default_value = "false")]
        flag: bool,
    }
}

#[derive(Subcommand, Debug)]
pub enum ConstructSubCommands {
    Construct {
        #[arg(short = 'n', long)]
        name: String,
    }
}

#[derive(Subcommand, Debug)]
pub enum IgniteSubCommands {
    Ignite {
        #[arg(short = 'n', long)]
        name: String,
    }
}

#[derive(Subcommand, Debug)]
pub enum BlueprintSubCommands {
    Blueprint {
        #[arg(short = 'n', long)]
        name: String,
    }
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}