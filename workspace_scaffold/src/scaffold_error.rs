#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum ScaffoldError {
    #[error(
        "usage: workspace-scaffold project <snake_case_name> <repository_url> | service <snake_case_name> <port> | generate <sync|check> | deployment <sync|check> | manifest <rendered_manifest> | manifest_example <rendered_manifest>"
    )]
    Arguments,
    #[error("deployment service catalog is invalid")]
    Catalog,
    #[error("{0}")]
    Manifest(#[from] macro_helpers::production_manifest_error::ProductionManifestError),
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
    #[error(
        "project or service name must start with a lowercase ASCII letter and use lowercase snake_case ASCII"
    )]
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_scaffold_io_error_conversion_preserves_variant_diagnostic_and_source() {
        [
            std::io::ErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied,
            std::io::ErrorKind::Other,
        ]
        .into_iter()
        .fold((), |(), error_kind| {
            let native_error = std::io::Error::from(error_kind);
            let native_display = native_error.to_string();
            let native_debug = format!("{native_error:?}");
            let error = crate::scaffold_error::ScaffoldError::from(native_error);
            assert!(error.to_string().ends_with(&native_display));
            assert!(format!("{error:?}").contains(&native_debug));
            assert!(std::error::Error::source(&error).is_some_and(|source| {
                source
                    .downcast_ref::<crate::scaffold_io_error::ScaffoldIoError>()
                    .is_some()
            }));
            assert!(matches!(error, crate::scaffold_error::ScaffoldError::Io(_)));
        });
    }
}
