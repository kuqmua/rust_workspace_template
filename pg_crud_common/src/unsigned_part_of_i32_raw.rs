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
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[serde(from = "i32")]
pub struct UnsignedPartOfI32Raw(i32);

impl to_err_string::to_err_string::ToErrString for UnsignedPartOfI32Raw {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_raw_integer_diagnostic_text_preserves_full_signed_range() {
        assert!(
            [i32::MIN, -1i32, 0i32, 1i32, i32::MAX]
                .into_iter()
                .all(|integer| {
                    let unsigned_part_of_i32_raw =
                        crate::unsigned_part_of_i32_raw::UnsignedPartOfI32Raw::from(integer);
                    unsigned_part_of_i32_raw.get() == integer
                        && unsigned_part_of_i32_raw.to_string() == integer.to_string()
                        && to_err_string::to_err_string::ToErrString::to_err_string(
                            &unsigned_part_of_i32_raw,
                        )
                        .as_ref()
                            == integer.to_string()
                        && serde_json::to_value(unsigned_part_of_i32_raw)
                            .is_ok_and(|json| json == serde_json::json!(integer))
                        && serde_json::from_value::<
                            crate::unsigned_part_of_i32_raw::UnsignedPartOfI32Raw,
                        >(serde_json::json!(integer))
                        .is_ok_and(|decoded| decoded == unsigned_part_of_i32_raw)
                })
        );
    }
}
