#[test]
fn contract_bodies_reject_values_above_shared_limit() {
    let oversized =
        vec![constants_u8::ZERO; super::FRONTEND_CONTRACT_BODY_MAX_BYTES + constants_usize::ONE];
    assert_eq!(
        super::TransportBody::try_from(oversized),
        Err(super::FrontendContractBodyError)
    );
}
#[allow(clippy::needless_for_each)] // iterator form follows the workspace ban on explicit for loops
#[test]
fn api_problem_status_mapping_is_stable_and_redacted() {
    let cases = [
        (400u16, super::ApiProblemKind::InvalidRequest),
        (401u16, super::ApiProblemKind::Authentication),
        (403u16, super::ApiProblemKind::Authorization),
        (404u16, super::ApiProblemKind::NotFound),
        (405u16, super::ApiProblemKind::MethodNotAllowed),
        (409u16, super::ApiProblemKind::Conflict),
        (412u16, super::ApiProblemKind::Precondition),
        (413u16, super::ApiProblemKind::PayloadTooLarge),
        (418u16, super::ApiProblemKind::RequestFailed),
        (422u16, super::ApiProblemKind::Validation),
        (425u16, super::ApiProblemKind::InProgress),
        (428u16, super::ApiProblemKind::PreconditionRequired),
        (429u16, super::ApiProblemKind::RateLimited),
        (500u16, super::ApiProblemKind::Internal),
        (503u16, super::ApiProblemKind::Internal),
    ];
    cases.into_iter().for_each(|(status, expected_kind)| {
        let problem = super::ApiProblem::from_error(super::ApiProblemError::from_status(
            super::ApiProblemStatus::try_from(status).expect(
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
    let type_contract = super::TypeContract::new(
        super::InputKind::Number,
        super::ValueFormat::Int64,
        super::Nullability::NonNullable,
    )
    .with_minimum(super::NumericBound::Inclusive(super::ContractI64::from(1)))
    .with_step(super::InputStep::Integer);
    let field = super::FieldContract::new(
        super::ContractStr::from(constants_str::SQL_NAMES_ID),
        super::ContractStr::from(constants_str::ID),
        type_contract,
    )
    .with_primary_key(super::PrimaryKeyKind::Primary)
    .with_readable(super::FieldCapability::Enabled);
    assert_eq!(field.type_contract().input_kind(), super::InputKind::Number);
    assert_eq!(field.primary_key(), super::PrimaryKeyKind::Primary);
    assert_eq!(field.readable(), super::FieldCapability::Enabled);
}
#[test]
fn public_catalog_wrappers_preserve_vec_conversions() {
    let fields = super::FieldContracts::from(Vec::<super::FieldContract>::new());
    let actions = super::ActionContracts::from(Vec::<super::ActionContract>::new());
    let routes = super::RouteContracts::from(Vec::<super::RouteContract>::new());
    let coverage =
        super::RouteCoverageDescriptors::from(Vec::<super::RouteCoverageDescriptor>::new());
    let schemas = super::RouteSchemaContracts::from(Vec::<super::RouteSchemaContract>::new());
    let metadata = super::RouteMetadataList::from(Vec::<super::RouteMetadata>::new());
    let categories = super::RouteTestCategories::from(vec![
        super::RouteTestCategory::FixtureHook,
        super::RouteTestCategory::Metadata,
    ]);
    assert!(fields.as_ref().is_empty());
    assert!(actions.as_ref().is_empty());
    assert!(routes.as_ref().is_empty());
    assert!(coverage.as_ref().is_empty());
    assert!(schemas.as_ref().is_empty());
    assert!(metadata.as_ref().is_empty());
    assert_eq!(
        categories.as_ref(),
        [
            super::RouteTestCategory::FixtureHook,
            super::RouteTestCategory::Metadata,
        ]
    );
}
#[test]
fn route_contract_keeps_transport_policy_together() {
    let route = super::RouteContract::new(
        super::AuthenticationRequirement::Permission(super::ContractStr::from(
            constants_str::PERMISSION,
        )),
        super::HttpMethod::Patch,
        super::MutationKind::Mutating,
        super::ContractStr::from(constants_str::USERS_ID),
        super::SuccessStatus::Code204,
    );
    assert_eq!(route.mutation(), super::MutationKind::Mutating);
    assert_eq!(route.method(), super::HttpMethod::Patch);
    assert_eq!(route.path().as_ref(), "/users/{id}");
}
#[test]
fn route_error_policy_derives_statuses_from_access_and_mutation() {
    let permission = super::AuthenticationRequirement::Permission(super::ContractStr::from(
        constants_str::PERMISSION,
    ));
    assert_eq!(
        super::RouteErrorPolicy::Default.statuses(
            super::AuthenticationRequirement::Public,
            super::RouteMutation::ReadOnly,
        ),
        super::PUBLIC_READ_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        super::RouteErrorPolicy::Default.statuses(
            super::AuthenticationRequirement::Authenticated,
            super::RouteMutation::Mutating,
        ),
        super::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        super::RouteErrorPolicy::Default.statuses(permission, super::RouteMutation::Mutating,),
        super::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        super::RouteErrorPolicy::Authentication
            .statuses(permission, super::RouteMutation::ReadOnly),
        super::PUBLIC_AUTH_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        super::RouteErrorPolicy::Delete.statuses(permission, super::RouteMutation::Mutating),
        super::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        super::RouteErrorPolicy::ValidatedRead.statuses(permission, super::RouteMutation::ReadOnly),
        super::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES
    );
}
#[test]
fn response_interpretation_uses_shared_success_and_problem_contract() {
    let problem = super::ApiProblem::from_error(super::ApiProblemError::from_status(
        super::ApiProblemStatus::try_from(401u16).expect("b8fc4707 response_interpretation_uses_shared_success_and_problem_contract invariant must hold"),
    ));
    let body = super::TransportBody::try_from(serde_json::to_vec(&problem).expect("f542a3cb response_interpretation_uses_shared_success_and_problem_contract invariant must hold"))
        .expect("864276f2 response_interpretation_uses_shared_success_and_problem_contract invariant must hold");
    let response = super::TransportResponse::new(
        body,
        super::TransportStatus::try_from(401u16).expect("a05ea02c response_interpretation_uses_shared_success_and_problem_contract invariant must hold"),
    );
    let error = response
        .success_body(super::SuccessStatus::Code200.transport_status())
        .expect_err(constants_str::VALUE_5EEA7F90);
    assert!(matches!(
        error,
        super::ClientError::Problem(value)
            if value.kind() == super::ApiProblemKind::Authentication
    ));
    assert_eq!(
        u16::from(super::SuccessStatus::Code201.transport_status()),
        201u16
    );
}
#[test]
fn transport_response_preserves_retry_after() {
    let response = super::TransportResponse::new(
        super::TransportBody::try_from(Vec::new())
            .expect("da32dc29 transport_response_preserves_retry_after invariant must hold"),
        super::TransportStatus::try_from(429u16)
            .expect("7a783a69 transport_response_preserves_retry_after invariant must hold"),
    )
    .with_retry_after(Some(
        super::TransportRetryAfter::try_from(constants_str::TEST_VALUE_30.to_owned())
            .expect("9b6750d4 transport_response_preserves_retry_after invariant must hold"),
    ));
    assert_eq!(
        response.retry_after().map(AsRef::as_ref),
        Some(constants_str::TEST_VALUE_30)
    );
}
#[test]
fn http_status_wrappers_reject_values_below_protocol_range() {
    let _transport_error = super::TransportStatus::try_from(99u16).expect_err("5d7c8801");
    let _problem_error = super::ApiProblemStatus::try_from(99u16).expect_err("e65c913c");
}
