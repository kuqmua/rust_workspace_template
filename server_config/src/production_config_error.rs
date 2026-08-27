#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ProductionConfigError {
    #[error("production administrator cookies must be secure")]
    AdminCookieInsecure,
    #[error("production administrator Swagger must be disabled")]
    AdminSwaggerEnabled,
    #[error("production CORS origins must use explicit HTTPS URLs")]
    CorsOriginInsecure,
    #[error("production administrator JWT secret must not use the template development value")]
    DevelopmentJwtSecret,
}
