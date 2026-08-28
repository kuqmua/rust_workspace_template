#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub struct UtoipaOpenApiRefMut<'value_lt>(pub(super) &'value_lt mut utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaOpenApiRefMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::UTOIPAOPENAPIREFMUT)
            .finish_non_exhaustive()
    }
}
