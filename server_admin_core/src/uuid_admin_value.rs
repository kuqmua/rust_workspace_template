#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::FromInner,
    newtype::GetInner,
)]
#[serde(from = "uuid::Uuid")]
pub struct UuidAdminValue(uuid::Uuid);
impl utoipa::PartialSchema for UuidAdminValue {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .format(Some(utoipa::openapi::SchemaFormat::Custom(
                constants_str::PG_CRUD_PG_UUID.to_owned(),
            )))
            .into()
    }
}
impl utoipa::ToSchema for UuidAdminValue {}
