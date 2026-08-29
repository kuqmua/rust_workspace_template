pub use super::build_service_runtime::build_service_runtime;
pub use super::service_runtime_io_error::ServiceRuntimeIoError;
pub use super::tokio_service_runtime::TokioServiceRuntime;
pub use super::wait_for_service_shutdown_signal::wait_for_service_shutdown_signal;
#[cfg(test)]
mod tests {
    #[test]
    fn service_runtime_builder_enables_tokio_runtime() {
        let wrapped_runtime = super::build_service_runtime()
            .expect("5ecc3726 service_runtime_builder_enables_tokio_runtime invariant must hold");
        let runtime = tokio::runtime::Runtime::from(wrapped_runtime);
        assert_eq!(runtime.block_on(async { 2u8 + 2u8 }), 4u8);
    }
}

// Root-owned module compatibility wrappers.
mod build_service_runtime {
    pub use super::super::build_service_runtime::*;
}
mod service_runtime_io_error {
    pub use super::super::service_runtime_io_error::*;
}
mod tokio_service_runtime {
    pub use super::super::tokio_service_runtime::*;
}
mod wait_for_service_shutdown_signal {
    pub use super::super::wait_for_service_shutdown_signal::*;
}
