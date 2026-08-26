pub(super) async fn apply_refresh_failure_delay(delay: super::StdAdminFailureDelayMillis) {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay.0)).await;
}
