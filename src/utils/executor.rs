// Function to execute a command or exit the process if it fails
pub fn execute_or_exit(result: Result<(), String>, error_message: &str) {
    if let Err(e) = result {
        eprintln!("{}: {}", error_message, e);
        std::process::exit(1);
    }
}