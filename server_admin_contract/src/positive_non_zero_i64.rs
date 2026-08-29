#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    newtype::Display,
    newtype::FromInner,
)]
pub struct PositiveNonZeroI64(std::num::NonZeroI64);

impl utoipa::PartialSchema for PositiveNonZeroI64 {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::Integer)
            .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                utoipa::openapi::KnownFormat::Int64,
            )))
            .minimum(Some(1.0))
            .into()
    }
}

impl utoipa::ToSchema for PositiveNonZeroI64 {}

impl TryFrom<i64> for PositiveNonZeroI64 {
    type Error = crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        std::num::NonZeroI64::new(value)
            .filter(|non_zero| non_zero.get().is_positive())
            .map(Self)
            .ok_or(crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error)
    }
}

impl PositiveNonZeroI64 {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
}
