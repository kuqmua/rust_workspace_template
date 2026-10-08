#![allow(
    unused_crate_dependencies,
    reason = "the startup integration target uses Tokio and constants while service dependencies belong to the separate executable"
)]

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_notification_service_startup_rejects_invalid_socket_configuration() {
        let result = tokio::process::Command::new(env!("CARGO_BIN_EXE_notification_service"))
            .env_clear()
            .env(
                stringify!(notification_database_url).to_ascii_uppercase(),
                constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .env(
                stringify!(request_timeout_seconds).to_ascii_uppercase(),
                30u64.to_string(),
            )
            .env(
                constants_str::ENV_NAMES_PG_POOL_MAX_CONNECTIONS,
                10u32.to_string(),
            )
            .env(
                constants_str::ENV_NAMES_TRACING_FORMAT,
                constants_str::TRACING_FORMAT_TEXT,
            )
            .env(stringify!(SVC_MODE), constants_str::SERVICE_MODE_SERVE)
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
