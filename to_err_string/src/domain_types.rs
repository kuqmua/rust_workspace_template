to_err_string_macros::impl_to_err_string_with!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, f32, f64, bool, char => |v| v.to_string());
#[cfg(not(target_arch = "wasm32"))]
to_err_string_macros::impl_to_err_string_with!(reqwest::header::HeaderMap, http_body::SizeHint => |v| format!("{v:#?}"));
to_err_string_macros::impl_to_err_string_with!(
    usize,
    std::io::Error,
    serde_json::Error
    => |v| v.to_string()
);
#[cfg(not(target_arch = "wasm32"))]
to_err_string_macros::impl_to_err_string_with!(
    http::header::ToStrError,
    axum::Error,
    time::error::ComponentRange,
    sqlx::types::uuid::Error,
    sqlx::Error,
    reqwest::Error,
    reqwest::StatusCode,
    axum::extract::rejection::JsonDataError,
    sqlx::migrate::MigrateError,
    axum::extract::rejection::JsonSyntaxError,
    axum::extract::rejection::JsonRejection,
    sqlx::types::chrono::NaiveTime,
    sqlx::types::chrono::NaiveDate,
    sqlx::types::chrono::NaiveDateTime,
    sqlx::types::time::Time,
    sqlx::types::time::PrimitiveDateTime,
    sqlx::types::Decimal,
    sqlx::types::BigDecimal
    => |v| v.to_string()
);
to_err_string_macros::impl_to_err_string_as_ref_str!(String, str, std::borrow::Cow<'_, str>);
to_err_string_macros::impl_to_err_string_const!(
    tracing::dispatcher::SetGlobalDefaultError => constants_str::integration_fixtures::TRACING_PATH_DISPATCHER_PATH_SETGLOBALDEFAULTERROR,
    tracing::log::SetLoggerError => constants_str::integration_fixtures::TRACING_PATH_LOG_PATH_TRACING_PATH_LOG_PATH_SETLOGGERERROR,
);
#[cfg(test)]
mod tests {
    fn assert_to_err_string(v: impl crate::to_err_string::ToErrString, exp: &str) {
        assert_eq!(v.to_err_string().as_ref(), exp);
    }
    #[test]
    fn to_err_string_for_primitives_and_options() {
        assert_to_err_string(42i32, constants_str::catalog::VALUE_42);
        assert_to_err_string(42i128, constants_str::catalog::VALUE_42);
        assert_to_err_string(42isize, constants_str::catalog::VALUE_42);
        assert_to_err_string(42u128, constants_str::catalog::VALUE_42);
        assert_to_err_string(Some(7u8), constants_str::catalog::SOME_7);
        assert_to_err_string(None::<u16>, constants_str::catalog::NONE);
        assert_to_err_string(true, constants_str::catalog::TRUE);
        assert_to_err_string('x', constants_str::catalog::X);
        assert_to_err_string(
            Some(String::from(constants_str::catalog::ABC_ALT_3)),
            constants_str::catalog::SOME_ABC,
        );
    }
    #[test]
    fn to_err_string_for_strings_and_str_refs() {
        let owned = String::from(constants_str::catalog::ABC_ALT_3);
        let borrowed = constants_str::catalog::XYZ;
        assert_to_err_string(owned, constants_str::catalog::ABC_ALT_3);
        assert_to_err_string(borrowed, constants_str::catalog::XYZ);
        assert_to_err_string(
            std::borrow::Cow::Borrowed(constants_str::catalog::QWE),
            constants_str::catalog::QWE,
        );
        assert_to_err_string(
            std::borrow::Cow::<'_, str>::Owned(String::from(constants_str::catalog::RTY)),
            constants_str::catalog::RTY,
        );
    }
    #[test]
    fn to_err_string_for_result_values() {
        assert_to_err_string(Result::<u8, u16>::Ok(5), constants_str::catalog::OK_5);
        assert_to_err_string(
            Result::<u8, &'static str>::Err(constants_str::catalog::CONFIG_TRACING_ERROR),
            constants_str::catalog::ERR_ERROR,
        );
    }
    #[test]
    #[cfg_attr(
        miri,
        ignore = "oversized JSON serialization is covered natively and is prohibitively slow under interpretation"
    )]
    fn error_text_owns_the_shared_length_invariant() {
        let valid =
            crate::error_text::ErrorText::try_from(String::from(constants_str::catalog::ERROR))
                .expect("11a745a8 error_text_owns_the_shared_length_invariant invariant must hold");
        assert_eq!(valid.as_ref(), constants_str::catalog::ERROR);

        let oversized = constants_str::catalog::X.repeat(
            crate::error_text_max_len::ERROR_TEXT_MAX_LEN.saturating_add(constants_usize::ONE),
        );
        let _conversion_error = crate::error_text::ErrorText::try_from(oversized.clone())
            .expect_err(constants_str::test_fixtures::VALUE_DFA2D703);
        let serialized = serde_json::to_string(&oversized)
            .expect("fe92c1a6 error_text_owns_the_shared_length_invariant invariant must hold");
        let _deserialization_error =
            serde_json::from_str::<crate::error_text::ErrorText>(serialized.as_str())
                .expect_err(constants_str::test_fixtures::VALUE_2377E790);
    }
}
