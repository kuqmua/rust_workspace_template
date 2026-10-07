#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_to_err_string::ToErrString,
)]
#[serde(from = "usize")]
pub struct BodySizeLimitBytes(usize);

impl BodySizeLimitBytes {
    pub(crate) const fn value(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_body_limit_deserialization_preserves_native_integer_bounds() {
        [
            constants_usize::ZERO,
            constants_usize::ONE,
            std::num::NonZeroUsize::MAX.get(),
        ]
        .into_iter()
        .fold((), |(), value| {
            let deserialized = <super::BodySizeLimitBytes as serde::Deserialize>::deserialize(
                serde::de::value::UsizeDeserializer::<serde::de::value::Error>::new(value),
            );
            assert!(deserialized.is_ok());
            let Ok(body_size_limit_bytes) = deserialized else {
                return;
            };
            assert_eq!(body_size_limit_bytes.value(), value);
            assert_eq!(
                body_size_limit_bytes,
                super::BodySizeLimitBytes::from(value)
            );
            assert_eq!(
                to_err_string::to_err_string::ToErrString::to_err_string(&body_size_limit_bytes)
                    .as_ref(),
                &value.to_string(),
            );
        });
    }
}
