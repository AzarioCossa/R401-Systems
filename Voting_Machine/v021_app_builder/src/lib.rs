pub mod app_builder;
pub mod configuration;

pub use app_builder::run_app;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::configuration::Configuration;

    #[tokio::test]
async fn test_voting_logic() {
    use crate::run_app;
    use crate::configuration::Configuration;
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = Command::new("cargo")
        .args(["run", "--", "--candidates", "Alice", "Bob"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    writeln!(stdin, "voter John Alice").unwrap();
    writeln!(stdin, "voter Jane Bob").unwrap();
    writeln!(stdin, "score").unwrap();

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("John a voté pour Alice"));
    assert!(stdout.contains("Jane a voté pour Bob"));
    assert!(stdout.contains("Alice: 1"));
    assert!(stdout.contains("Bob: 1"));
}

}