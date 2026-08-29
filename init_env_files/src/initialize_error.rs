#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum InitializeError {
    #[error("workspace member path is invalid: {member}")]
    InvalidMember {
        member: crate::workspace_member::WorkspaceMember,
    },
    #[error("failed to parse workspace manifest")]
    ManifestParse {
        #[source]
        source: crate::toml_init_error::TomlInitError,
    },
    #[error("workspace manifest does not contain a members array")]
    MembersMissing,
    #[error("failed to read environment example")]
    ReadExample {
        #[source]
        source: server_runtime_http::bounded_read_error::BoundedReadError,
    },
    #[error("failed to read workspace manifest")]
    ReadManifest {
        #[source]
        source: server_runtime_http::bounded_read_error::BoundedReadError,
    },
    #[error(transparent)]
    String(#[from] crate::init_string_error::InitStringError),
    #[error("failed to write environment file")]
    WriteEnvironment {
        #[source]
        source: crate::init_io_error::InitIoError,
    },
}
