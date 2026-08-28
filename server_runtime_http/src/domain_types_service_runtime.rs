pub use crate::build_service_runtime::build_service_runtime;
pub use crate::service_runtime_io_error::ServiceRuntimeIoError;
pub use crate::tokio_service_runtime::TokioServiceRuntime;
pub use crate::wait_for_service_shutdown_signal::wait_for_service_shutdown_signal;

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
    pub use crate::build_service_runtime::*;
}
mod service_runtime_io_error {
    pub use crate::service_runtime_io_error::*;
}
mod tokio_service_runtime {
    pub use crate::tokio_service_runtime::*;
}
mod wait_for_service_shutdown_signal {
    pub use crate::wait_for_service_shutdown_signal::*;
}
