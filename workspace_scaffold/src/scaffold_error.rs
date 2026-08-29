#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum ScaffoldError {
    #[error(
        "usage: workspace-scaffold project <snake_case_name> <repository_url> | service <snake_case_name> <port> | generate <sync|check> | deployment <sync|check>"
    )]
    Arguments,
    #[error("deployment service catalog is invalid")]
    Catalog,
    #[error("generated code-style snapshots are not synchronized")]
    GeneratedCodeStyle,
    #[error("generated configuration projections are not synchronized")]
    GeneratedConfig,
    #[error("generated deployment projections are not synchronized")]
    GeneratedDeployment,
    #[error("workspace operation failed: {0}")]
    Io(#[from] crate::scaffold_io_error::ScaffoldIoError),
    #[error("workspace file does not contain the expected template marker")]
    Marker,
    #[error("project or service name must be non-empty lowercase snake_case ASCII")]
    ProjectName,
    #[error("workspace content read failed: {0}")]
    Read(#[from] server_runtime_http::bounded_read_error::BoundedReadError),
    #[error("repository URL must use https:// and must not end with /")]
    RepositoryUrl,
    #[error("service destination already exists")]
    ServiceExists,
    #[error("service port must be greater than zero")]
    ServicePort,
}
impl From<std::io::Error> for ScaffoldError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(crate::scaffold_io_error::ScaffoldIoError::from(value))
    }
}
