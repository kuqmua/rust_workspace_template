#[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkPgPoolMaxConnectionsError {
    #[error("{pg_pool_max_connections:?}")]
    PgPoolMaxConnections {
        pg_pool_max_connections: super::PgPoolMaxConnectionsTryFromU32Error,
    },
    #[error("{:?}", .u32_parsing)]
    U32Parsing {
        u32_parsing: crate::domain_types::U32ParseIntError,
    },
}
