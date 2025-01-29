use crate::{commands::assemble::{setup_react_vite, install_dependencies, setup_tailwindcss, setup_tailwind}, utils::executor::execute_or_exit};
use indicatif::ProgressBar;

// Function to setup a React project with Vite and TailwindCSS
pub fn setup_fabricate(name: &str, lang: &str, bar: &ProgressBar, flag: bool) {
    let _ = flag;
    if lang != "js" && lang != "ts" {
        eprintln!("Invalid language: {}. Supported values are 'js' or 'ts'.", lang);
        std::process::exit(1);
    }

    bar.set_message("Setting up React with Vite...");
    execute_or_exit(setup_react_vite(name, lang, bar), "Failed to setup React with Vite.");
    bar.inc(1);

    bar.set_message("Installing dependencies...");
    execute_or_exit(install_dependencies(name, bar), "Failed to install dependencies.");
    bar.inc(1);

    if !flag {
        bar.finish_with_message("Installing TailwindCSS...");
        execute_or_exit(setup_tailwindcss(name, lang, bar), "Failed to setup Tailwind.");
        bar.inc(1);
    }
    else{
    bar.set_message("Installing TailwindCSS...");
    execute_or_exit(setup_tailwind(name, lang, bar), "Failed to setup Tailwind.");
    bar.inc(1);
    }

    bar.finish_with_message("React-Vite project setup complete!");
    println!("Navigate to '{}' to start building your project.", name);

}
