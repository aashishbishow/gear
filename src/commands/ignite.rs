use crate::utils::run::run_command;
use crate::utils::executor::execute_or_exit;

// Function to ignite the development server
pub fn ignite_development(project_name: &str) {
    println!("Starting the development server...");
    execute_or_exit(
        run_command(&format!("cd {} && npm run dev", project_name)),
        "Failed to start the development server.",
    );
}
