#[cfg(test)]
mod tests {
    #[test]
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(clippy::needless_for_each)]
    fn every_api_problem_error_is_an_error_enum_with_a_json_response() {
        fn assert_error<Error>()
        where
            Error: std::error::Error,
        {
        }

        assert_error::<crate::api_problem_error::ApiProblemError>();
        let internal_status = crate::api_problem_status::ApiProblemStatus::try_from(500u16)
            .expect("d2372bb7 assert_error invariant must hold");
        let request_failed_status = crate::api_problem_status::ApiProblemStatus::try_from(418u16)
            .expect("805da7f4 assert_error invariant must hold");
        [
            (
                crate::api_problem_error::ApiProblemError::InvalidRequest,
                400u16,
                crate::api_problem_kind::ApiProblemKind::InvalidRequest,
            ),
            (
                crate::api_problem_error::ApiProblemError::Authentication,
                401u16,
                crate::api_problem_kind::ApiProblemKind::Authentication,
            ),
            (
                crate::api_problem_error::ApiProblemError::Authorization,
                403u16,
                crate::api_problem_kind::ApiProblemKind::Authorization,
            ),
            (
                crate::api_problem_error::ApiProblemError::NotFound,
                404u16,
                crate::api_problem_kind::ApiProblemKind::NotFound,
            ),
            (
                crate::api_problem_error::ApiProblemError::MethodNotAllowed,
                405u16,
                crate::api_problem_kind::ApiProblemKind::MethodNotAllowed,
            ),
            (
                crate::api_problem_error::ApiProblemError::Conflict,
                409u16,
                crate::api_problem_kind::ApiProblemKind::Conflict,
            ),
            (
                crate::api_problem_error::ApiProblemError::Precondition,
                412u16,
                crate::api_problem_kind::ApiProblemKind::Precondition,
            ),
            (
                crate::api_problem_error::ApiProblemError::PayloadTooLarge,
                413u16,
                crate::api_problem_kind::ApiProblemKind::PayloadTooLarge,
            ),
            (
                crate::api_problem_error::ApiProblemError::Validation,
                422u16,
                crate::api_problem_kind::ApiProblemKind::Validation,
            ),
            (
                crate::api_problem_error::ApiProblemError::InProgress,
                425u16,
                crate::api_problem_kind::ApiProblemKind::InProgress,
            ),
            (
                crate::api_problem_error::ApiProblemError::PreconditionRequired,
                428u16,
                crate::api_problem_kind::ApiProblemKind::PreconditionRequired,
            ),
            (
                crate::api_problem_error::ApiProblemError::RateLimited,
                429u16,
                crate::api_problem_kind::ApiProblemKind::RateLimited,
            ),
            (
                crate::api_problem_error::ApiProblemError::Internal(internal_status),
                500u16,
                crate::api_problem_kind::ApiProblemKind::Internal,
            ),
            (
                crate::api_problem_error::ApiProblemError::ServiceUnavailable,
                503u16,
                crate::api_problem_kind::ApiProblemKind::Internal,
            ),
            (
                crate::api_problem_error::ApiProblemError::RequestFailed(request_failed_status),
                418u16,
                crate::api_problem_kind::ApiProblemKind::RequestFailed,
            ),
        ]
        .into_iter()
        .for_each(|(error, status, kind)| {
            let response = axum::response::IntoResponse::into_response(error);
            assert_eq!(response.status().as_u16(), status);
            assert_eq!(
                response.headers().get(axum::http::header::CONTENT_TYPE),
                Some(&axum::http::HeaderValue::from_static(
                    constants_str::catalog::APPLICATION_PROBLEM_PLUS_JSON
                ))
            );
            assert_eq!(
                response
                    .headers()
                    .contains_key(axum::http::header::RETRY_AFTER),
                status == 429u16
            );
            let body = futures::executor::block_on(axum::body::to_bytes(
                response.into_body(),
                16_384usize,
            ))
            .expect("3e43e7bc assert_error invariant must hold");
            let problem = serde_json::from_slice::<crate::api_problem::ApiProblem>(&body)
                .expect("116dc695 assert_error invariant must hold");
            assert_eq!(u16::from(problem.status()), status);
            assert_eq!(problem.kind(), kind);
        });
    }

    #[test]
    fn problem_text_deserialization_uses_bounded_try_from() {
        let detail = serde_json::to_string(&constants_str::catalog::X.repeat(1_025usize)).expect(
            "6e2db8a1 problem_text_deserialization_uses_bounded_try_from invariant must hold",
        );
        let request_id = serde_json::to_string(&constants_str::catalog::X.repeat(129usize)).expect(
            "f289a40c problem_text_deserialization_uses_bounded_try_from invariant must hold",
        );
        let _detail_error =
            serde_json::from_str::<crate::api_problem_detail::ApiProblemDetail>(&detail)
                .expect_err(constants_str::test_fixtures::VALUE_9024021D);
        let _field_error =
            serde_json::from_str::<crate::api_problem_field::ApiProblemField>(&request_id)
                .expect_err(constants_str::test_fixtures::VALUE_9D9ABF28);
        let _request_id_error =
            serde_json::from_str::<crate::api_problem_request_id::ApiProblemRequestId>(&request_id)
                .expect_err(constants_str::test_fixtures::VALUE_4DC83C61);
    }

    #[test]
    fn problem_violation_deserialization_rejects_too_many_items() {
        let item = serde_json::json!({ "detail": "invalid", "field": "name" });
        let serialized = serde_json::to_string(&vec![item; 129usize]).expect(
            "a1010d3f problem_violation_deserialization_rejects_too_many_items invariant must hold",
        );
        let _error = serde_json::from_str::<crate::api_problem_violations::ApiProblemViolations>(
            &serialized,
        )
        .expect_err(constants_str::test_fixtures::VALUE_8961C40A);
    }
}
