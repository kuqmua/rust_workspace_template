#[derive(
    proc_macro_getters::Getters,
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
)]
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
                location: proc_macro_location_bang::location!(),
            })
        }
    }
}

impl TryFrom<std::num::NonZeroI32> for UnsignedPartOfI32 {
    type Error = crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error;

    fn try_from(value: std::num::NonZeroI32) -> Result<Self, Self::Error> {
        Self::try_from(value.get())
    }
}

impl From<crate::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32> for UnsignedPartOfI32 {
    fn from(value: crate::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32) -> Self {
        Self(value.get_inner().get())
    }
}

impl to_err_string::to_err_string::ToErrString for UnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.0.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}

impl sqlx::Type<sqlx::Postgres> for UnsignedPartOfI32 {
    fn compatible(type_info: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <i32 as sqlx::Type<sqlx::Postgres>>::compatible(type_info)
    }

    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for UnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        pg_argument_buffer: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, pg_argument_buffer)
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
    fn test_unsigned_integer_json_and_database_encoding_preserve_boundaries() {
        assert!(
            [(0i32, [0u8; 4]), (i32::MAX, [0x7fu8, 0xff, 0xff, 0xff])]
                .into_iter()
                .all(|(integer, encoded_bytes)| {
                    crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(integer).is_ok_and(
                        |unsigned_part_of_i32| {
                            let json = serde_json::json!(integer);
                            let serialized = serde_json::to_value(unsigned_part_of_i32);
                            let deserialized = serde_json::from_value::<
                                crate::unsigned_part_of_i32::UnsignedPartOfI32,
                            >(json.clone());
                            let mut buffer = sqlx::postgres::PgArgumentBuffer::default();
                            let encoded =
                                <crate::unsigned_part_of_i32::UnsignedPartOfI32 as sqlx::Encode<
                                    sqlx::Postgres,
                                >>::encode_by_ref(
                                    &unsigned_part_of_i32, &mut buffer
                                );
                            serialized.is_ok_and(|value| value == json)
                                && deserialized.is_ok_and(|value| value == unsigned_part_of_i32)
                                && matches!(encoded, Ok(sqlx::encode::IsNull::No))
                                && buffer.as_slice() == encoded_bytes.as_slice()
                                && to_err_string::to_err_string::ToErrString::to_err_string(
                                    &unsigned_part_of_i32,
                                )
                                .as_ref()
                                .parse::<i32>()
                                .is_ok_and(|value| value == integer)
                        },
                    )
                })
        );
        let integer_type = <i32 as sqlx::Type<sqlx::Postgres>>::type_info();
        assert_eq!(
            <crate::unsigned_part_of_i32::UnsignedPartOfI32 as
                sqlx::Type<sqlx::Postgres>>::type_info(),
            integer_type,
        );
        assert!(<crate::unsigned_part_of_i32::UnsignedPartOfI32 as
            sqlx::Type<sqlx::Postgres>>::compatible(&integer_type));
        assert!(!<crate::unsigned_part_of_i32::UnsignedPartOfI32 as
            sqlx::Type<sqlx::Postgres>>::compatible(
                &<String as sqlx::Type<sqlx::Postgres>>::type_info(),
            ));
    }

    #[test]
    fn test_unsigned_integer_rejections_preserve_raw_values_and_validate_json() {
        assert!([-1i32, i32::MIN].into_iter().all(|integer| {
            matches!(
                crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(integer),
                Err(crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error::LessThanZero {
                    v: unsigned_part_of_i32_raw,
                    ..
                }) if unsigned_part_of_i32_raw == crate::unsigned_part_of_i32_raw::UnsignedPartOfI32Raw::from(integer)
            ) && serde_json::from_value::<crate::unsigned_part_of_i32::UnsignedPartOfI32>(
                serde_json::json!(integer),
            ).is_err()
        }));
    }

    #[test]
    fn test_unsigned_database_value_rejects_negative_input() {
        assert!(matches!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(-1i32),
            Err(crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error::LessThanZero { .. })
        ));
        assert_eq!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(7i32)
                .expect(constants_str::DIAGNOSTIC_EA8C2D71),
            crate::unsigned_part_of_i32::UnsignedPartOfI32::from(7u16)
        );
        assert!(matches!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(
                std::num::NonZeroI32::new(7i32).expect(constants_str::DIAGNOSTIC_DD53FC4D),
            ),
            Ok(unsigned) if unsigned == crate::unsigned_part_of_i32::UnsignedPartOfI32::from(7u16)
        ));
        assert!(matches!(
            crate::unsigned_part_of_i32::UnsignedPartOfI32::try_from(
                std::num::NonZeroI32::MIN
            ),
            Err(crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error::LessThanZero { .. })
        ));
    }
}
