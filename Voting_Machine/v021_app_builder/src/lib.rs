pub mod app_builder;
pub mod configuration;

pub use app_builder::run_app;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::configuration::Configuration;

    #[tokio::test]
    async fn test_run_app() {
        // Simulate command line arguments
        let args = vec![
            "program_name".to_string(),
            "--candidates".to_string(),
            "Alice".to_string(),
            "Bob".to_string(),
        ];
        env::set_var("RUST_TEST_ARGS", args.join(" "));

        // Call the function
        let config = Configuration {
            candidates: vec!["Alice".to_string(), "Bob".to_string()],
        };
        let result = run_app(config).await;

        // Check the result
        assert!(result.is_ok());
    }
}