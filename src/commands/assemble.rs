use crate::utils::run::run_command;
use crate::utils::executor::execute_or_exit;
use std::fs;
use indicatif::ProgressBar;

// Function to setup a react project with Vite
pub fn setup_react_vite(project_name: &str, lang: &str, bar: &ProgressBar) -> Result<(), String> {
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

pub fn install_dependencies(project_name: &str, bar: &ProgressBar) -> Result<(), String> {
    bar.set_message("Installing dependencies...");
    let command = format!("cd {} && npm install", project_name);
    execute_or_exit(
        run_command(&command),
        "Failed to install dependencies.",
    );
    Ok(())
}

// Function to setup Tailwind CSS
pub fn setup_tailwindcss(project_name: &str, lang:&str, bar: &ProgressBar) -> Result<(), String> {
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


// Function to setup a React project with Vite and TailwindCSS for ShadcnUI
pub fn setup_tailwind(name: &str, lang: &str, bar: &ProgressBar) -> Result<(), String> { 
    // To be implemented
}