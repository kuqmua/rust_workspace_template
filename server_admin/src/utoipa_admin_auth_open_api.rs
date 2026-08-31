#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    newtype::IntoInnerFrom,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct UtoipaAdminAuthOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminAuthOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::UTOIPAADMINAUTHOPENAPI)
    }
}
