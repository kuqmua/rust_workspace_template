pub(crate) async fn authn_apply_refresh_failure_delay(
    std_admin_failure_delay_millis: &crate::std_admin_failure_delay_millis::StdAdminFailureDelayMillis,
) {
    tokio::time::sleep(tokio::time::Duration::from_millis(
        *std_admin_failure_delay_millis.get_inner(),
    ))
    .await;
}
