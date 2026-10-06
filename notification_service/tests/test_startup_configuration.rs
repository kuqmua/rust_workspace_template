#![allow(
    unused_crate_dependencies,
    reason = "the startup integration target uses Tokio and constants while service dependencies belong to the separate executable"
)]

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_notification_service_startup_rejects_invalid_socket_configuration() {
        let result = tokio::process::Command::new(env!("CARGO_BIN_EXE_notification_service"))
            .env(
                stringify!(notification_service_socket_address).to_ascii_uppercase(),
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
