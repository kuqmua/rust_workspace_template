#[derive(
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
)]
pub struct AdminGeneratedToken {
    hash: crate::admin_token_hash::AdminTokenHash,
    token: crate::admin_opaque_token::AdminOpaqueToken,
}
