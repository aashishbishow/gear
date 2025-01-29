use std::process::{Command, Stdio};

// Function to run a command
pub fn run_command(command: &str) -> Result<(), String> {
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
