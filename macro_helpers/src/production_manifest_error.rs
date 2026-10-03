#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum ProductionManifestError {
    #[error("example production values were accepted")]
    ExampleAccepted,
    #[error("production manifest contains an example or replacement value")]
    Placeholder,
    #[error("production manifest must declare the real trusted ingress proxy ranges")]
    TrustedProxy,
    #[error("production manifest must pin both service images by sha256 digest")]
    Images,
    #[error("production manifest must enable production mode")]
    ProductionMode,
    #[error("production manifest must require secure administrator cookies")]
    SecureCookie,
    #[error("production manifest is missing Deployment")]
    Deployment,
    #[error("production manifest is missing NetworkPolicy")]
    NetworkPolicy,
    #[error("production manifest is missing PodDisruptionBudget")]
    DisruptionBudget,
}
