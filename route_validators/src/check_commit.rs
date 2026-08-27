#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "axum_commit_to_str_conversion_error.rs"]
mod axum_commit_to_str_conversion_error;
#[path = "check_commit/check_commit.rs"]
mod check_commit;
#[path = "commit_error.rs"]
mod commit_error;
#[path = "commit_header_name.rs"]
mod commit_header_name;
#[path = "commit_not_eq_message.rs"]
mod commit_not_eq_message;
#[path = "commit_to_use.rs"]
mod commit_to_use;
#[path = "enable_api_git_commit_check.rs"]
mod enable_api_git_commit_check;
#[path = "no_commit_header_message.rs"]
mod no_commit_header_message;
#[path = "read_commit_header_str.rs"]
mod read_commit_header_str;
#[path = "validate_commit_header.rs"]
mod validate_commit_header;
#[path = "validate_commit_header_value.rs"]
mod validate_commit_header_value;

pub use axum_commit_to_str_conversion_error::AxumCommitToStrConversionError;
pub use check_commit::check_commit;
pub use commit_error::CommitError;
pub use commit_not_eq_message::CommitNotEqMessage;
pub use commit_to_use::CommitToUse;
pub use enable_api_git_commit_check::EnableApiGitCommitCheck;
pub use no_commit_header_message::NoCommitHeaderMessage;
pub(super) use read_commit_header_str::read_commit_header_str;
pub(super) use validate_commit_header::validate_commit_header;
pub(super) use validate_commit_header_value::validate_commit_header_value;
#[cfg(test)]
mod tests {
    fn check_commit_enabled(headers: &axum::http::HeaderMap) -> Result<(), super::CommitError> {
        super::check_commit(
            true.into(),
            crate::domain_types::header_value::AxumHeadersRef::from(headers),
        )
    }
    fn make_headers_with_commit_header_value<ValueTy>(
        value: ValueTy,
    ) -> crate::domain_types::test_helper::AxumTestHeaders
    where
        ValueTy: Into<crate::domain_types::test_helper::AxumTestHeaderValue>,
    {
        crate::domain_types::test_helper::make_headers_with_entry(
            super::commit_header_name::COMMIT_HEADER_NAME,
            value,
        )
    }
    fn make_headers_with_commit(commit: &str) -> crate::domain_types::test_helper::AxumTestHeaders {
        make_headers_with_commit_header_value(
            axum::http::HeaderValue::from_str(commit)
                .expect("9f2db59c make_headers_with_commit invariant must hold"),
        )
    }
    fn make_headers_with_wrong_commit() -> crate::domain_types::test_helper::AxumTestHeaders {
        make_headers_with_commit(constants_str::TEST_VALUES_WRONG_COMMIT)
    }
    fn make_headers_with_project_commit() -> crate::domain_types::test_helper::AxumTestHeaders {
        make_headers_with_commit(git_info::domain_types::project_git_info().commit().as_ref())
    }
    fn make_headers_with_non_utf8_commit() -> crate::domain_types::test_helper::AxumTestHeaders {
        make_headers_with_commit_header_value(
            crate::domain_types::test_helper::non_utf8_header_value(),
        )
    }
    fn check_commit_ok(
        enable_api_git_commit_check: bool,
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
    ) {
        crate::domain_types::test_helper::expect_ok(
            super::check_commit(
                enable_api_git_commit_check.into(),
                crate::domain_types::header_value::AxumHeadersRef::from(headers),
            ),
            exp_id,
        );
    }
    fn check_commit_enabled_ok(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        check_commit_ok(true, headers, exp_id);
    }
    fn check_commit_bad_request(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        crate::domain_types::test_helper::assert_err_status_code_only(
            check_commit_enabled(headers),
            exp_id,
            crate::domain_types::AxumHttpStatusCode::bad_request(),
        );
    }
    fn no_commit_header_message(v: &super::CommitError) -> Option<&'static str> {
        match v {
            super::CommitError::NoCommitHeader {
                no_commit_header, ..
            } => Some(no_commit_header.0),
            super::CommitError::CommitNotEq { .. }
            | super::CommitError::CommitToStrConversion { .. } => None,
        }
    }
    fn is_commit_to_str_conversion(v: &super::CommitError) -> Option<()> {
        match v {
            super::CommitError::CommitToStrConversion { .. } => Some(()),
            super::CommitError::CommitNotEq { .. } | super::CommitError::NoCommitHeader { .. } => {
                None
            }
        }
    }
    fn commit_not_eq_fields(v: &super::CommitError) -> Option<(&'static str, &'static str)> {
        match v {
            super::CommitError::CommitNotEq {
                commit_not_eq,
                commit_to_use,
                ..
            } => Some((commit_not_eq.0, commit_to_use.0)),
            super::CommitError::CommitToStrConversion { .. }
            | super::CommitError::NoCommitHeader { .. } => None,
        }
    }
    fn assert_no_commit_header_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        let no_commit_header =
            expect_check_commit_err_variant(headers, exp_id, no_commit_header_message);
        assert_eq!(
            no_commit_header,
            constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
        );
    }
    fn expect_commit_to_str_conversion_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        expect_check_commit_err_variant(headers, exp_id, is_commit_to_str_conversion);
    }
    fn assert_wrong_commit_fields(fields: (&'static str, &'static str)) {
        let (commit_not_eq, commit_to_use) = fields;
        assert_eq!(
            commit_not_eq,
            constants_str::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
        );
        assert_eq!(
            commit_to_use,
            <&'static str>::from(git_info::domain_types::project_git_commit_link_ref()),
        );
    }
    fn assert_wrong_commit_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        let fields = expect_check_commit_err_variant(headers, exp_id, commit_not_eq_fields);
        assert_wrong_commit_fields(fields);
    }
    fn expect_check_commit_err_variant<R>(
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&super::CommitError) -> Option<R>,
    ) -> R {
        crate::domain_types::test_helper::expect_err_variant_ref_with_status(
            check_commit_enabled(headers),
            exp_id,
            Some(crate::domain_types::AxumHttpStatusCode::bad_request()),
            map,
        )
    }
    fn expect_get_commit_header_str_err_variant<R>(
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&super::CommitError) -> Option<R>,
    ) -> R {
        crate::domain_types::test_helper::expect_err_variant_ref_with_status(
            super::read_commit_header_str(crate::domain_types::header_value::AxumHeadersRef::from(
                headers,
            )),
            exp_id,
            None,
            map,
        )
    }
    #[test]
    fn check_commit_is_skipped_when_validation_is_disabled() {
        let headers = axum::http::HeaderMap::new();
        check_commit_ok(false, &headers, constants_str::F4CAB210);
    }
    #[test]
    fn check_commit_skip_mode_ignores_non_utf8_commit_header() {
        let headers = make_headers_with_non_utf8_commit();
        check_commit_ok(false, &headers, constants_str::VALUE_2F2A7B69);
    }
    #[test]
    fn check_commit_returns_no_header_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_no_commit_header_err(&headers, constants_str::C89F19A5);
    }
    #[test]
    fn check_commit_returns_to_str_error_for_non_utf8_header() {
        let headers = make_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&headers, constants_str::VALUE_7B9AC2E3);
    }
    #[test]
    fn commit_header_str_returns_header_value_when_present() {
        let headers = make_headers_with_project_commit();
        crate::domain_types::test_helper::assert_ok_eq(
            super::read_commit_header_str(crate::domain_types::header_value::AxumHeadersRef::from(
                &headers,
            ))
            .map(crate::domain_types::header_value::HeaderStrRef::get),
            constants_str::E1D07F53,
            &git_info::domain_types::project_git_info().commit().as_ref(),
        );
    }
    #[test]
    fn validate_commit_header_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = crate::domain_types::test_helper::expect_error_variant_ref(
            super::validate_commit_header(crate::domain_types::header_value::AxumHeadersRef::from(
                &headers,
            )),
            constants_str::VALUE_31EA9A57,
            no_commit_header_message,
        );
        assert_eq!(
            no_commit_header,
            constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
        );
    }
    #[test]
    fn validate_commit_header_accepts_project_commit() {
        let headers = make_headers_with_project_commit();
        crate::domain_types::test_helper::expect_ok(
            super::validate_commit_header(crate::domain_types::header_value::AxumHeadersRef::from(
                &headers,
            )),
            constants_str::VALUE_4D60C385,
        );
    }
    #[test]
    fn commit_header_str_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = expect_get_commit_header_str_err_variant(
            &headers,
            constants_str::VALUE_72E4A18D,
            no_commit_header_message,
        );
        assert_eq!(
            no_commit_header,
            constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
        );
    }
    #[test]
    fn commit_header_str_returns_error_for_non_utf8_header() {
        let headers = make_headers_with_non_utf8_commit();
        expect_get_commit_header_str_err_variant(
            &headers,
            constants_str::VALUE_6B4A128F,
            is_commit_to_str_conversion,
        );
    }
    #[test]
    fn check_commit_returns_mismatch_error_for_wrong_commit() {
        let headers = make_headers_with_wrong_commit();
        assert_wrong_commit_err(&headers, constants_str::VALUE_14F304D8);
    }
    #[test]
    fn validate_commit_header_value_returns_mismatch_for_wrong_commit() {
        let fields = crate::domain_types::test_helper::expect_error_variant_ref(
            super::validate_commit_header_value(
                crate::domain_types::header_value::HeaderStrRef::from(
                    constants_str::TEST_VALUES_WRONG_COMMIT,
                ),
            ),
            constants_str::VALUE_6804382F,
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn validate_commit_header_value_accepts_project_commit() {
        crate::domain_types::test_helper::expect_ok(
            super::validate_commit_header_value(
                crate::domain_types::header_value::HeaderStrRef::from(
                    git_info::domain_types::project_git_info().commit().as_ref(),
                ),
            ),
            constants_str::VALUE_5EF927D2,
        );
    }
    #[test]
    fn check_commit_returns_expected_commit_link_for_wrong_commit() {
        let headers = make_headers_with_wrong_commit();
        let fields = crate::domain_types::test_helper::expect_error_variant_ref(
            check_commit_enabled(&headers),
            constants_str::VALUE_3DB98D20,
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn check_commit_treats_empty_commit_as_mismatch() {
        let headers = make_headers_with_commit(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
        assert_wrong_commit_err(&headers, constants_str::VALUE_491EF4D6);
    }
    #[test]
    fn check_commit_accepts_header_name_with_different_case() {
        let mut headers = make_headers_with_project_commit();
        crate::domain_types::test_helper::replace_header_name(
            &mut headers,
            super::commit_header_name::COMMIT_HEADER_NAME,
            constants_str::COMMIT,
            constants_str::VALUE_12653C9A,
        );
        check_commit_enabled_ok(&headers, constants_str::BB6C239E);
    }
    #[test]
    fn check_commit_returns_ok_for_matching_commit() {
        let headers = make_headers_with_project_commit();
        check_commit_enabled_ok(&headers, constants_str::C95E27D1);
    }
    #[test]
    fn project_commit_is_recognized_by_git_info_helper() {
        assert!(git_info::domain_types::is_project_commit(
            git_info::domain_types::project_git_info().commit()
        ));
    }
    #[test]
    fn non_project_commit_is_rejected_by_git_info_helper() {
        assert!(!git_info::domain_types::is_project_commit(
            constants_str::TEST_VALUES_WRONG_COMMIT
        ));
    }
    #[test]
    fn commit_errors_have_bad_request_status_code() {
        let headers = axum::http::HeaderMap::new();
        assert_no_commit_header_err(&headers, constants_str::VALUE_76314DB5);
        check_commit_bad_request(&headers, constants_str::F39BDCC6);
        let non_utf8_headers = make_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&non_utf8_headers, constants_str::E1C2D84A);
        check_commit_bad_request(&non_utf8_headers, constants_str::VALUE_2E86AA15);
        let mismatch_headers = make_headers_with_wrong_commit();
        check_commit_bad_request(&mismatch_headers, constants_str::VALUE_1CABE205);
    }
}
