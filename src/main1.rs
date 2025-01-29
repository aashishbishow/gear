use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self};
use std::process::Command;
use std::path::Path;
// use std::thread;
// use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value = "js")]
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

        let bar = ProgressBar::new(4);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                .unwrap()
                .progress_chars("\u{2699} >-"),
        );

        bar.set_message("Setting up frontend...");
        if let Err(e) = setup_frontend(&project_name, &lang) {
            eprintln!("Failed to setup frontend: {}", e);
            std::process::exit(1);
        }
        bar.inc(1);

        // Add a small delay to ensure project creation is complete
        // thread::sleep(Duration::from_secs(2));

        bar.set_message("Installing dependencies...");
        if let Err(e) = install_dependencies(&project_name) {
            eprintln!("Failed to install dependencies for project '{}' :{}",project_name, e);
            std::process::exit(1);
        }
        bar.inc(1);

        bar.set_message("Installing TailwindCSS...");
        if let Err(e) = setup_tailwind(&project_name, &lang) {
            eprintln!("Failed to setup Tailwind: {}", e);
            std::process::exit(1);
        }
        bar.inc(1);

        bar.finish_with_message("Project setup complete!");
        println!("Navigate to '{}' to start building your project.", project_name);
    }
}

fn setup_frontend(project_name: &str, lang: &str) -> Result<(), String> {
    let lang_flag = if lang == "js" { "--template react-swc" } else { "--template react-ts-swc" };
    let cmd = format!("npm create vite@latest {} -- {}", project_name, lang_flag);
    run_command(&cmd)
}

fn install_dependencies(project_name: &str) -> Result<(), String> {
    run_command(&format!("cd {} && npm install", project_name))
}

fn setup_tailwind(project_name: &str, lang: &str) -> Result<(), String> {
    // Install dependencies
    run_command(&format!(
        "cd {} && npm install tailwindcss @tailwindcss/vite",
        project_name
    ))?;
    
    // Conditionally set config path based on language
    let config_extension = if lang == "ts" { "ts" } else { "js" };
    let config_path = format!("{}/vite.config.{}", project_name, config_extension);
    let css_path = format!("{}/src/index.css", project_name);

    fs::create_dir_all(Path::new(&config_path).parent().unwrap())
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(Path::new(&css_path).parent().unwrap())
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    fs::write(
        config_path,
        r#"import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import tailwindcss from '@tailwindcss/vite';

// https://vite.dev/config/
export default defineConfig({
plugins: [react(), tailwindcss()],
});
"#,
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

    Ok(())
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