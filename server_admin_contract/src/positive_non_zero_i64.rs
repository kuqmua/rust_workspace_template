#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
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
        if let Some(non_zero) =
            std::num::NonZeroI64::new(value).filter(|candidate| candidate.get().is_positive())
        {
            return Ok(Self(non_zero));
        }
        Err(crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid)
    }
}

impl PositiveNonZeroI64 {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_positive_identifier_schema_preserves_integer_format_and_minimum() {
        let schema = <super::PositiveNonZeroI64 as utoipa::PartialSchema>::schema();
        let serialized = serde_json::to_value(schema);
        assert!(serialized.is_ok());
        let Ok(wire) = serialized else {
            return;
        };
        assert_eq!(
            wire,
            serde_json::json!({(stringify!(type)): stringify!(integer), (stringify!(format)): stringify!(int64), (stringify!(minimum)): 1i64})
        );
    }
}
