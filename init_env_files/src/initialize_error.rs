#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum InitializeError {
    #[error("workspace member path is invalid: {member}")]
    InvalidMember { member: crate::WorkspaceMember },
    #[error("failed to parse workspace manifest")]
    ManifestParse {
        #[source]
        source: crate::TomlInitError,
    },
    #[error("workspace manifest does not contain a members array")]
    MembersMissing,
    #[error("failed to read environment example")]
    ReadExample {
        #[source]
        source: server_runtime_http::domain_types::BoundedReadError,
    },
    #[error("failed to read workspace manifest")]
    ReadManifest {
        #[source]
        source: server_runtime_http::domain_types::BoundedReadError,
    },
    #[error(transparent)]
    String(#[from] crate::InitStringError),
    #[error("failed to write environment file")]
    WriteEnvironment {
        #[source]
        source: crate::InitIoError,
    },
}
