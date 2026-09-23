pub(crate) static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate::Migrator {
    ignore_missing: true,
    ..sqlx::migrate!("../server_admin_migrations")
};
