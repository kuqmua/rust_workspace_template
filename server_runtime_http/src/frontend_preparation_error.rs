#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum FrontendPreparationError {
    #[error("{message}: {0}", message = constants_str::FRONTEND_PREPARATION_ENVIRONMENT_ERROR)]
    Environment(#[source] crate::service_runtime_io_error::ServiceRuntimeIoError),
    #[error("{message}: {0}", message = constants_str::FRONTEND_PREPARATION_FILE_ERROR)]
    File(#[source] crate::service_runtime_io_error::ServiceRuntimeIoError),
    #[error("{message}: {0}", message = constants_str::FRONTEND_PREPARATION_READ_ERROR)]
    Read(#[source] crate::bounded_read_error::BoundedReadError),
    #[error("{message}", message = constants_str::FRONTEND_PREPARATION_NODE_INVALID)]
    NodeVersion,
    #[error("{message}: {0}", message = constants_str::FRONTEND_PREPARATION_NODE_PARSE_ERROR)]
    NodeVersionParse(#[source] crate::service_runtime_io_error::ServiceRuntimeIoError),
    #[error("{message}", message = constants_str::FRONTEND_PREPARATION_NODE_REQUIRED)]
    NodeUnsupported,
    #[error(
        "{message}: {frontend_build_step:?}: {source}",
        message = constants_str::FRONTEND_PREPARATION_COMMAND_ERROR
    )]
    Command {
        #[source]
        source: crate::service_runtime_io_error::ServiceRuntimeIoError,
        frontend_build_step: crate::frontend_build_step::FrontendBuildStep,
    },
    #[error(
        "{message}: {frontend_build_step:?}: {child_exit_status:?}",
        message = constants_str::FRONTEND_PREPARATION_COMMAND_FAILED
    )]
    Failed {
        child_exit_status: crate::child_exit_status::ChildExitStatus,
        frontend_build_step: crate::frontend_build_step::FrontendBuildStep,
    },
}
