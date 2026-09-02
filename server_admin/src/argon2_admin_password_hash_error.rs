#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::DebugTransparent,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct Argon2AdminPasswordHashError(argon2::password_hash::Error);
