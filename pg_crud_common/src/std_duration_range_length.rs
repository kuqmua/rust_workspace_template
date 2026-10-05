const MICROSECONDS_PER_DAY: u128 = 86_400_000_000u128;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "std::time::Duration")]
pub struct StdDurationRangeLength(std::time::Duration);

impl From<std::num::NonZeroU16> for StdDurationRangeLength {
    fn from(value: std::num::NonZeroU16) -> Self {
        Self(std::time::Duration::from_micros(
            std::num::NonZeroU64::from(value).get(),
        ))
    }
}

impl TryFrom<std::time::Duration> for StdDurationRangeLength {
    type Error = crate::std_duration_range_length_error::StdDurationRangeLengthError;

    #[allow(
        clippy::integer_division,
        clippy::integer_division_remainder_used,
        reason = "PostgreSQL interval validation needs exact whole-day division without floating-point rounding"
    )]
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            return Err(Self::Error::IsZero {
                location: proc_macro_location_bang::location!(),
            });
        }
        if !value.as_nanos().is_multiple_of(1_000u128) {
            return Err(Self::Error::SubmicrosecondPrecision {
                location: proc_macro_location_bang::location!(),
            });
        }
        if i32::try_from(value.as_micros() / MICROSECONDS_PER_DAY).is_err() {
            return Err(Self::Error::ExceedsPostgresIntervalDays {
                location: proc_macro_location_bang::location!(),
            });
        }
        Ok(Self(value))
    }
}

impl utoipa::PartialSchema for StdDurationRangeLength {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::SECS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer),
            )
            .property(
                constants_str::NANOS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer),
            )
            .required(constants_str::SECS)
            .required(constants_str::NANOS)
            .into()
    }
}

impl utoipa::ToSchema for StdDurationRangeLength {}

impl sqlx::Type<sqlx::Postgres> for StdDurationRangeLength {
    fn compatible(type_info: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::compatible(type_info)
    }

    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for StdDurationRangeLength {
    #[allow(
        clippy::integer_division,
        clippy::integer_division_remainder_used,
        reason = "PostgreSQL interval binary encoding requires exact day and microsecond quotient and remainder"
    )]
    fn encode_by_ref(
        &self,
        pg_argument_buffer: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let total_microseconds = self.0.as_micros();
        let interval = sqlx::postgres::types::PgInterval {
            months: 0i32,
            days: i32::try_from(total_microseconds / MICROSECONDS_PER_DAY)?,
            microseconds: i64::try_from(total_microseconds % MICROSECONDS_PER_DAY)?,
        };
        <sqlx::postgres::types::PgInterval as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(
            &interval,
            pg_argument_buffer,
        )
    }
}

impl Default for StdDurationRangeLength {
    fn default() -> Self {
        Self::from(std::num::NonZeroU16::MIN)
    }
}

impl crate::default_some_one_element::DefaultSomeOneElement for StdDurationRangeLength {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}

