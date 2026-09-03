#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct UtoipaOpenApiComponentsRefMut<'value_lt>(
    &'value_lt mut utoipa::openapi::schema::Components,
);
impl std::fmt::Debug for UtoipaOpenApiComponentsRefMut<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::UTOIPAOPENAPICOMPONENTSREFMUT)
            .finish_non_exhaustive()
    }
}
