mod cli;
mod utils;
mod commands;


use cli::{Commands, parse_cli};
use crate::commands::assemble::{ignite_development, setup_react_vite, setup_tailwindcss};
use crate::commands::fabricate::setup_fabricate;
use utils::{dependencies::check_dependencies, progress::create_progress_bar};

fn main() {

    check_dependencies(&["npm", "npx"]);

    let cli = parse_cli();

    if let Some(command) = cli.command {
        let bar = create_progress_bar();

        match command {
            // Fabricate
            Commands::Fabricate { subcommand } => {
                if let Some(sub) = subcommand {
                    match sub {
                        cli::FabricateSubCommands::Fabricate { name, lang, flag } => {
                            setup_fabricate(&name, &lang, &bar, flag);
                        }
                    }
                } else {
                    eprintln!("Error: No subcommand provided for 'fabricate'.");
                }
            }

            // Assemble
            Commands::Assemble { subcommand } => {
                if let Some(sub) = subcommand {
                    match sub {
                        cli::AssembleSubCommands::ReactVite { name, lang } => {
                            setup_react_vite(&name, &lang, &bar);
                        }
                        cli::AssembleSubCommands::TailwindCSS { name, lang } => {
                            setup_tailwindcss(&name, &lang, &bar);
                        }
                    }
                } else {
                    eprintln!("Error: No subcommand provided for 'assemble'. Use 'react-vite' or 'tailwindcss'.");
                }
            }

            // Ignite
            Commands::Ignite { subcommand } => {
                if let Some(sub) = subcommand {
                    match sub {
                        cli::IgniteSubCommands::Ignite { name } => {
                            ignite_development(&name);
                        }
                    }
                } else {
                    eprintln!("Error: No subcommand provided for 'ignite'.");
                }
            }

            // Construct
            Commands::Construct { subcommand } => {
                if let Some(sub) = subcommand {
                    match sub {
                        cli::ConstructSubCommands::Construct { name } => {
                            println!("Executing Construct command for: {}", name);
                        }
                    }
                } else {
                    eprintln!("Error: No subcommand provided for 'construct'.");
                }
            }

            // Blueprint
            Commands::Blueprint { subcommand } => {
                if let Some(sub) = subcommand {
                    match sub {
                        cli::BlueprintSubCommands::Blueprint { name } => {
                            println!("Executing Blueprint command for: {}", name);
                        }
                    }
                } else {
                    eprintln!("Error: No subcommand provided for 'blueprint'.");
                }
            }
        }
}

}
