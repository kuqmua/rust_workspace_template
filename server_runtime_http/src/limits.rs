pub use crate::acquire_permit::acquire_permit;
pub use crate::acquire_permit_error::AcquirePermitError;
pub use crate::arc_tokio_semaphore::ArcTokioSemaphore;
pub use crate::permit_wait_timeout_duration::PermitWaitTimeoutDuration;
pub use crate::retry_after_secs::RetryAfterSecs;
pub use crate::retry_after_secs_try_from_u64_error::RetryAfterSecsTryFromU64Error;
pub use crate::semaphore_permit_count_non_zero_usize::SemaphorePermitCountNonZeroUsize;
pub use crate::tokio_acquire_error::TokioAcquireError;
pub use crate::tokio_owned_semaphore_permit::TokioOwnedSemaphorePermit;

// Root-owned module compatibility wrappers.
mod acquire_permit {
    pub use crate::acquire_permit::*;
}
mod acquire_permit_error {
    pub use crate::acquire_permit_error::*;
}
mod arc_tokio_semaphore {
    pub use crate::arc_tokio_semaphore::*;
}
mod permit_wait_timeout_duration {
    pub use crate::permit_wait_timeout_duration::*;
}
mod retry_after_secs {
    pub use crate::retry_after_secs::*;
}
mod retry_after_secs_try_from_u64_error {
    pub use crate::retry_after_secs_try_from_u64_error::*;
}
mod semaphore_permit_count_non_zero_usize {
    pub use crate::semaphore_permit_count_non_zero_usize::*;
}
mod tokio_acquire_error {
    pub use crate::tokio_acquire_error::*;
}
mod tokio_owned_semaphore_permit {
    pub use crate::tokio_owned_semaphore_permit::*;
}
