#[test]
fn contract_bodies_reject_values_above_shared_limit() {
    let oversized =
        vec![constants_u8::ZERO; constants_usize::VALUE_16_777_216 + constants_usize::ONE];
    assert_eq!(
        crate::transport_body::TransportBody::try_from(oversized),
        Err(crate::frontend_contract_body_error::FrontendContractBodyError::TooLarge)
    );
}
#[allow(clippy::needless_for_each)] // iterator form follows the workspace ban on explicit for loops
#[test]
fn api_problem_status_mapping_is_stable_and_redacted() {
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
        let problem = crate::api_problem::ApiProblem::from_error(crate::api_problem_error::ApiProblemError::from_status(
            crate::api_problem_status::ApiProblemStatus::try_from(status).expect(
                "ff774b42 api_problem_status_mapping_is_stable_and_redacted invariant must hold",
            ),
        ));
        assert_eq!(problem.kind(), expected_kind);
        assert_eq!(u16::from(problem.status()), status);
        let serialized = serde_json::to_string(&problem).expect(
            "f459312e api_problem_status_mapping_is_stable_and_redacted invariant must hold",
        );
        assert!(!serialized.contains("postgres://"));
        assert!(!serialized.contains("sqlx"));
        assert!(!serialized.contains("password"));
    });
}
#[test]
fn contracts_preserve_typed_metadata() {
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
        crate::contract_str::ContractStr::from(constants_str::catalog::SQL_NAMES_ID),
        crate::contract_str::ContractStr::from(constants_str::catalog::ID),
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
fn public_catalog_wrappers_preserve_checked_vec_conversions() {
    let fields = crate::field_contracts::FieldContracts::try_from(Vec::<
        crate::field_contract::FieldContract,
    >::new())
    .expect("0e62631f empty fields fit collection bound");
    let actions = crate::action_contracts::ActionContracts::try_from(Vec::<
        crate::action_contract::ActionContract,
    >::new())
    .expect("8add9c33 empty actions fit collection bound");
    let routes = crate::route_contracts::RouteContracts::try_from(Vec::<
        crate::route_contract::RouteContract,
    >::new())
    .expect("96a2f2a6 empty routes fit collection bound");
    let coverage = crate::route_coverage_descriptors::RouteCoverageDescriptors::try_from(Vec::<
        crate::route_coverage_descriptor::RouteCoverageDescriptor,
    >::new(
    ))
    .expect("2ba61bce empty coverage descriptors fit collection bound");
    let schemas = crate::route_schema_contracts::RouteSchemaContracts::try_from(Vec::<
        crate::route_schema_contract::RouteSchemaContract,
    >::new())
    .expect("c335f3ea empty schemas fit collection bound");
    let metadata = crate::route_metadata_list::RouteMetadataList::try_from(Vec::<
        crate::route_metadata::RouteMetadata,
    >::new())
    .expect("207b72cc empty metadata fit collection bound");
    let categories = crate::route_test_categories::RouteTestCategories::try_from(vec![
        crate::route_test_category::RouteTestCategory::FixtureHook,
        crate::route_test_category::RouteTestCategory::Metadata,
    ])
    .expect("76f3e14a categories fit collection bound");
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
fn route_test_categories_reject_oversized_vec() {
    let categories = vec![
        crate::route_test_category::RouteTestCategory::Metadata;
        bounded_types::collection_max_len::COLLECTION_MAX_LEN
            + constants_usize::ONE
    ];
    let _error = crate::route_test_categories::RouteTestCategories::try_from(categories)
        .expect_err(constants_str::test_fixtures::VALUE_64271BEF);
}
#[test]
fn route_contract_keeps_transport_policy_together() {
    let route = crate::route_contract::RouteContract::new(
        crate::authentication_requirement::AuthenticationRequirement::Permission(
            crate::contract_str::ContractStr::from(constants_str::catalog::PERMISSION),
        ),
        crate::route_method::RouteMethod::Patch,
        crate::mutation_kind::MutationKind::Mutating,
        crate::contract_str::ContractStr::from(constants_str::catalog::USERS_ID),
        crate::success_status::SuccessStatus::Code204,
    );
    assert_eq!(
        route.mutation(),
        crate::mutation_kind::MutationKind::Mutating
    );
    assert_eq!(route.method(), crate::route_method::RouteMethod::Patch);
    assert_eq!(route.path().as_ref(), "/users/{id}");
}
#[test]
fn route_error_policy_derives_statuses_from_access_and_mutation() {
    let permission = crate::authentication_requirement::AuthenticationRequirement::Permission(
        crate::contract_str::ContractStr::from(constants_str::catalog::PERMISSION),
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
fn response_interpretation_uses_shared_success_and_problem_contract() {
    let problem = crate::api_problem::ApiProblem::from_error(crate::api_problem_error::ApiProblemError::from_status(
        crate::api_problem_status::ApiProblemStatus::try_from(401u16).expect("b8fc4707 response_interpretation_uses_shared_success_and_problem_contract invariant must hold"),
    ));
    let body = crate::transport_body::TransportBody::try_from(serde_json::to_vec(&problem).expect("f542a3cb response_interpretation_uses_shared_success_and_problem_contract invariant must hold"))
        .expect("864276f2 response_interpretation_uses_shared_success_and_problem_contract invariant must hold");
    let response = crate::transport_response::TransportResponse::new(
        body,
        crate::transport_status::TransportStatus::try_from(401u16).expect("a05ea02c response_interpretation_uses_shared_success_and_problem_contract invariant must hold"),
    );
    let error = response
        .success_body(crate::success_status::SuccessStatus::Code200.transport_status())
        .expect_err(constants_str::catalog::VALUE_5EEA7F90);
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
fn transport_response_preserves_retry_after() {
    let response = crate::transport_response::TransportResponse::new(
        crate::transport_body::TransportBody::try_from(Vec::new())
            .expect("da32dc29 transport_response_preserves_retry_after invariant must hold"),
        crate::transport_status::TransportStatus::try_from(429u16)
            .expect("7a783a69 transport_response_preserves_retry_after invariant must hold"),
    )
    .with_retry_after(Some(
        crate::transport_retry_after::TransportRetryAfter::try_from(
            constants_str::test_fixtures::TEST_VALUE_30.to_owned(),
        )
        .expect("9b6750d4 transport_response_preserves_retry_after invariant must hold"),
    ));
    assert_eq!(
        response.retry_after().map(AsRef::as_ref),
        Some(constants_str::test_fixtures::TEST_VALUE_30)
    );
}
#[test]
fn http_status_wrappers_reject_values_below_protocol_range() {
    let _transport_error = crate::transport_status::TransportStatus::try_from(99u16)
        .expect_err(constants_str::test_fixtures::VALUE_0A8708C8);
    let _problem_error = crate::api_problem_status::ApiProblemStatus::try_from(99u16)
        .expect_err(constants_str::test_fixtures::VALUE_766AAE46);
}
