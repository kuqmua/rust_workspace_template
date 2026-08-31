pub(crate) async fn authn_apply_refresh_failure_delay(
    delay: &crate::std_admin_failure_delay_millis::StdAdminFailureDelayMillis,
) {
    tokio::time::sleep(tokio::time::Duration::from_millis(*delay.get_inner())).await;
}
