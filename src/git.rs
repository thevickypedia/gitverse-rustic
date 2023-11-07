use std::process::{Command, Stdio};

use log::{debug, error};

pub fn run(command: &str) -> String {
    let output = Command::new("sh")  // invoke a shell
        .arg("-c")  // execute command as interpreted by program
        .arg(command)  // run the command
        .stdout(Stdio::piped())  // Redirect stdout to /dev/null
        .stderr(Stdio::null())  // Redirect stderr to /dev/null
        .output()  // Capture both stdout and stderr
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let exit_code = output.status.code(); // Option<i32>
    match exit_code {
        Some(0) => {
            debug!("Command '{}' executed successfully", command);
            return stdout.to_string();
        }
        Some(code) => {
            error!("Command '{}' failed with exit code: {}", command, code);
        }
        None => {
            error!("Command '{}' failed, but couldn't retrieve exit code", command);
            error!("Standard Output: {}", stdout);
            error!("Standard Error: {}", stderr);
        }
    }
    return "FAILED".to_string();
}
