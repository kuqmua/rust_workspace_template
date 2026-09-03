#[test]
fn test_contract_bodies_reject_values_above_shared_limit() {
    let oversized =
        vec![constants_u8::ZERO; constants_usize::VALUE_16_777_216 + constants_usize::ONE];
    assert_eq!(
        crate::transport_body::TransportBody::try_from(oversized),
        Err(crate::frontend_contract_body_error::FrontendContractBodyError::TooLarge)
    );
}
#[allow(
    clippy::needless_for_each,
    reason = "lint suppression is required here"
)]
#[test]
fn test_api_problem_status_mapping_is_stable_and_redacted() {
    let cases = [
        (
            400u16,
            crate::api_problem_kind::ApiProblemKind::InvalidRequest,
        ),
        (
            401u16,
            crate::api_problem_kind::ApiProblemKind::Authentication,
        ),
        (
            403u16,
            crate::api_problem_kind::ApiProblemKind::Authorization,
        ),
        (404u16, crate::api_problem_kind::ApiProblemKind::NotFound),
        (
            405u16,
            crate::api_problem_kind::ApiProblemKind::MethodNotAllowed,
        ),
        (409u16, crate::api_problem_kind::ApiProblemKind::Conflict),
        (
            412u16,
            crate::api_problem_kind::ApiProblemKind::Precondition,
        ),
        (
            413u16,
            crate::api_problem_kind::ApiProblemKind::PayloadTooLarge,
        ),
        (
            418u16,
            crate::api_problem_kind::ApiProblemKind::RequestFailed,
        ),
        (422u16, crate::api_problem_kind::ApiProblemKind::Validation),
        (425u16, crate::api_problem_kind::ApiProblemKind::InProgress),
        (
            428u16,
            crate::api_problem_kind::ApiProblemKind::PreconditionRequired,
        ),
        (429u16, crate::api_problem_kind::ApiProblemKind::RateLimited),
        (500u16, crate::api_problem_kind::ApiProblemKind::Internal),
        (503u16, crate::api_problem_kind::ApiProblemKind::Internal),
    ];
    cases.into_iter().for_each(|(status, expected_kind)| {
        let problem = crate::api_problem::ApiProblem::from_error(
            crate::api_problem_error::ApiProblemError::from_status(
                crate::api_problem_status::ApiProblemStatus::try_from(status)
                    .expect(constants_str::DIAGNOSTIC_FF774B42),
            ),
        );
        assert_eq!(problem.kind(), expected_kind);
        assert_eq!(u16::from(problem.status()), status);
        let serialized = serde_json::to_string(&problem).expect(constants_str::DIAGNOSTIC_F459312E);
        assert!(!serialized.contains(constants_str::VALUE_FB4B8AD6));
        assert!(!serialized.contains(constants_str::SQLX));
        assert!(!serialized.contains(constants_str::PASSWORD));
    });
}
#[test]
fn test_contracts_preserve_typed_metadata() {
    let type_contract = crate::type_contract::TypeContract::new(
        crate::input_kind::InputKind::Number,
        crate::value_format::ValueFormat::Int64,
        crate::nullability::Nullability::NonNullable,
    )
    .with_minimum(crate::numeric_bound::NumericBound::Inclusive(
        crate::contract_i64::ContractI64::from(1),
    ))
    .with_step(crate::input_step::InputStep::Integer);
    let field = crate::field_contract::FieldContract::new(
        crate::contract_str::ContractStr::from(constants_str::SQL_NAMES_ID),
        crate::contract_str::ContractStr::from(constants_str::ID),
        type_contract,
    )
    .with_primary_key(crate::primary_key_kind::PrimaryKeyKind::Primary)
    .with_readable(crate::field_capability::FieldCapability::Enabled);
    assert_eq!(
        field.type_contract().input_kind(),
        crate::input_kind::InputKind::Number
    );
    assert_eq!(
        field.primary_key(),
        crate::primary_key_kind::PrimaryKeyKind::Primary
    );
    assert_eq!(
        field.readable(),
        crate::field_capability::FieldCapability::Enabled
    );
}
#[test]
fn test_public_catalog_wrappers_preserve_checked_vec_conversions() {
    let fields = crate::field_contracts::FieldContracts::try_from(Vec::<
        crate::field_contract::FieldContract,
    >::new())
    .expect(constants_str::DIAGNOSTIC_0E62631F);
    let actions = crate::action_contracts::ActionContracts::try_from(Vec::<
        crate::action_contract::ActionContract,
    >::new())
    .expect(constants_str::DIAGNOSTIC_8ADD9C33);
    let routes = crate::route_contracts::RouteContracts::try_from(Vec::<
        crate::route_contract::RouteContract,
    >::new())
    .expect(constants_str::DIAGNOSTIC_96A2F2A6);
    let coverage = crate::route_coverage_descriptors::RouteCoverageDescriptors::try_from(Vec::<
        crate::route_coverage_descriptor::RouteCoverageDescriptor,
    >::new(
    ))
    .expect(constants_str::DIAGNOSTIC_2BA61BCE);
    let schemas = crate::route_schema_contracts::RouteSchemaContracts::try_from(Vec::<
        crate::route_schema_contract::RouteSchemaContract,
    >::new())
    .expect(constants_str::DIAGNOSTIC_C335F3EA);
    let metadata = crate::route_metadata_list::RouteMetadataList::try_from(Vec::<
        crate::route_metadata::RouteMetadata,
    >::new())
    .expect(constants_str::DIAGNOSTIC_207B72CC);
    let categories = crate::route_test_categories::RouteTestCategories::try_from(vec![
        crate::route_test_category::RouteTestCategory::FixtureHook,
        crate::route_test_category::RouteTestCategory::Metadata,
    ])
    .expect(constants_str::DIAGNOSTIC_76F3E14A);
    assert!(fields.as_ref().is_empty());
    assert!(actions.as_ref().is_empty());
    assert!(routes.as_ref().is_empty());
    assert!(coverage.as_ref().is_empty());
    assert!(schemas.as_ref().is_empty());
    assert!(metadata.as_ref().is_empty());
    assert_eq!(
        categories.as_ref(),
        [
            crate::route_test_category::RouteTestCategory::FixtureHook,
            crate::route_test_category::RouteTestCategory::Metadata,
        ]
    );
}

