#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "i32")]
pub struct NotZeroUnsignedPartOfI32(pub(super) std::num::NonZeroI32);

impl From<std::num::NonZeroU16> for NotZeroUnsignedPartOfI32 {
    fn from(value: std::num::NonZeroU16) -> Self {
        Self::from(std::num::NonZeroI32::from(value))
    }
}

impl From<std::num::NonZeroI32> for NotZeroUnsignedPartOfI32 {
    fn from(value: std::num::NonZeroI32) -> Self {
        Self(value)
    }
}

impl utoipa::PartialSchema for NotZeroUnsignedPartOfI32 {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::Integer)
            .minimum(Some(1.0f64))
            .maximum(Some(f64::from(i32::MAX)))
            .into()
    }
}

impl utoipa::ToSchema for NotZeroUnsignedPartOfI32 {}

impl TryFrom<i32> for NotZeroUnsignedPartOfI32 {
    type Error = crate::not_zero_unsigned_part_of_i32_try_from_i32_error::NotZeroUnsignedPartOfI32TryFromI32Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        let value =
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(v).map_err(|error| {
                Self::Error::UnsignedPartOfI32TryFromI32Error {
                    v: error,
                    location: location_macros::location!(),
                }
            })?;
        std::num::NonZeroI32::new(value.0)
            .map(Self)
            .ok_or_else(|| Self::Error::IsZero {
                location: location_macros::location!(),
            })
    }
}

impl to_err_string::to_err_string::ToErrString for NotZeroUnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        crate::unsigned_part_of_i32::UnsignedPartOfI32::from(self.0).to_err_string()
    }
}

impl sqlx::Type<sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <crate::unsigned_part_of_i32::UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::compatible(
            ty,
        )
    }

    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::unsigned_part_of_i32::UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <crate::unsigned_part_of_i32::UnsignedPartOfI32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(
            &crate::unsigned_part_of_i32::UnsignedPartOfI32::from(self.0),
            buf,
        )
    }
}

impl NotZeroUnsignedPartOfI32 {
    #[must_use]
    pub fn get(&self) -> crate::unsigned_part_of_i32::UnsignedPartOfI32 {
        crate::unsigned_part_of_i32::UnsignedPartOfI32::from(self.0)
    }
}

impl Default for NotZeroUnsignedPartOfI32 {
    fn default() -> Self {
        Self::from(std::num::NonZeroI32::new(1i32).unwrap_or(std::num::NonZeroI32::MAX))
    }
}

impl crate::default_some_one_element::DefaultSomeOneElement for NotZeroUnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(std::num::NonZeroU16::MIN)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nonzero_database_value_rejects_zero() {
        assert!(matches!(
            crate::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32::try_from(constants_i32::ZERO),
            Err(crate::not_zero_unsigned_part_of_i32_try_from_i32_error::NotZeroUnsignedPartOfI32TryFromI32Error::IsZero { .. })
        ));
        assert!(matches!(
            crate::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32::try_from(1i32),
            Ok(_value)
        ));
        assert_eq!(
            crate::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32::default().get(),
            crate::unsigned_part_of_i32::UnsignedPartOfI32::from(1u16)
        );
    }
}
