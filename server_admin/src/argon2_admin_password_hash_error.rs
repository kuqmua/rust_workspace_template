#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::DebugTransparent,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct Argon2AdminPasswordHashError(argon2::password_hash::Error);
