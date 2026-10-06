#![allow(
    unused_crate_dependencies,
    reason = "the startup integration target uses Tokio and constants while service dependencies belong to the separate executable"
)]

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_server_startup_rejects_invalid_socket_configuration() {
        let result = tokio::process::Command::new(env!("CARGO_BIN_EXE_server"))
            .env(
                constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS,
                constants_str::X,
            )
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;
        assert!(result.is_ok_and(|status| status.code() == Some(1i32)));
    }
}
