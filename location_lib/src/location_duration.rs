#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
#[serde(from = "std::time::Duration")]
pub struct LocationDuration(pub(super) std::time::Duration);
impl utoipa::PartialSchema for LocationDuration {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::catalog::SECS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int64,
                    ))),
            )
            .property(
                constants_str::catalog::NANOS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    )))
                    .minimum(Some(0.0))
                    .maximum(Some(999_999_999.0)),
            )
            .required(constants_str::catalog::SECS)
            .required(constants_str::catalog::NANOS)
            .build()
            .into()
    }
}
impl utoipa::ToSchema for LocationDuration {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::catalog::STDLOCATIONDURATION)
    }
}
