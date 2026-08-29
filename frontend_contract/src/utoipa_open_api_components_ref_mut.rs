#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub struct UtoipaOpenApiComponentsRefMut<'value_lt>(
    pub(super) &'value_lt mut utoipa::openapi::schema::Components,
);
impl std::fmt::Debug for UtoipaOpenApiComponentsRefMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::catalog::UTOIPAOPENAPICOMPONENTSREFMUT)
            .finish_non_exhaustive()
    }
}
