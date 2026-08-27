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
pub struct UnsignedPartOfI32(pub(super) i32);

impl From<u16> for UnsignedPartOfI32 {
    fn from(value: u16) -> Self {
        Self(i32::from(value))
    }
}

impl From<std::num::NonZeroI32> for UnsignedPartOfI32 {
    fn from(value: std::num::NonZeroI32) -> Self {
        Self(value.get())
    }
}

impl TryFrom<i32> for UnsignedPartOfI32 {
    type Error = crate::domain_types::UnsignedPartOfI32TryFromI32Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v >= 0 {
            Ok(Self(v))
        } else {
            Err(Self::Error::LessThanZero {
                v: crate::domain_types::UnsignedPartOfI32Raw::from(v),
                location: location_macros::location!(),
            })
        }
    }
}

impl to_err_string::domain_types::ToErrString for UnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(self.0.to_string())
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
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

impl crate::domain_types::DefaultSomeOneElement for UnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(constants_u16::ZERO)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unsigned_database_value_rejects_negative_input() {
        assert!(matches!(
            super::UnsignedPartOfI32::try_from(-1i32),
            Err(crate::domain_types::UnsignedPartOfI32TryFromI32Error::LessThanZero { .. })
        ));
        assert_eq!(
            super::UnsignedPartOfI32::try_from(7i32).expect(
                "ea8c2d71 unsigned_database_value_rejects_negative_input invariant must hold"
            ),
            super::UnsignedPartOfI32::from(7u16)
        );
    }
}
