mod main2;

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

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
    },
    Ignite {
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
            Commands::Construct { name } => setup_fullstack(&name, &bar),
            Commands::Assemble { name } => setup_express(&name, &bar),
            Commands::Ignite { name } => ignite_development(&name),
        }
    }
}

fn check_dependencies(dependencies: &[&str]) {
    for dep in dependencies {
        if Command::new(dep).arg("--version").output().is_err() {
            eprintln!("Missing dependency: {}. Please install it.", dep);
            std::process::exit(1);
        }
    }
}

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

fn setup_fabricate(name: &str, lang: &str, bar: &ProgressBar) {
    if lang != "js" && lang != "ts" {
        eprintln!("Invalid language: {}. Supported values are 'js' or 'ts'.", lang);
        std::process::exit(1);
    }

    bar.set_message("Setting up React with Vite...");
    execute_or_exit(setup_frontend(name, lang), "Failed to setup React with Vite.");
    bar.inc(1);

    bar.set_message("Installing dependencies...");
    execute_or_exit(install_dependencies(name), "Failed to install dependencies.");
    bar.inc(1);

    bar.set_message("Installing TailwindCSS...");
    execute_or_exit(setup_tailwind(name, lang), "Failed to setup Tailwind.");
    bar.inc(1);

    bar.finish_with_message("React-Vite project setup complete!");
    println!("Navigate to '{}' to start building your project.", name);
}

fn setup_fullstack(name: &str, bar: &ProgressBar) {
    bar.set_message("Setting up Next.js with Express...");
    execute_or_exit(setup_nextjs(name), "Failed to setup Next.js.");
    bar.inc(1);

    execute_or_exit(setup_express(name), "Failed to setup Express.js during Next.js setup.");
    bar.inc(1);

    bar.finish_with_message("Next.js and Express.js setup complete!");
    println!("Your full-stack project is ready to go!");
}

fn setup_express(name: &str, bar: &ProgressBar) {
    bar.set_message("Setting up Express.js...");
    execute_or_exit(setup_express(name), "Failed to setup Express.js.");
    bar.inc(1);
    bar.finish_with_message("Express.js setup complete!");
    println!("Your Express.js project is ready to go!");
}

fn setup_frontend(project_name: &str, lang: &str) -> Result<(), String> {
    let lang_flag = if lang == "js" { "--template react-swc" } else { "--template react-swc-ts" };
    run_command(&format!("npm create vite@latest {} -- {}", project_name, lang_flag))
}

fn install_dependencies(project_name: &str) -> Result<(), String> {
    run_command(&format!("cd {} && npm install", project_name))
}

fn setup_tailwind(project_name: &str, lang: &str) -> Result<(), String> {
    run_command(&format!("cd {} && npm install tailwindcss @tailwindcss/vite", project_name))?;

    let config_extension = if lang == "ts" { "ts" } else { "js" };
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
    )?;

    fs::write(
        css_path,
        "@import 'tailwindcss';\nbody { font-family: Inter, system-ui; }",
    )?;

    Ok(())
}

fn setup_nextjs(project_name: &str) -> Result<(), String> {
    run_command(&format!("npx create-next-app@latest {} --ts", project_name))
}

fn setup_express(project_name: &str) -> Result<(), String> {
    run_command(&format!("mkdir {} && cd {} && npm init -y && npm install express", project_name, project_name))
}

fn ignite_development(project_name: &str) {
    println!("Starting the development server...");
    execute_or_exit(
        run_command(&format!("cd {} && npm run dev", project_name)),
        "Failed to start the development server.",
    );
}

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

fn execute_or_exit(result: Result<(), String>, error_message: &str) {
    if let Err(e) = result {
        eprintln!("{}: {}", error_message, e);
        std::process::exit(1);
    }
}
