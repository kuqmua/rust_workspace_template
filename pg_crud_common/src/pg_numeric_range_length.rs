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
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(try_from = "u64")]
pub struct PgNumericRangeLength(std::num::NonZeroU64);

impl From<std::num::NonZeroU16> for PgNumericRangeLength {
    fn from(value: std::num::NonZeroU16) -> Self {
        Self::from(std::num::NonZeroU64::from(value))
    }
}

impl TryFrom<u64> for PgNumericRangeLength {
    type Error = crate::pg_numeric_range_length_error::PgNumericRangeLengthError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or_else(|| Self::Error::IsZero {
                location: proc_macro_location_bang::location!(),
            })
    }
}

impl utoipa::PartialSchema for PgNumericRangeLength {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::Integer)
            .minimum(Some(1.0f64))
            .into()
    }
}

impl utoipa::ToSchema for PgNumericRangeLength {}

impl sqlx::Type<sqlx::Postgres> for PgNumericRangeLength {
    fn compatible(type_info: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::compatible(type_info)
    }

    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BigDecimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for PgNumericRangeLength {
    fn encode_by_ref(
        &self,
        pg_argument_buffer: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let decimal = sqlx::types::BigDecimal::from(self.0.get());
        <sqlx::types::BigDecimal as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(
            &decimal,
            pg_argument_buffer,
        )
    }
}

impl Default for PgNumericRangeLength {
    fn default() -> Self {
        Self::from(std::num::NonZeroU16::MIN)
    }
}

impl crate::default_some_one_element::DefaultSomeOneElement for PgNumericRangeLength {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}

impl crate::pg_range_length_sql::PgRangeLengthSql for PgNumericRangeLength {
    const USE_NUMERIC_DIFFERENCE: bool = true;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_numeric_range_length_json_preserves_integer_boundaries() {
        assert!(
            [
                1u64,
                u64::from(u16::MAX),
                2_147_483_647u64,
                9_223_372_036_854_775_807u64,
                u64::MAX
            ]
            .into_iter()
            .all(|value| {
                crate::pg_numeric_range_length::PgNumericRangeLength::try_from(value).is_ok_and(
                    |length| {
                        serde_json::to_value(length).is_ok_and(|json| {
                            json == value
                                && serde_json::from_value::<
                                    crate::pg_numeric_range_length::PgNumericRangeLength,
                                >(json)
                                .is_ok_and(|decoded| decoded == length)
                        })
                    },
                )
            })
        );
        assert!(
            [
                serde_json::Value::Null,
                serde_json::Value::Bool(true),
                serde_json::Value::from(-1i64),
                serde_json::Value::from(1.5f64),
                serde_json::Value::from(constants_str::X),
            ]
            .into_iter()
            .all(|json| {
                serde_json::from_value::<crate::pg_numeric_range_length::PgNumericRangeLength>(json)
                    .is_err_and(|error| error.is_data())
            })
        );
        assert_eq!(
            crate::pg_numeric_range_length::PgNumericRangeLength::try_from(u64::from(u16::MAX)),
            Ok(crate::pg_numeric_range_length::PgNumericRangeLength::from(
                std::num::NonZeroU16::MAX
            ))
        );
    }

    #[test]
    fn test_full_u64_range_length_validation() {
        assert!(matches!(
            crate::pg_numeric_range_length::PgNumericRangeLength::try_from(0u64),
            Err(crate::pg_numeric_range_length_error::PgNumericRangeLengthError::IsZero { .. })
        ));
        assert!(
            crate::pg_numeric_range_length::PgNumericRangeLength::try_from(u64::MAX)
                .is_ok_and(|length| length
                    > crate::pg_numeric_range_length::PgNumericRangeLength::default())
        );
        assert!(matches!(
            serde_json::from_value::<crate::pg_numeric_range_length::PgNumericRangeLength>(serde_json::Value::from(0u64)),
            Err(error) if error.is_data()
        ));
    }

    #[test]
    fn test_full_u64_range_length_encodes_as_exact_numeric() {
        assert!(
            crate::pg_numeric_range_length::PgNumericRangeLength::try_from(u64::MAX).is_ok_and(
                |length| {
                    let mut actual = sqlx::postgres::PgArgumentBuffer::default();
                    let mut expected = sqlx::postgres::PgArgumentBuffer::default();
                    let reference = sqlx::types::BigDecimal::from(u64::MAX);
                    matches!(
                        <crate::pg_numeric_range_length::PgNumericRangeLength as sqlx::Encode<
                            sqlx::Postgres,
                        >>::encode_by_ref(&length, &mut actual),
                        Ok(sqlx::encode::IsNull::No)
                    ) && matches!(
                        <sqlx::types::BigDecimal as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(
                            &reference,
                            &mut expected
                        ),
                        Ok(sqlx::encode::IsNull::No)
                    ) && actual.as_slice() == expected.as_slice()
                }
            )
        );
    }
}
