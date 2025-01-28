use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self};
use std::process::{Command, Stdio};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fabricate{
        #[arg(short=  'n', long)]
        name: String,

        #[arg(short = 'l', long, default_value = "js")]
        lang: String,
    },

    Construct{
        #[arg(short = 'n', long)]
        name: String,
    },

    Assemble{
        #[arg(short = 'n', long)]
        name: String,
    },

    Ignite{
        #[arg(short = 'n', long)]
        name: String,
    },
}

fn main() {
    check_dependency("npm");
    check_dependency("npx");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Fabricate { name, lang }) => {
            let project_name = name.clone();
            let lang = lang;

            if lang != "js" && lang != "ts" {
                eprintln!("Invalid language: {}. Supported values are 'js' or 'ts'.", lang);
                std::process::exit(1);
            }

            println!("Creating a new react-vite project: {}", project_name);

            let bar = ProgressBar::new(4);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                    .unwrap()
                    .progress_chars("\u{2699} >-"),
            );

            bar.set_message("Setting up React with Vite...");
            if let Err(e) = setup_frontend(&project_name, &lang) {
                eprintln!("Failed to setup React with Vite: {}", e);
                std::process::exit(1);
            }
            bar.inc(1);

            bar.set_message("Installing dependencies...");
            if let Err(e) = install_dependencies(&project_name) {
                eprintln!("Failed to install dependencies for project '{}' :{}", project_name, e);
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

        // If Next.js setup is requested, set up Next.js + Express automatically
        Some(Commands::Construct { name }) => {

            let bar = ProgressBar::new(4);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                    .unwrap()
                    .progress_chars("\u{2699} >-"),
            );

            let project_name = name.clone();
            bar.set_message("Setting up NextJs with Express...");
            if let Err(e) = setup_nextjs(&project_name) {
                eprintln!("Failed to setup NextJs: {}", e);
                std::process::exit(1);
            }
            bar.inc(1);
            
        // Express will automatically be installed during Next.js setup
            if let Err(e) = setup_expressjs(&project_name) {
                eprintln!("Failed to setup ExpressJs during NextJs setup: {}", e);
                std::process::exit(1);
            }
            bar.inc(1);
            bar.finish_with_message("NextJs and ExpressJs setup complete!");
            println!("Your full stack project is ready to go!");
    }

        // If only Express.js setup is requested
        Some(Commands::Assemble { name }) => {
            let bar = ProgressBar::new(1);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                    .unwrap()
                    .progress_chars("\u{2699} >-"),
            );

            let project_name = name.clone();
            bar.set_message("Setting up ExpressJs...");
            if let Err(e) = setup_expressjs(&project_name) {
                eprintln!("Failed to setup ExpressJs: {}", e);
                std::process::exit(1);
            }
            bar.inc(1);
            bar.finish_with_message("ExpressJs setup complete!");
            println!("Your ExpressJs project is ready to go!");
        }

        Some(Commands::Ignite { name }) => {
            let project_name = name.clone();
            if let Err(e) = ignite_frontend(&project_name) {
                eprintln!("Failed to ignite React with Vite: {}", e);
                std::process::exit(1);
            }
        }
        None => {}
    }
}



fn setup_frontend(project_name: &str, lang: &str) -> Result<(), String> {
    let lang_flag = if lang == "js" { "--template react-swc" } else { "--template react-swc-ts" };
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

fn ignite_frontend(project_name: &str) -> Result<(), String> {
    let cmd = format!("cd {} && npm run dev", project_name);

    println!("Starting the development server...");
    println!("Command: {}", cmd);

    // Spawn the command as a child process
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &cmd])
            .stdout(Stdio::inherit()) // Stream stdout
            .stderr(Stdio::inherit()) // Stream stderr
            .status() // Wait for the command to complete
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::inherit()) // Stream stdout
            .stderr(Stdio::inherit()) // Stream stderr
            .status() // Wait for the command to complete
    };

    // Check the result of the process execution
    match status {
        Ok(exit_status) if exit_status.success() => Ok(()),
        Ok(exit_status) => Err(format!("Command exited with status: {}", exit_status)),
        Err(err) => Err(format!("Failed to execute command: {}", err)),
    }
}

fn setup_nextjs(project_name: &str) -> Result<(), String> {
        // Install Nextjs dependencies
        run_command(&format!(
            "npx create-next-app@latest {}",
            project_name
        ))?;


    let package_json_path = format!("{}/package.json", project_name);

    fs::create_dir_all(&project_name).map_err(|e| format!("Failed to create package.json file: {}", e))?;    

    fs::write(
        package_json_path,
        r#"{
  "name": "my-nextjs-app",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "node server.js",
    "lint": "next lint"
  },
  "dependencies": {
    "express": "^4.21.2",
    "next": "15.1.6",
    "react": "^19.0.0",
    "react-dom": "^19.0.0"
  },
  "devDependencies": {
    "@eslint/eslintrc": "^3",
    "@types/node": "^20",
    "@types/react": "^19",
    "@types/react-dom": "^19",
    "eslint": "^9",
    "eslint-config-next": "15.1.6",
    "postcss": "^8",
    "tailwindcss": "^3.4.1",
    "typescript": "^5"
  }
}
"#,)
    .map_err(|e| format!("Failed to write package.json: {}", e))?;

    Ok(())
}

fn setup_expressjs(project_name: &str) -> Result<(), String> {
    // Install Express dependencies
    run_command(&format!(
        "cd {} && npm install express",
        project_name
    ))?;

    let server_js_path = format!("{}/server.js", project_name);

    fs::create_dir_all(Path::new(&server_js_path).parent().unwrap())
        .map_err(|e| format!("Failed to create server.js: {}", e))?;

    fs::write(
        server_js_path,
        r#"const express = require('express');
const next = require('next');

const dev = process.env.NODE_ENV !== 'production';
const app = next({ dev });
const handle = app.getRequestHandler();

app.prepare().then(() => {
  const server = express();

  // Define your custom routes or middleware if necessary
  server.get('*', (req, res) => {
    return handle(req, res); // Let Next.js handle routing
  });

  // Start the server
  server.listen(3000, (err) => {
    if (err) throw err;
    console.log('> Ready on http://localhost:3000');
  });
});
"#)
        .map_err(|e| format!("Failed to write server.js: {}", e))?;

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