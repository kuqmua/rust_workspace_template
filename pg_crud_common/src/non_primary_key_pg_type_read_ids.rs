#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(from = "crate::v::V<Option<()>>")]
#[derive(newtype::FromInner)]
pub struct NonPrimaryKeyPgTypeReadIds(crate::v::V<Option<()>>);

impl utoipa::PartialSchema for NonPrimaryKeyPgTypeReadIds {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::PG_CRUD_V_FIELD,
                utoipa::openapi::schema::OneOfBuilder::new()
                    .item(
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::Type::Null),
                    )
                    .item(utoipa::openapi::schema::empty()),
            )
            .required(constants_str::PG_CRUD_V_FIELD)
            .build()
            .into()
    }
}

impl utoipa::ToSchema for NonPrimaryKeyPgTypeReadIds {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::NONPRIMARYKEYPGTYPEREADIDS)
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for NonPrimaryKeyPgTypeReadIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value).map(|v0| v0.0)
    }
}

impl sqlx::Type<sqlx::Postgres> for NonPrimaryKeyPgTypeReadIds {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }

    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl Default for NonPrimaryKeyPgTypeReadIds {
    fn default() -> Self {
        Self::from(crate::v::V::new(None))
    }
}