impl crate::pg_range_length_sql::PgRangeLengthSql for StdDurationRangeLength {
    const USE_NUMERIC_DIFFERENCE: bool = false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_interval_encoding_preserves_day_boundaries_and_maximum_day_count() {
        let day = std::time::Duration::from_hours(24u64);
        let last_microsecond_of_day = std::time::Duration::from_micros(86_399_999_999u64);
        let microsecond = std::time::Duration::from_micros(1u64);
        let maximum_days =
            std::time::Duration::from_secs(u64::from(i32::MAX.unsigned_abs()) * 86_400u64);
        assert!(
            [
                (microsecond, 0i32, 1i64),
                (last_microsecond_of_day, 0i32, 86_399_999_999i64),
                (day, 1i32, 0i64),
                (day + microsecond, 1i32, 1i64),
                (maximum_days, i32::MAX, 0i64),
                (maximum_days + last_microsecond_of_day, i32::MAX, 86_399_999_999i64),
            ]
            .into_iter()
            .all(|(duration, days, microseconds)| {
                crate::std_duration_range_length::StdDurationRangeLength::try_from(duration)
                    .is_ok_and(|length| {
                        let mut actual = sqlx::postgres::PgArgumentBuffer::default();
                        let mut expected = sqlx::postgres::PgArgumentBuffer::default();
                        let reference = sqlx::postgres::types::PgInterval {
                            months: 0i32,
                            days,
                            microseconds,
                        };
                        matches!(
                            <crate::std_duration_range_length::StdDurationRangeLength as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&length, &mut actual),
                            Ok(sqlx::encode::IsNull::No)
                        ) && matches!(
                            <sqlx::postgres::types::PgInterval as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&reference, &mut expected),
                            Ok(sqlx::encode::IsNull::No)
                        ) && actual.as_slice() == expected.as_slice()
                            && serde_json::to_value(duration).is_ok_and(|value| {
                                serde_json::from_value::<crate::std_duration_range_length::StdDurationRangeLength>(value)
                                    .is_ok_and(|decoded| decoded == length)
                            })
                    })
            })
        );
        assert!(matches!(
            crate::std_duration_range_length::StdDurationRangeLength::try_from(maximum_days + day),
            Err(crate::std_duration_range_length_error::StdDurationRangeLengthError::ExceedsPostgresIntervalDays { .. })
        ));
        assert!(matches!(
            crate::std_duration_range_length::StdDurationRangeLength::try_from(
                maximum_days + day + std::time::Duration::from_nanos(1u64)
            ),
            Err(crate::std_duration_range_length_error::StdDurationRangeLengthError::SubmicrosecondPrecision { .. })
        ));
    }

    #[test]
    fn test_interval_length_validation() {
        assert!(matches!(
            crate::std_duration_range_length::StdDurationRangeLength::try_from(
                std::time::Duration::ZERO
            ),
            Err(crate::std_duration_range_length_error::StdDurationRangeLengthError::IsZero { .. })
        ));
        assert!(matches!(
            crate::std_duration_range_length::StdDurationRangeLength::try_from(std::time::Duration::from_nanos(1u64)),
            Err(crate::std_duration_range_length_error::StdDurationRangeLengthError::SubmicrosecondPrecision { .. })
        ));
        assert!(
            crate::std_duration_range_length::StdDurationRangeLength::try_from(
                std::time::Duration::from_micros(1u64)
            )
            .is_ok_and(|length| length
                == crate::std_duration_range_length::StdDurationRangeLength::default())
        );
        assert!(matches!(
            crate::std_duration_range_length::StdDurationRangeLength::try_from(
                std::time::Duration::from_secs(u64::MAX)
            ),
            Err(crate::std_duration_range_length_error::StdDurationRangeLengthError::ExceedsPostgresIntervalDays { .. })
        ));
    }

    #[test]
    fn test_interval_length_encodes_exact_day_and_microsecond() {
        let duration =
            std::time::Duration::from_hours(24u64) + std::time::Duration::from_micros(1u64);
        assert!(crate::std_duration_range_length::StdDurationRangeLength::try_from(duration)
            .is_ok_and(|length| {
                let mut actual = sqlx::postgres::PgArgumentBuffer::default();
                let mut expected = sqlx::postgres::PgArgumentBuffer::default();
                let reference = sqlx::postgres::types::PgInterval {
                    months: 0i32,
                    days: 1i32,
                    microseconds: 1i64,
                };
                matches!(
                    <crate::std_duration_range_length::StdDurationRangeLength as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&length, &mut actual),
                    Ok(sqlx::encode::IsNull::No)
                ) && matches!(
                    <sqlx::postgres::types::PgInterval as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&reference, &mut expected),
                    Ok(sqlx::encode::IsNull::No)
                ) && actual.as_slice() == expected.as_slice()
            }));
    }

    #[test]
    fn test_interval_length_deserialization_uses_validated_conversion() {
        let valid = serde_json::to_value(std::time::Duration::from_micros(1u64));
        assert!(valid.is_ok_and(|value| {
            serde_json::from_value::<crate::std_duration_range_length::StdDurationRangeLength>(
                value,
            )
            .is_ok_and(|length| {
                length == crate::std_duration_range_length::StdDurationRangeLength::default()
            })
        }));
        let zero = serde_json::to_value(std::time::Duration::ZERO);
        assert!(zero.is_ok_and(|value| {
            serde_json::from_value::<crate::std_duration_range_length::StdDurationRangeLength>(
                value,
            )
            .is_err()
        }));
    }

    #[test]
    fn test_interval_length_schema_preserves_required_integer_fields() {
        let schema = <crate::std_duration_range_length::StdDurationRangeLength as utoipa::PartialSchema>::schema();
        assert!(matches!(
            &schema,
            utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(_))
        ));
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema
        else {
            return;
        };
        assert_eq!(object.properties.len(), 2usize);
        assert!(
            object
                .required
                .iter()
                .map(String::as_str)
                .eq([constants_str::SECS, constants_str::NANOS])
        );
        assert!([constants_str::SECS, constants_str::NANOS].into_iter().all(|name| {
            matches!(object.properties.get(name), Some(utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(property))) if property.schema_type == utoipa::openapi::schema::Type::Integer.into())
        }));
    }

    #[test]
    fn test_interval_length_type_metadata_and_one_element_default_preserve_contracts() {
        let type_info = <crate::std_duration_range_length::StdDurationRangeLength as sqlx::Type<
            sqlx::Postgres,
        >>::type_info();
        assert_eq!(
            type_info,
            <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
        );
        assert!(
            <crate::std_duration_range_length::StdDurationRangeLength as sqlx::Type<
                sqlx::Postgres,
            >>::compatible(&type_info)
        );
        assert!(
            !<crate::std_duration_range_length::StdDurationRangeLength as sqlx::Type<
                sqlx::Postgres,
            >>::compatible(&<i64 as sqlx::Type<sqlx::Postgres>>::type_info())
        );
        assert_eq!(<crate::std_duration_range_length::StdDurationRangeLength as crate::default_some_one_element::DefaultSomeOneElement>::default_some_one_element(), crate::std_duration_range_length::StdDurationRangeLength::default());
    }
}
