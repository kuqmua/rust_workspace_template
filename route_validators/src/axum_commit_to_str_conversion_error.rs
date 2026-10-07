#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_to_err_string::ToErrString,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AxumCommitToStrConversionError(axum::http::header::ToStrError);

#[cfg(test)]
mod tests {
    #[test]
    fn test_commit_header_conversion_error_preserves_native_diagnostic() {
        let header_value = crate::non_utf8_header_value::non_utf8_header_value();
        let conversion = header_value.to_str();
        assert!(conversion.is_err());
        let Err(native_error) = conversion else {
            return;
        };
        let expected = to_err_string::to_err_string::ToErrString::to_err_string(&native_error);
        let wrapped = super::AxumCommitToStrConversionError::from(native_error);
        assert_eq!(
            to_err_string::to_err_string::ToErrString::to_err_string(&wrapped),
            expected,
        );
    }
}
