pub(super) static ADMIN_MIGRATOR: sqlx::migrate::Migrator =
    sqlx::migrate!("../server_admin_migrations");
