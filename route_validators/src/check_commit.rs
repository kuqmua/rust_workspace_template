const COMMIT_HEADER_NAME: axum::http::HeaderName =
    axum::http::HeaderName::from_static(str_constants::ROUTE_VALIDATORS_COMMIT_HEADER_NAME);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::ToErrStringAsRefStr,
    newtype::FromInner,
)]
pub struct CommitNotEqMessage(&'static str);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::ToErrStringAsRefStr,
    newtype::FromInner,
)]
pub struct CommitToUse(&'static str);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::ToErrStringAsRefStr,
    newtype::FromInner,
)]
pub struct NoCommitHeaderMessage(&'static str);
#[derive(optml::Optml, Debug, newtype::ToErrString, newtype::FromInner)]
pub struct AxumCommitToStrConversionError(axum::http::header::ToStrError);
#[derive(optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
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
    fn get_axum_http_status_code(&self) -> crate::AxumHttpStatusCode {
        crate::AxumHttpStatusCode::bad_request()
    }
}
impl CommitError {
    #[allow(clippy::single_call_fn)] // keeps mismatch error construction reusable and explicit
    fn commit_not_eq(commit_to_use: CommitToUse) -> Self {
        Self::CommitNotEq {
            commit_not_eq: CommitNotEqMessage::from(
                str_constants::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
            ),
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
            no_commit_header: NoCommitHeaderMessage::from(
                str_constants::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG,
            ),
            location: location_macros::location!(),
        }
    }
}
#[allow(clippy::single_call_fn)] // separates commit-value validation from header parsing for reuse and focused tests
fn validate_commit_header_value(
    commit: crate::hdr_val::HeaderStrRef<'_>,
) -> Result<(), CommitError> {
    git_info::validate_project_commit(commit.as_ref())
        .map_err(|error| {
            CommitToUse::from(<&'static str>::from(
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
        |error| CommitError::commit_to_str_conversion(AxumCommitToStrConversionError::from(error)),
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
        mk_headers_with_commit(str_constants::TEST_VALUES_WRONG_COMMIT)
    }
    fn mk_headers_with_project_commit() -> crate::test_hlp::AxumTestHeaders {
        mk_headers_with_commit(git_info::project_git_info().commit().as_ref())
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
            crate::AxumHttpStatusCode::bad_request(),
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
            str_constants::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
        );
    }
    fn expect_commit_to_str_conversion_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        expect_check_commit_err_variant(headers, exp_id, is_commit_to_str_conversion);
    }
    fn assert_wrong_commit_fields(fields: (&'static str, &'static str)) {
        let (commit_not_eq, commit_to_use) = fields;
        assert_eq!(
            commit_not_eq,
            str_constants::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
        );
        assert_eq!(
            commit_to_use,
            <&'static str>::from(git_info::project_git_commit_link_ref()),
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
        crate::test_hlp::expect_err_variant_ref_with_status(
            check_commit_enabled(headers),
            exp_id,
            Some(crate::AxumHttpStatusCode::bad_request()),
            map,
        )
    }
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
        check_commit_ok(false, &headers, str_constants::F4CAB210);
    }
    #[test]
    fn check_commit_skip_mode_ignores_non_utf8_commit_header() {
        let headers = mk_headers_with_non_utf8_commit();
        check_commit_ok(false, &headers, str_constants::VALUE_2F2A7B69);
    }
    #[test]
    fn check_commit_returns_no_header_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        assert_no_commit_header_err(&headers, str_constants::C89F19A5);
    }
    #[test]
    fn check_commit_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&headers, str_constants::VALUE_7B9AC2E3);
    }
    #[test]
    fn get_commit_header_str_returns_header_value_when_present() {
        let headers = mk_headers_with_project_commit();
        crate::test_hlp::assert_ok_eq(
            super::read_commit_header_str(crate::hdr_val::AxumHeadersRef::from(&headers))
                .map(crate::hdr_val::HeaderStrRef::get),
            str_constants::E1D07F53,
            &git_info::project_git_info().commit().as_ref(),
        );
    }
    #[test]
    fn validate_commit_header_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = crate::test_hlp::expect_error_variant_ref(
            super::validate_commit_header(crate::hdr_val::AxumHeadersRef::from(&headers)),
            str_constants::VALUE_31EA9A57,
            no_commit_header_message,
        );
        assert_eq!(
            no_commit_header,
            str_constants::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
        );
    }
    #[test]
    fn validate_commit_header_accepts_project_commit() {
        let headers = mk_headers_with_project_commit();
        crate::test_hlp::expect_ok(
            super::validate_commit_header(crate::hdr_val::AxumHeadersRef::from(&headers)),
            str_constants::VALUE_4D60C385,
        );
    }
    #[test]
    fn get_commit_header_str_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = expect_get_commit_header_str_err_variant(
            &headers,
            str_constants::VALUE_72E4A18D,
            no_commit_header_message,
        );
        assert_eq!(
            no_commit_header,
            str_constants::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
        );
    }
    #[test]
    fn get_commit_header_str_returns_error_for_non_utf8_header() {
        let headers = mk_headers_with_non_utf8_commit();
        expect_get_commit_header_str_err_variant(
            &headers,
            str_constants::VALUE_6B4A128F,
            is_commit_to_str_conversion,
        );
    }
    #[test]
    fn check_commit_returns_mismatch_error_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        assert_wrong_commit_err(&headers, str_constants::VALUE_14F304D8);
    }
    #[test]
    fn validate_commit_header_value_returns_mismatch_for_wrong_commit() {
        let fields = crate::test_hlp::expect_error_variant_ref(
            super::validate_commit_header_value(crate::hdr_val::HeaderStrRef::from(
                str_constants::TEST_VALUES_WRONG_COMMIT,
            )),
            str_constants::VALUE_6804382F,
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn validate_commit_header_value_accepts_project_commit() {
        crate::test_hlp::expect_ok(
            super::validate_commit_header_value(crate::hdr_val::HeaderStrRef::from(
                git_info::project_git_info().commit().as_ref(),
            )),
            str_constants::VALUE_5EF927D2,
        );
    }
    #[test]
    fn check_commit_returns_expected_commit_link_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        let fields = crate::test_hlp::expect_error_variant_ref(
            check_commit_enabled(&headers),
            str_constants::VALUE_3DB98D20,
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn check_commit_treats_empty_commit_as_mismatch() {
        let headers = mk_headers_with_commit(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX);
        assert_wrong_commit_err(&headers, str_constants::VALUE_491EF4D6);
    }
    #[test]
    fn check_commit_accepts_header_name_with_different_case() {
        let mut headers = mk_headers_with_project_commit();
        crate::test_hlp::replace_header_name(
            &mut headers,
            super::COMMIT_HEADER_NAME,
            str_constants::COMMIT,
            str_constants::VALUE_12653C9A,
        );
        check_commit_enabled_ok(&headers, str_constants::BB6C239E);
    }
    #[test]
    fn check_commit_returns_ok_for_matching_commit() {
        let headers = mk_headers_with_project_commit();
        check_commit_enabled_ok(&headers, str_constants::C95E27D1);
    }
    #[test]
    fn project_commit_is_recognized_by_git_info_helper() {
        assert!(git_info::is_project_commit(
            git_info::project_git_info().commit()
        ));
    }
    #[test]
    fn non_project_commit_is_rejected_by_git_info_helper() {
        assert!(!git_info::is_project_commit(
            str_constants::TEST_VALUES_WRONG_COMMIT
        ));
    }
    #[test]
    fn commit_errors_have_bad_request_status_code() {
        let headers = axum::http::HeaderMap::new();
        assert_no_commit_header_err(&headers, str_constants::VALUE_76314DB5);
        check_commit_bad_request(&headers, str_constants::F39BDCC6);
        let non_utf8_headers = mk_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&non_utf8_headers, str_constants::E1C2D84A);
        check_commit_bad_request(&non_utf8_headers, str_constants::VALUE_2E86AA15);
        let mismatch_headers = mk_headers_with_wrong_commit();
        check_commit_bad_request(&mismatch_headers, str_constants::VALUE_1CABE205);
    }
}
