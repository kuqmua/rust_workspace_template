#[path = "exclusive_run_exclusive_run.rs"]
#[allow(
    clippy::module_inception,
    reason = "the compatibility facade retains its public path while the same-named owner receives a dedicated module"
)]
mod exclusive_run;
#[path = "exclusive_run_exclusive_run_already_active.rs"]
mod exclusive_run_already_active;
#[path = "exclusive_run_exclusive_run_atomic_bool.rs"]
mod exclusive_run_atomic_bool;
#[path = "exclusive_run_exclusive_run_guard.rs"]
mod exclusive_run_guard;

pub use exclusive_run::ExclusiveRun;
pub use exclusive_run_already_active::ExclusiveRunAlreadyActive;
pub use exclusive_run_guard::ExclusiveRunGuard;
