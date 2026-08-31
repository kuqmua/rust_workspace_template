#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    generate_accessor::Getters,
)]
pub struct AdminGeneratedToken {
    hash: crate::admin_token_hash::AdminTokenHash,
    token: crate::admin_opaque_token::AdminOpaqueToken,
}
