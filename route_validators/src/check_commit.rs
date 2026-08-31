#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
pub fn check_commit(
    enable_api_git_commit_check: crate::enable_api_git_commit_check::EnableApiGitCommitCheck,
    headers: crate::axum_headers_ref::AxumHeadersRef<'_>,
) -> Result<(), crate::commit_error::CommitError> {
    if !enable_api_git_commit_check.is_enabled() {
        return Ok(());
    }
    let commit = headers
        .header(crate::commit_header_name::COMMIT_HEADER_NAME)
        .ok_or_else(|| crate::commit_error::CommitError::NoCommitHeader {
            no_commit_header: crate::no_commit_header_message::NoCommitHeaderMessage::from(
                constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG,
            ),
            location: location_macros::location!(),
        })?
        .to_str()
        .map_err(|error| crate::commit_error::CommitError::CommitToStrConversion {
            commit_to_str_conversion: crate::axum_commit_to_str_conversion_error::AxumCommitToStrConversionError::from(error),
            location: location_macros::location!(),
        })?;
    git_info::validate_project_commit::validate_project_commit(commit)
        .map_err(|error| {
            crate::commit_to_use::CommitToUse::from(<&'static str>::from(
                git_info::project_git_commit_link_ref::ProjectGitCommitLinkRef::from(error),
            ))
        })
        .map_err(
            |commit_to_use| crate::commit_error::CommitError::CommitNotEq {
                commit_not_eq: crate::commit_not_eq_message::CommitNotEqMessage::from(
                    constants_str::ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG,
                ),
                commit_to_use,
                location: location_macros::location!(),
            },
        )
}
#[cfg(test)]
mod tests {
    fn check_commit_enabled(
        headers: &axum::http::HeaderMap,
    ) -> Result<(), crate::commit_error::CommitError> {
        crate::check_commit::check_commit(
            true.into(),
            crate::axum_headers_ref::AxumHeadersRef::from(headers),
        )
    }
    fn make_headers_with_commit_header_value<ValueTy>(
        value: ValueTy,
    ) -> crate::axum_test_headers::AxumTestHeaders
    where
        ValueTy: Into<crate::axum_test_header_value::AxumTestHeaderValue>,
    {
        crate::make_headers_with_entry::make_headers_with_entry(
            crate::commit_header_name::COMMIT_HEADER_NAME,
            value,
        )
    }
    fn make_headers_with_commit(commit: &str) -> crate::axum_test_headers::AxumTestHeaders {
        make_headers_with_commit_header_value(
            axum::http::HeaderValue::from_str(commit)
                .expect("9f2db59c make_headers_with_commit invariant must hold"),
        )
    }
    fn make_headers_with_wrong_commit() -> crate::axum_test_headers::AxumTestHeaders {
        make_headers_with_commit(constants_str::TEST_VALUES_WRONG_COMMIT)
    }
    fn make_headers_with_project_commit() -> crate::axum_test_headers::AxumTestHeaders {
        make_headers_with_commit(
            git_info::project_git_info_value::project_git_info_value()
                .commit()
                .as_ref(),
        )
    }
    fn make_headers_with_non_utf8_commit() -> crate::axum_test_headers::AxumTestHeaders {
        make_headers_with_commit_header_value(crate::non_utf8_header_value::non_utf8_header_value())
    }
    fn check_commit_ok(
        enable_api_git_commit_check: bool,
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
    ) {
        crate::expect_ok::expect_ok(
            crate::check_commit::check_commit(
                enable_api_git_commit_check.into(),
                crate::axum_headers_ref::AxumHeadersRef::from(headers),
            ),
            exp_id,
        );
    }
    fn check_commit_enabled_ok(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        check_commit_ok(true, headers, exp_id);
    }
    fn check_commit_bad_request(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        crate::assert_err_status_code_only::assert_err_status_code_only(
            check_commit_enabled(headers),
            exp_id,
            crate::axum_http_status_code::AxumHttpStatusCode::bad_request(),
        );
    }
    fn no_commit_header_message(v: &crate::commit_error::CommitError) -> Option<&'static str> {
        match v {
            crate::commit_error::CommitError::NoCommitHeader {
                no_commit_header, ..
            } => Some(no_commit_header.as_str()),
            crate::commit_error::CommitError::CommitNotEq { .. }
            | crate::commit_error::CommitError::CommitToStrConversion { .. } => None,
        }
    }
    fn is_commit_to_str_conversion(v: &crate::commit_error::CommitError) -> Option<()> {
        match v {
            crate::commit_error::CommitError::CommitToStrConversion { .. } => Some(()),
            crate::commit_error::CommitError::CommitNotEq { .. }
            | crate::commit_error::CommitError::NoCommitHeader { .. } => None,
        }
    }
    fn commit_not_eq_fields(
        v: &crate::commit_error::CommitError,
    ) -> Option<(&'static str, &'static str)> {
        match v {
            crate::commit_error::CommitError::CommitNotEq {
                commit_not_eq,
                commit_to_use,
                ..
            } => Some((commit_not_eq.as_str(), commit_to_use.as_str())),
            crate::commit_error::CommitError::CommitToStrConversion { .. }
            | crate::commit_error::CommitError::NoCommitHeader { .. } => None,
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
            <&'static str>::from(
                git_info::project_git_commit_link_ref_value::project_git_commit_link_ref_value()
            ),
        );
    }
    fn assert_wrong_commit_err(headers: &axum::http::HeaderMap, exp_id: &'static str) {
        let fields = expect_check_commit_err_variant(headers, exp_id, commit_not_eq_fields);
        assert_wrong_commit_fields(fields);
    }
    fn expect_check_commit_err_variant<R>(
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&crate::commit_error::CommitError) -> Option<R>,
    ) -> R {
        crate::expect_err_variant_ref_with_status::expect_err_variant_ref_with_status(
            check_commit_enabled(headers),
            exp_id,
            Some(crate::axum_http_status_code::AxumHttpStatusCode::bad_request()),
            map,
        )
    }
    fn expect_get_commit_header_str_err_variant<R>(
        headers: &axum::http::HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&crate::commit_error::CommitError) -> Option<R>,
    ) -> R {
        crate::expect_err_variant_ref_with_status::expect_err_variant_ref_with_status(
            crate::read_commit_header_str::read_commit_header_str(
                crate::axum_headers_ref::AxumHeadersRef::from(headers),
            ),
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
        crate::assert_ok_eq::assert_ok_eq(
            crate::read_commit_header_str::read_commit_header_str(
                crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            )
            .map(<&str>::from),
            constants_str::E1D07F53,
            &git_info::project_git_info_value::project_git_info_value()
                .commit()
                .as_ref(),
        );
    }
    #[test]
    fn validate_commit_header_returns_error_when_header_is_absent() {
        let headers = axum::http::HeaderMap::new();
        let no_commit_header = crate::expect_error_variant_ref::expect_error_variant_ref(
            crate::validate_commit_header::validate_commit_header(
                crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            ),
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
        crate::expect_ok::expect_ok(
            crate::validate_commit_header::validate_commit_header(
                crate::axum_headers_ref::AxumHeadersRef::from(&headers),
            ),
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
        let fields = crate::expect_error_variant_ref::expect_error_variant_ref(
            crate::validate_commit_header_value::validate_commit_header_value(
                crate::header_str_ref::HeaderStrRef::from(constants_str::TEST_VALUES_WRONG_COMMIT),
            ),
            constants_str::VALUE_6804382F,
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn validate_commit_header_value_accepts_project_commit() {
        crate::expect_ok::expect_ok(
            crate::validate_commit_header_value::validate_commit_header_value(
                crate::header_str_ref::HeaderStrRef::from(
                    git_info::project_git_info_value::project_git_info_value()
                        .commit()
                        .as_ref(),
                ),
            ),
            constants_str::VALUE_5EF927D2,
        );
    }
    #[test]
    fn check_commit_returns_expected_commit_link_for_wrong_commit() {
        let headers = make_headers_with_wrong_commit();
        let fields = crate::expect_error_variant_ref::expect_error_variant_ref(
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
        crate::replace_header_name::replace_header_name(
            &mut headers,
            crate::commit_header_name::COMMIT_HEADER_NAME,
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
        assert!(git_info::check_is_project_commit::check_is_project_commit(
            git_info::project_git_info_value::project_git_info_value().commit()
        ));
    }
    #[test]
    fn non_project_commit_is_rejected_by_git_info_helper() {
        assert!(!git_info::check_is_project_commit::check_is_project_commit(
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
