use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self};
use std::process::Command;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value = "ts")]
    lang: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
}

fn main() {
    check_dependency("npm");
    check_dependency("npx");

    let cli = Cli::parse();

    if let Some(Commands::Init) = cli.command {
        let project_name = cli.name;
        let lang = cli.lang;

        if lang != "js" && lang != "ts" {
            eprintln!("Invalid language: {}. Supported values are 'js' or 'ts'.", lang);
            std::process::exit(1);
        }

        println!("Creating a new frontend project: {}", project_name);

        let bar = ProgressBar::new(3);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        bar.set_message("Setting up frontend...");
        if let Err(e) = setup_frontend(&project_name, &lang) {
            eprintln!("Failed to setup frontend: {}", e);
            std::process::exit(1);
        }
        bar.inc(1);

        bar.set_message("Installing TailwindCSS...");
        if let Err(e) = setup_tailwind(&project_name) {
            eprintln!("Failed to setup Tailwind: {}", e);
            std::process::exit(1);
        }
        bar.inc(1);

        bar.set_message("Installing shadcn-ui...");
        if let Err(e) = install_shadcn_ui(&project_name) {
            eprintln!("Failed to install shadcn-ui: {}", e);
            std::process::exit(1);
        }
        bar.inc(1);

        bar.finish_with_message("Project setup complete!");
        println!("Navigate to '{}' to start building your project.", project_name);
    }
}

fn setup_frontend(project_name: &str, lang: &str) -> Result<(), String> {
    let lang_flag = if lang == "js" { "--template react" } else { "--template react-ts" };
    let cmd = format!("npm create vite@latest {} -- {}", project_name, lang_flag);
    run_command(&cmd)
}

fn setup_tailwind(project_name: &str) -> Result<(), String> {
    // Install dependencies first
    run_command(&format!(
        "cd {} && npm install -D tailwindcss postcss autoprefixer",
        project_name
    ))?;
    
    // Initialize Tailwind separately
    run_command(&format!("cd {} && npx tailwindcss init -p", project_name))?;

    let config_path = format!("{}/tailwind.config.js", project_name);
    let css_path = format!("{}/src/index.css", project_name);

    fs::create_dir_all(Path::new(&config_path).parent().unwrap())
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(Path::new(&css_path).parent().unwrap())
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    fs::write(
        config_path,
        r#"/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}"#,
    )
    .map_err(|e| format!("Failed to write tailwind config: {}", e))?;

    fs::write(
        css_path,
        "@tailwind base;\n@tailwind components;\n@tailwind utilities;\n",
    )
    .map_err(|e| format!("Failed to write CSS file: {}", e))?;

    Ok(())
}

fn install_shadcn_ui(project_name: &str) -> Result<(), String> {
    run_command(&format!("cd {} && npx shadcn-ui@latest init", project_name))
}

fn run_command(cmd: &str) -> Result<(), String> {
    println!("Running command: {}", cmd);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
    }.map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed: {}\nError: {}", cmd, error))
    } else {
        Ok(())
    }
}

fn check_dependency(tool: &str) {
    let status = if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(tool)
            .status()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", tool))
            .status()
    }.expect("Failed to check dependencies");

    if !status.success() {
        eprintln!("Required tool '{}' is not installed. Please install it first.", tool);
        std::process::exit(1);
    }
}