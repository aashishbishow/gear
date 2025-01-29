use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Barrier;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

fn main() {
    check_dependencies(&["npm", "npx"]);

    let cli = Cli::parse();

    if let Some(command) = cli.command {
        let bar = create_progress_bar();

        match command {
            Commands::Fabricate { name, lang } => setup_fabricate(&name, &lang, &bar),
            Commands::Construct { name } => setup_fullstack(&name, &lang, &bar),
            Commands::Assemble { name , lang} => setup_react_vite(&name, &lang, &bar ),
            Commands::Assemble { name , lang} => setup_tailwindcss(&name, &lang, &bar),
            Commands::Assemble { name } => setup_shadcnui(&name, &bar),
            Commands::Assemble { name } => setup_nextjs(&name, &lang, &bar),
            Commands::Assemble { name } => setup_expressjs(&name, &lang, &bar),
            Commands::Ignite { name } => ignite_development(&name),
            Commands::Blueprint { name } => setup_blueprint(&name, &bar),
        }
    }
}

// Function to check for dependencies
fn check_dependencies(dependencies: &[&str]) {
    for dep in dependencies {
        if Command::new(dep).arg("--version").output().is_err() {
            eprintln!("Missing dependency: {}. Please install it.", dep);
            std::process::exit(1);
        }
    }
}

// Function to create a progress bar
fn create_progress_bar() -> ProgressBar {
    let bar = ProgressBar::new(4);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
            .unwrap()
            .progress_chars("\u{2699} >-"),
    );
    bar
}

// Function to run a command
fn run_command(command: &str) -> Result<(), String> {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    };

    match status {
        Ok(exit_status) if exit_status.success() => Ok(()),
        Ok(exit_status) => Err(format!("Command exited with status: {}", exit_status)),
        Err(err) => Err(format!("Failed to execute command: {}", err)),
    }
}

// Function to execute a command or exit the process if it fails
fn execute_or_exit(result: Result<(), String>, error_message: &str) {
    if let Err(e) = result {
        eprintln!("{}: {}", error_message, e);
        std::process::exit(1);
    }
}

// Function to ignite the development server
fn ignite_development(project_name: &str) {
    println!("Starting the development server...");
    execute_or_exit(
        run_command(&format!("cd {} && npm run dev", project_name)),
        "Failed to start the development server.",
    );
}

// Function to setup a react project with Vite
fn setup_react_vite(project_name: &str, lang: &str, bar: &ProgressBar) -> Result<(), String> {
    bar.set_message("Creating a new React project with Vite...");

    // Determine the language flag based on the selected language
    let lang_flag = match lang {
        "ts" => "--template react-ts",
        _ => "--template react",
    };

    //Construct the command to create a new React project with Vite
    let command = format!("npx create-vite@latest {} {}", project_name, lang_flag);

    // Execute the command and exit the process if it fails
    execute_or_exit(
        run_command(&command),
        "Failed to create a new React project with Vite.",
    );
    bar.inc(1);
    Ok(())
}

// Function to setup Tailwind CSS
fn setup_tailwindcss(project_name: &str, lang:&str, bar: &ProgressBar) -> Result<(), String> {
    bar.set_message("Setting up Tailwind CSS...");
    
    let command = format!("cd {} && npm install tailwindcss @tailwindcss/vite", project_name);

    let config_extension = match lang {
        "ts" => "ts",
        _ => "js",
    };

    let config_path = format!("{}/vite.config.{}", project_name, config_extension);
    let css_path = format!("{}/src/index.css", project_name);
    
    fs::write(
        config_path,
        r#"import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
    plugins: [react(), tailwindcss()],
});"#,
    )
    .map_err(|e| format!("Failed to write tailwind config: {}", e))?;

    fs::write(
        css_path,
        r#"@import "tailwindcss";
        :root {
            font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
            line-height: 1.5;
            font-weight: 400;
          
            color-scheme: light dark;
            color: rgba(255, 255, 255, 0.87);
            background-color: #242424;
          
            font-synthesis: none;
            text-rendering: optimizeLegibility;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
          }
          
          a {
            font-weight: 500;
            color: #646cff;
            text-decoration: inherit;
          }
          a:hover {
            color: #535bf2;
          }
          
          body {
            margin: 0;
            display: flex;
            place-items: center;
            min-width: 320px;
            min-height: 100vh;
          }
          
          h1 {
            font-size: 3.2em;
            line-height: 1.1;
          }
          
          button {
            border-radius: 8px;
            border: 1px solid transparent;
            padding: 0.6em 1.2em;
            font-size: 1em;
            font-weight: 500;
            font-family: inherit;
            background-color: #1a1a1a;
            cursor: pointer;
            transition: border-color 0.25s;
          }
          button:hover {
            border-color: #646cff;
          }
          button:focus,
          button:focus-visible {
            outline: 4px auto -webkit-focus-ring-color;
          }
          
          @media (prefers-color-scheme: light) {
            :root {
              color: #213547;
              background-color: #ffffff;
            }
            a:hover {
              color: #747bff;
            }
            button {
              background-color: #f9f9f9;
            }
          }"#,
    )
    .map_err(|e| format!("Failed to write CSS file: {}", e))?;

    // Execute the command and exit the process if it fails
    execute_or_exit(
        run_command(&command),
        "Failed to add tailwindcss.",
    );

    bar.inc(1);
    bar.finish_with_message("Tailwind CSS setup complete!");
    Ok(())
}