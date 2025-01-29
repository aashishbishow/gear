use std::process::Command;

// Function to check for dependencies
pub fn check_dependencies(dependencies: &[&str]) {
    for dep in dependencies {
        if Command::new(dep).arg("--version").output().is_err() {
            eprintln!("Missing dependency: {}. Please install it.", dep);
            std::process::exit(1);
        }
    }
}