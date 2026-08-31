#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct UtoipaAdminOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::UTOIPAADMINOPENAPI)
    }
}
