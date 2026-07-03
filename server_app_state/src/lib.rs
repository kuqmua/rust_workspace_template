#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ServerAppState {
    config: server_config::Config,
    pg_pool: app_state::PgPool,
}

impl ServerAppState {
    #[must_use]
    pub const fn new(config: server_config::Config, pg_pool: app_state::PgPool) -> Self {
        Self { config, pg_pool }
    }
}

impl app_state::GetPgPool for ServerAppState {
    fn get_pg_pool(&self) -> &app_state::PgPool {
        &self.pg_pool
    }
}
