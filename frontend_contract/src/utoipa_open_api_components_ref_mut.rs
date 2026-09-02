#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub struct UtoipaOpenApiComponentsRefMut<'value_lt>(
    &'value_lt mut utoipa::openapi::schema::Components,
);
impl std::fmt::Debug for UtoipaOpenApiComponentsRefMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::UTOIPAOPENAPICOMPONENTSREFMUT)
            .finish_non_exhaustive()
    }
}
