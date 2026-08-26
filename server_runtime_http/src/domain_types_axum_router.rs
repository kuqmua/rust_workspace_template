#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the request-id layer owner applies middleware to this private router wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct AxumRouter(axum::Router);
