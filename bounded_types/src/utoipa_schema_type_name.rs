#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct UtoipaSchemaTypeName(crate::bounded_string::BoundedString);

impl UtoipaSchemaTypeName {
    #[must_use]
    pub fn for_type<Item>() -> Self {
        let item_type_name = std::any::type_name::<Item>();
        let mut encoded = String::with_capacity(item_type_name.len().saturating_mul(2usize));
        let hexadecimal_digits = b"0123456789abcdef";
        item_type_name.bytes().for_each(|byte| {
            encoded.push(char::from(
                hexadecimal_digits
                    .get(usize::from(byte >> 4u8))
                    .copied()
                    .unwrap_or_default(),
            ));
            encoded.push(char::from(
                hexadecimal_digits
                    .get(usize::from(byte & 15u8))
                    .copied()
                    .unwrap_or_default(),
            ));
        });
        Self::from(crate::bounded_string::BoundedString::from_truncated(
            encoded,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_schema_type_name_encoding_preserves_complete_native_type_name() {
        let assert_encoding =
            |native_type_name: &str, utoipa_schema_type_name: super::UtoipaSchemaTypeName| {
                let encoded = utoipa_schema_type_name.into_inner();
                assert_eq!(encoded.len(), native_type_name.len().saturating_mul(2));
                assert!(encoded.bytes().all(|byte| !byte.is_ascii_uppercase()));
                assert!(
                    encoded
                        .as_bytes()
                        .as_chunks::<2>()
                        .0
                        .iter()
                        .zip(native_type_name.bytes())
                        .all(|(encoded_byte, native_byte)| {
                            std::str::from_utf8(encoded_byte)
                                .is_ok_and(|text| u8::from_str_radix(text, 16) == Ok(native_byte))
                        })
                );
            };
        assert_encoding(
            std::any::type_name::<u8>(),
            super::UtoipaSchemaTypeName::for_type::<u8>(),
        );
        assert_encoding(
            std::any::type_name::<
                Option<Result<&str, crate::bounded_string_error::BoundedStringError>>,
            >(),
            super::UtoipaSchemaTypeName::for_type::<
                Option<Result<&str, crate::bounded_string_error::BoundedStringError>>,
            >(),
        );
    }

    #[test]
    fn test_schema_type_names_are_distinct_and_component_safe() {
        let first = super::UtoipaSchemaTypeName::for_type::<u8>().into_inner();
        let second = super::UtoipaSchemaTypeName::for_type::<u16>().into_inner();
        assert_ne!(first, second);
        assert!(first.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert!(second.bytes().all(|byte| byte.is_ascii_hexdigit()));
    }
}
