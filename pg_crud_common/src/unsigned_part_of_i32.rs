#[derive(
    generate_accessor::Getters,
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
    newtype::FromGetter,
)]
#[from_getter(source = std::num::NonZeroI32, getter = get)]
#[serde(try_from = "i32")]
pub struct UnsignedPartOfI32(i32);

impl From<u16> for UnsignedPartOfI32 {
    fn from(value: u16) -> Self {
        Self(i32::from(value))
    }
}

impl TryFrom<i32> for UnsignedPartOfI32 {
    type Error = crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Self(value))
        } else {
            Err(Self::Error::LessThanZero {
                v: crate::unsigned_part_of_i32_raw::UnsignedPartOfI32Raw::from(value),
                location: location_macros::location!(),
            })
        }
    }
}

impl to_err_string::to_err_string::ToErrString for UnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.0.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}

impl sqlx::Type<sqlx::Postgres> for UnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }

    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for UnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}

impl UnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> Self {
        *self
    }
}

impl crate::default_some_one_element::DefaultSomeOneElement for UnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(constants_u16::ZERO)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unsigned_database_value_rejects_negative_input() {
        assert!(matches!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(-1i32),
            Err(crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error::LessThanZero { .. })
        ));
        assert_eq!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(7i32).expect(
                "ea8c2d71 unsigned_database_value_rejects_negative_input invariant must hold"
            ),
            crate::unsigned_part_of_i32::UnsignedPartOfI32::from(7u16)
        );
        assert_eq!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::from(
                std::num::NonZeroI32::new(7i32)
                    .expect("dd53fc4d nonzero test fixture must remain nonzero"),
            ),
            crate::unsigned_part_of_i32::UnsignedPartOfI32::from(7u16)
        );
    }
}