#[test]
fn test_route_test_categories_reject_oversized_vec() {
    let categories = vec![
        crate::route_test_category::RouteTestCategory::Metadata;
        bounded_types::collection_max_len::COLLECTION_MAX_LEN
            + constants_usize::ONE
    ];
    let _error = crate::route_test_categories::RouteTestCategories::try_from(categories)
        .expect_err(constants_str::VALUE_64271BEF);
}
#[test]
fn test_route_contract_keeps_transport_policy_together() {
    let route = crate::route_contract::RouteContract::new(
        crate::authentication_requirement::AuthenticationRequirement::Permission(
            crate::contract_str::ContractStr::from(constants_str::PERMISSION),
        ),
        crate::route_method::RouteMethod::Patch,
        crate::mutation_kind::MutationKind::Mutating,
        crate::contract_str::ContractStr::from(constants_str::USERS_ID),
        crate::success_status::SuccessStatus::Code204,
    );
    assert_eq!(
        route.mutation(),
        crate::mutation_kind::MutationKind::Mutating
    );
    assert_eq!(route.method(), crate::route_method::RouteMethod::Patch);
    assert_eq!(route.path().as_ref(), constants_str::USERS_ID);
}
#[test]
fn test_route_error_policy_derives_statuses_from_access_and_mutation() {
    let permission = crate::authentication_requirement::AuthenticationRequirement::Permission(
        crate::contract_str::ContractStr::from(constants_str::PERMISSION),
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Default.statuses(
            crate::authentication_requirement::AuthenticationRequirement::Public,
            crate::route_mutation::RouteMutation::ReadOnly,
        ),
        crate::route_contract::PUBLIC_READ_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Default.statuses(
            crate::authentication_requirement::AuthenticationRequirement::Authenticated,
            crate::route_mutation::RouteMutation::Mutating,
        ),
        crate::route_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Default
            .statuses(permission, crate::route_mutation::RouteMutation::Mutating,),
        crate::route_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Authentication
            .statuses(permission, crate::route_mutation::RouteMutation::ReadOnly),
        crate::route_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Delete
            .statuses(permission, crate::route_mutation::RouteMutation::Mutating),
        crate::route_contract::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::ValidatedRead
            .statuses(permission, crate::route_mutation::RouteMutation::ReadOnly),
        crate::route_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES
    );
}
#[test]
fn test_response_interpretation_uses_shared_success_and_problem_contract() {
    let problem = crate::api_problem::ApiProblem::from_error(
        crate::api_problem_error::ApiProblemError::from_status(
            crate::api_problem_status::ApiProblemStatus::try_from(401u16)
                .expect(constants_str::DIAGNOSTIC_B8FC4707),
        ),
    );
    let body = crate::transport_body::TransportBody::try_from(
        serde_json::to_vec(&problem).expect(constants_str::DIAGNOSTIC_F542A3CB),
    )
    .expect(constants_str::DIAGNOSTIC_864276F2);
    let response = crate::transport_response::TransportResponse::new(
        body,
        crate::transport_status::TransportStatus::try_from(401u16)
            .expect(constants_str::DIAGNOSTIC_A05EA02C),
    );
    let error = response
        .success_body(crate::success_status::SuccessStatus::Code200.transport_status())
        .expect_err(constants_str::VALUE_5EEA7F90);
    assert!(matches!(
        error,
        crate::client_error::ClientError::Problem(value)
            if value.kind() == crate::api_problem_kind::ApiProblemKind::Authentication
    ));
    assert_eq!(
        u16::from(crate::success_status::SuccessStatus::Code201.transport_status()),
        201u16
    );
}
#[test]
fn test_transport_response_preserves_retry_after() {
    let response = crate::transport_response::TransportResponse::new(
        crate::transport_body::TransportBody::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_DA32DC29),
        crate::transport_status::TransportStatus::try_from(429u16)
            .expect(constants_str::DIAGNOSTIC_7A783A69),
    )
    .with_retry_after(Some(
        crate::transport_retry_after::TransportRetryAfter::try_from(
            constants_str::TEST_VALUE_30.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_9B6750D4),
    ));
    assert_eq!(
        response.retry_after().map(AsRef::as_ref),
        Some(constants_str::TEST_VALUE_30)
    );
}
#[test]
fn test_http_status_wrappers_reject_values_below_protocol_range() {
    let _transport_error = crate::transport_status::TransportStatus::try_from(99u16)
        .expect_err(constants_str::VALUE_0A8708C8);
    let _problem_error = crate::api_problem_status::ApiProblemStatus::try_from(99u16)
        .expect_err(constants_str::VALUE_766AAE46);
}
