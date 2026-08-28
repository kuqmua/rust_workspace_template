#[allow(clippy::single_call_fn)] // migration mode has one process entrypoint
pub(crate) async fn migrate_notification(
    config: &notification_service_config::config::Config,
) -> Result<(), crate::domain_types::NotificationServiceError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(**config.pg_pool_max_connections())
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Database(
                crate::domain_types::SqlxNotificationDatabaseError::from(error),
            )
        })?;
    sqlx::migrate!("../notification_service_migrations")
        .run(&pool)
        .await
        .map_err(|error| {
            crate::domain_types::NotificationServiceError::Migration(
                crate::domain_types::SqlxNotificationMigrationError::from(error),
            )
        })
}
