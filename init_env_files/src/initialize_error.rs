use crate::domain_types::{InitIoError, InitStringError, TomlInitError, WorkspaceMember};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum InitializeError {
    #[error("workspace member path is invalid: {member}")]
    InvalidMember { member: WorkspaceMember },
    #[error("failed to parse workspace manifest")]
    ManifestParse {
        #[source]
        source: TomlInitError,
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
    String(#[from] InitStringError),
    #[error("failed to write environment file")]
    WriteEnvironment {
        #[source]
        source: InitIoError,
    },
}
