const COMMIT_HEADER_NAME: axum::http::HeaderName = axum::http::HeaderName::from_static("commit");
const NO_COMMIT_HEADER_MSG: &str = "no_commit_header";
const COMMIT_NOT_EQ_MSG: &str =
    "different project commit provided, services must work only with eq project commits";
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(to_err_string_as_ref_str)]
pub struct CommitNotEqMessage(&'static str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(to_err_string_as_ref_str)]
pub struct CommitToUse(&'static str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(to_err_string_as_ref_str)]
pub struct NoCommitHeaderMessage(&'static str);
#[derive(Debug, newtype::Newtype)]
#[newtype(to_err_string)]
pub struct AxumCommitToStrConversionError(axum::http::header::ToStrError);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from)]
pub struct EnableApiGitCommitCheck(bool);
#[derive(Debug, thiserror::Error, location::Location, optml::Optml)]
pub enum CommitError {
    CommitNotEq {
        #[eo_to_err_string]
        commit_not_eq: CommitNotEqMessage,
        #[eo_to_err_string]
        commit_to_use: CommitToUse,
        location: location_lib::location::Location,
    },
    CommitToStrConversion {
        location: location_lib::location::Location,
        #[eo_to_err_string]
        commit_to_str_conversion: AxumCommitToStrConversionError,
    },
    NoCommitHeader {
        #[eo_to_err_string]
        no_commit_header: NoCommitHeaderMessage,
        location: location_lib::location::Location,
    },
}
impl crate::GetAxumHttpStatusCode for CommitError {
    const AXUM_HTTP_STATUS_CODE: crate::AxumHttpStatusCode = crate::AxumHttpStatusCode::BAD_REQUEST;
}
impl CommitError {
    #[allow(clippy::single_call_fn)] // keeps mismatch error construction reusable and explicit
    fn commit_not_eq(commit_to_use: CommitToUse) -> Self {
        Self::CommitNotEq {
            commit_not_eq: CommitNotEqMessage(COMMIT_NOT_EQ_MSG),
            commit_to_use,
            location: location_macros::location!(),
        }
    }
    #[allow(clippy::single_call_fn)] // keeps header to-str conversion error construction reusable
    fn commit_to_str_conversion(commit_to_str_conversion: AxumCommitToStrConversionError) -> Self {
        Self::CommitToStrConversion {
            commit_to_str_conversion,
            location: location_macros::location!(),
        }
    }
    #[allow(clippy::single_call_fn)] // keeps missing-commit-header error construction reusable
    fn no_commit_header() -> Self {
        Self::NoCommitHeader {
            no_commit_header: NoCommitHeaderMessage(NO_COMMIT_HEADER_MSG),
            location: location_macros::location!(),
        }
    }
}
#[allow(clippy::single_call_fn)] // separates commit-value validation from header parsing for reuse and focused tests
fn validate_commit_header_value(
    commit: crate::hdr_val::HeaderStrRef<'_>,
) -> Result<(), CommitError> {
    git_info::validate_project_commit(commit.0)
        .map_err(|error| {
            CommitToUse(<&'static str>::from(
                git_info::ProjectGitCommitLinkRef::from(error),
            ))
        })
        .map_err(CommitError::commit_not_eq)
}
#[allow(clippy::single_call_fn)] // shared extractor keeps commit-header parsing reusable across commit-check entry points
fn read_commit_header_str(
    headers: crate::hdr_val::AxumHeadersRef<'_>,
) -> Result<crate::hdr_val::HeaderStrRef<'_>, CommitError> {
    crate::hdr_val::get_required_header_str(
        headers,
        COMMIT_HEADER_NAME,
        CommitError::no_commit_header,
        |error| CommitError::commit_to_str_conversion(AxumCommitToStrConversionError(error)),
    )
}
#[allow(clippy::single_call_fn)] // reusable validator keeps check_commit focused on feature-toggle behavior
fn validate_commit_header(headers: crate::hdr_val::AxumHeadersRef<'_>) -> Result<(), CommitError> {
    validate_commit_header_value(read_commit_header_str(headers)?)
}
pub fn check_commit(
    enable_api_git_commit_check: EnableApiGitCommitCheck,
    headers: crate::hdr_val::AxumHeadersRef<'_>,
) -> Result<(), CommitError> {
    if !enable_api_git_commit_check.0 {
        return Ok(());
    }
    validate_commit_header(headers)
}
#[cfg(test)]
mod tests {
    const WRONG_COMMIT: &str = "deadbeef";
    fn check_commit_enabled(headers: &axum::http::HeaderMap) -> Result<(), super::CommitError> {
        super::check_commit(true.into(), crate::hdr_val::AxumHeadersRef::from(headers))
    }
    fn mk_headers_with_commit_header_value<ValueTy>(
        value: ValueTy,
    ) -> crate::test_hlp::AxumTestHeaders
    where
        ValueTy: Into<crate::test_hlp::AxumTestHeaderValue>,
    {
        crate::test_hlp::mk_headers_with_entry(super::COMMIT_HEADER_NAME, value)
    }
    fn mk_headers_with_commit(commit: &str) -> crate::test_hlp::AxumTestHeaders {
        mk_headers_with_commit_header_value(
            axum::http::HeaderValue::from_str(commit).expect("9f2db59c"),
        )
    }
    fn mk_headers_with_wrong_commit() -> crate::test_hlp::AxumTestHeaders {
        mk_headers_with_commit(WRONG_COMMIT)
    }
    fn mk_headers_with_project_commit() -> crate::test_hlp::AxumTestHeaders {
        mk_headers_with_commit(git_info::PROJECT_GIT_INFO.commit.as_ref())
    }
    fn mk_headers_with_non_utf8_commit() -> crate::test_hlp::AxumTestHeaders {
        mk_headers_with_commit_header_value(crate::test_hlp::non_utf8_header_value())
    }
    fn check_commit_ok(
        enable_api_git_commit_check: bool,
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
    ) {
        crate::test_hlp::expect_ok(
            super::check_commit(
                enable_api_git_commit_check.into(),
                crate::hdr_val::AxumHeadersRef::from(headers),
            ),
            exp_id,
        );
    }
    fn check_commit_enabled_ok(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        check_commit_ok(true, headers, exp_id);
    }
    fn check_commit_bad_request(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        crate::test_hlp::assert_err_status_code_only(
            check_commit_enabled(headers),
            exp_id,
            crate::AxumHttpStatusCode::BAD_REQUEST,
        );
    }
    #[allow(clippy::single_call_fn)] // shared extractor keeps NoCommitHeader variant matching reusable across tests
    fn no_commit_header_message(v: &super::CommitError) -> Option<&'static str> {
        match v {
            super::CommitError::NoCommitHeader {
                no_commit_header, ..
            } => Some(no_commit_header.0),
            super::CommitError::CommitNotEq { .. }
            | super::CommitError::CommitToStrConversion { .. } => None,
        }
    }
    #[allow(clippy::single_call_fn)] // shared extractor keeps CommitToStrConversion checks concise across tests
    fn is_commit_to_str_conversion(v: &super::CommitError) -> Option<()> {
        match v {
            super::CommitError::CommitToStrConversion { .. } => Some(()),
            super::CommitError::CommitNotEq { .. } | super::CommitError::NoCommitHeader { .. } => {
                None
            }
        }
    }
    #[allow(clippy::single_call_fn)] // shared extractor centralizes CommitNotEq fields used by multiple assertions
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
    #[allow(clippy::single_call_fn)] // shared assertion keeps NoCommitHeader message checks coupled with variant extraction across tests
    fn assert_no_commit_header_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        let no_commit_header =
            expect_check_commit_err_variant(headers, exp_id, no_commit_header_message);
        assert_eq!(no_commit_header, super::NO_COMMIT_HEADER_MSG);
    }
    fn expect_commit_to_str_conversion_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        expect_check_commit_err_variant(headers, exp_id, is_commit_to_str_conversion);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps CommitNotEq expectation consistent across mismatch tests
    fn assert_commit_not_eq_fields(
        fields: (&'static str, &'static str),
        exp_commit_not_eq: &'static str,
        exp_commit_to_use: &'static str,
    ) {
        let (commit_not_eq, commit_to_use) = fields;
        assert_eq!(commit_not_eq, exp_commit_not_eq);
        assert_eq!(commit_to_use, exp_commit_to_use);
    }
    #[allow(clippy::single_call_fn)] // shared wrapper keeps wrong-commit assertions concise and stable across tests
    fn assert_wrong_commit_fields(fields: (&'static str, &'static str)) {
        assert_commit_not_eq_fields(
            fields,
            super::COMMIT_NOT_EQ_MSG,
            <&'static str>::from(git_info::project_git_commit_link_ref()),
        );
    }
    #[allow(clippy::single_call_fn)] // shared helper keeps wrong-commit check+assert flow reusable across mismatch tests
    fn assert_wrong_commit_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        let fields = expect_check_commit_err_variant(headers, exp_id, commit_not_eq_fields);
        assert_wrong_commit_fields(fields);
    }
    #[allow(clippy::single_call_fn)] // shared assertion wrapper keeps commit-enabled error mapping reusable across variant-specific helpers
    fn expect_check_commit_err_variant<R>(
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&super::CommitError) -> Option<R>,
    ) -> R {
        crate::test_hlp::expect_err_variant_ref_with_status(
            check_commit_enabled(headers),
            exp_id,
            Some(crate::AxumHttpStatusCode::BAD_REQUEST),
            map,
        )
    }
    #[allow(clippy::single_call_fn)] // shared wrapper keeps get_commit_header_str error-variant assertions concise and consistent
    fn expect_get_commit_header_str_err_variant<R>(
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&super::CommitError) -> Option<R>,
    ) -> R {
        crate::test_hlp::expect_err_variant_ref_with_status(
            super::read_commit_header_str(crate::hdr_val::AxumHeadersRef::from(headers)),
            exp_id,
            None,
            map,
        )
    }
    #[test]
    fn check_commit_is_skipped_when_validation_is_disabled() {
        let headers = axum::http::HeaderMap::new();
        check_commit_ok(false, &headers, "f4cab210");
    }
    #[test]
    fn check_commit_skip_mode_ignores_non_utf8_commit_header() {
        let headers = mk_headers_with_non_utf8_commit();
        check_commit_ok(false, &headers, "2f2a7b69");
    }
    #[test]
    fn check_commit_returns_no_header_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_no_commit_header_err(&headers, "c89f19a5");
    }
    #[test]
    fn check_commit_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&headers, "7b9ac2e3");
    }
    #[test]
    fn get_commit_header_str_returns_header_value_when_present() {
        let headers = mk_headers_with_project_commit();
        crate::test_hlp::assert_ok_eq(
            super::read_commit_header_str(crate::hdr_val::AxumHeadersRef::from(&headers))
                .map(|v| v.0),
            "e1d07f53",
            &git_info::PROJECT_GIT_INFO.commit.as_ref(),
        );
    }
    #[test]
    fn validate_commit_header_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = crate::test_hlp::expect_error_variant_ref(
            super::validate_commit_header(crate::hdr_val::AxumHeadersRef::from(&headers)),
            "31ea9a57",
            no_commit_header_message,
        );
        assert_eq!(no_commit_header, super::NO_COMMIT_HEADER_MSG);
    }
    #[test]
    fn validate_commit_header_accepts_project_commit() {
        let headers = mk_headers_with_project_commit();
        crate::test_hlp::expect_ok(
            super::validate_commit_header(crate::hdr_val::AxumHeadersRef::from(&headers)),
            "4d60c385",
        );
    }
    #[test]
    fn get_commit_header_str_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = expect_get_commit_header_str_err_variant(
            &headers,
            "72e4a18d",
            no_commit_header_message,
        );
        assert_eq!(no_commit_header, super::NO_COMMIT_HEADER_MSG);
    }
    #[test]
    fn get_commit_header_str_returns_error_for_non_utf8_header() {
        let headers = mk_headers_with_non_utf8_commit();
        expect_get_commit_header_str_err_variant(&headers, "6b4a128f", is_commit_to_str_conversion);
    }
    #[test]
    fn check_commit_returns_mismatch_error_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        assert_wrong_commit_err(&headers, "14f304d8");
    }
    #[test]
    fn validate_commit_header_value_returns_mismatch_for_wrong_commit() {
        let fields = crate::test_hlp::expect_error_variant_ref(
            super::validate_commit_header_value(crate::hdr_val::HeaderStrRef(WRONG_COMMIT)),
            "6804382f",
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn validate_commit_header_value_accepts_project_commit() {
        crate::test_hlp::expect_ok(
            super::validate_commit_header_value(crate::hdr_val::HeaderStrRef(
                git_info::PROJECT_GIT_INFO.commit.as_ref(),
            )),
            "5ef927d2",
        );
    }
    #[test]
    fn check_commit_returns_expected_commit_link_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        let fields = crate::test_hlp::expect_error_variant_ref(
            check_commit_enabled(&headers),
            "3db98d20",
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn check_commit_treats_empty_commit_as_mismatch() {
        let headers = mk_headers_with_commit("");
        assert_wrong_commit_err(&headers, "491ef4d6");
    }
    #[test]
    fn check_commit_accepts_header_name_with_different_case() {
        let mut headers = mk_headers_with_project_commit();
        crate::test_hlp::replace_header_name(
            &mut headers,
            super::COMMIT_HEADER_NAME,
            "Commit",
            "12653c9a",
        );
        check_commit_enabled_ok(&headers, "bb6c239e");
    }
    #[test]
    fn check_commit_returns_ok_for_matching_commit() {
        let headers = mk_headers_with_project_commit();
        check_commit_enabled_ok(&headers, "c95e27d1");
    }
    #[test]
    fn project_commit_is_recognized_by_git_info_helper() {
        assert!(git_info::is_project_commit(
            git_info::PROJECT_GIT_INFO.commit
        ));
    }
    #[test]
    fn non_project_commit_is_rejected_by_git_info_helper() {
        assert!(!git_info::is_project_commit(WRONG_COMMIT));
    }
    #[test]
    fn commit_errors_have_bad_request_status_code() {
        let headers = axum::http::HeaderMap::new();
        assert_no_commit_header_err(&headers, "76314db5");
        check_commit_bad_request(&headers, "f39bdcc6");
        let non_utf8_headers = mk_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&non_utf8_headers, "e1c2d84a");
        check_commit_bad_request(&non_utf8_headers, "2e86aa15");
        let mismatch_headers = mk_headers_with_wrong_commit();
        check_commit_bad_request(&mismatch_headers, "1cabe205");
    }
}
