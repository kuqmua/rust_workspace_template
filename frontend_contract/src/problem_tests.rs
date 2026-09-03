#[cfg(test)]
mod tests {
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "lint suppression is required here"
    )]
    fn test_every_api_problem_error_is_an_error_enum_with_a_json_response() {
        fn assert_error<Error>()
        where
            Error: std::error::Error,
        {
            let _: std::marker::PhantomData<Error> = std::marker::PhantomData;
        }

        assert_error::<crate::api_problem_error::ApiProblemError>();
        let internal_status = crate::api_problem_status::ApiProblemStatus::try_from(500u16)
            .expect(constants_str::DIAGNOSTIC_D2372BB7);
        let request_failed_status = crate::api_problem_status::ApiProblemStatus::try_from(418u16)
            .expect(constants_str::DIAGNOSTIC_805DA7F4);
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
                    constants_str::APPLICATION_PROBLEM_PLUS_JSON
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
            .expect(constants_str::DIAGNOSTIC_3E43E7BC);
            let problem = serde_json::from_slice::<crate::api_problem::ApiProblem>(&body)
                .expect(constants_str::DIAGNOSTIC_116DC695);
            assert_eq!(u16::from(problem.status()), status);
            assert_eq!(problem.kind(), kind);
        });
    }

    #[test]
    fn test_problem_text_deserialization_uses_bounded_try_from() {
        let detail = serde_json::to_string(&constants_str::X.repeat(1_025usize))
            .expect(constants_str::DIAGNOSTIC_6E2DB8A1);
        let request_id = serde_json::to_string(&constants_str::X.repeat(129usize))
            .expect(constants_str::DIAGNOSTIC_F289A40C);
        let _detail_error =
            serde_json::from_str::<crate::api_problem_detail::ApiProblemDetail>(&detail)
                .expect_err(constants_str::VALUE_9024021D);
        let _field_error =
            serde_json::from_str::<crate::api_problem_field::ApiProblemField>(&request_id)
                .expect_err(constants_str::VALUE_9D9ABF28);
        let _request_id_error =
            serde_json::from_str::<crate::api_problem_request_id::ApiProblemRequestId>(&request_id)
                .expect_err(constants_str::VALUE_4DC83C61);
    }

    #[test]
    fn test_problem_violation_deserialization_rejects_too_many_items() {
        let item = serde_json::json!({ "detail": "invalid", "field": "name" });
        let items: [serde_json::Value; 129usize] = std::array::from_fn(|_index| item.clone());
        let serialized =
            serde_json::to_string(&items[..]).expect(constants_str::DIAGNOSTIC_A1010D3F);
        let _error = serde_json::from_str::<crate::api_problem_violations::ApiProblemViolations>(
            &serialized,
        )
        .expect_err(constants_str::VALUE_8961C40A);
    }
}
