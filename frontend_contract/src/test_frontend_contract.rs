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
    reason = "test tests uses iterator traversal to comply with the workspace no-for-loop policy"
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
        crate::field_name::FieldName::from(crate::contract_str::ContractStr::from(
            constants_str::SQL_NAMES_ID,
        )),
        crate::field_label::FieldLabel::from(crate::contract_str::ContractStr::from(
            constants_str::ID,
        )),
        type_contract,
    )
    .with_primary_key(crate::primary_key_kind::PrimaryKeyKind::Primary)
    .with_readable(crate::field_capability::FieldCapability::Enabled);
    assert_eq!(field.name().as_ref(), constants_str::SQL_NAMES_ID);
    assert_eq!(field.label().as_ref(), constants_str::ID);
    assert_eq!(field.name().to_string(), constants_str::SQL_NAMES_ID);
    assert_eq!(field.label().to_string(), constants_str::ID);
    assert_eq!(
        crate::contract_str::ContractStr::from(field.name()),
        crate::contract_str::ContractStr::from(constants_str::SQL_NAMES_ID)
    );
    assert_eq!(
        crate::contract_str::ContractStr::from(field.label()),
        crate::contract_str::ContractStr::from(constants_str::ID)
    );
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
        crate::authentication_requirement::AuthenticationRequirement::Rule(
            crate::contract_str::ContractStr::from(constants_str::RULE),
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
    assert_eq!(
        route.authentication(),
        crate::authentication_requirement::AuthenticationRequirement::Rule(
            crate::contract_str::ContractStr::from(constants_str::RULE),
        )
    );
    assert_eq!(
        route.success_status(),
        crate::success_status::SuccessStatus::Code204
    );
}

#[test]
fn test_route_error_status_registries_preserve_exact_ordered_http_codes() {
    let authentication_statuses = [401u16, 413u16, 429u16, 500u16];
    let authorization_statuses = [401u16, 403u16, 413u16, 429u16, 500u16];
    assert!(
        [
            (
                crate::route_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
                authentication_statuses.as_slice()
            ),
            (
                crate::route_contract::PUBLIC_READ_ROUTE_ERROR_STATUSES,
                [500u16, 413u16, 429u16].as_slice()
            ),
            (
                crate::route_contract::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
                [413u16, 422u16, 429u16, 500u16].as_slice()
            ),
            (
                crate::route_contract::PUBLIC_REFRESH_ROUTE_ERROR_STATUSES,
                authentication_statuses.as_slice()
            ),
            (
                crate::route_contract::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES,
                authentication_statuses.as_slice()
            ),
            (
                crate::route_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES,
                authorization_statuses.as_slice()
            ),
            (
                crate::route_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES,
                [401u16, 403u16, 413u16, 422u16, 429u16, 500u16].as_slice()
            ),
            (
                crate::route_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES,
                authorization_statuses.as_slice()
            ),
            (
                crate::route_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES,
                [401u16, 403u16, 409u16, 413u16, 422u16, 429u16, 500u16].as_slice()
            ),
            (
                crate::route_contract::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES,
                [401u16, 403u16, 409u16, 413u16, 429u16, 500u16].as_slice()
            ),
        ]
        .into_iter()
        .all(|(statuses, expected)| statuses
            .iter()
            .copied()
            .map(|status| u16::from(status.transport_status()))
            .eq(expected.iter().copied()))
    );
}
#[test]
fn test_route_error_policy_derives_statuses_from_access_and_mutation() {
    let rule = crate::authentication_requirement::AuthenticationRequirement::Rule(
        crate::contract_str::ContractStr::from(constants_str::RULE),
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
            .statuses(rule, crate::route_mutation::RouteMutation::Mutating,),
        crate::route_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Authentication
            .statuses(rule, crate::route_mutation::RouteMutation::ReadOnly),
        crate::route_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::Delete
            .statuses(rule, crate::route_mutation::RouteMutation::Mutating),
        crate::route_contract::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES
    );
    assert_eq!(
        crate::route_error_policy::RouteErrorPolicy::ValidatedRead
            .statuses(rule, crate::route_mutation::RouteMutation::ReadOnly),
        crate::route_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES
    );
    [
        (
            crate::authentication_requirement::AuthenticationRequirement::Public,
            crate::route_mutation::RouteMutation::Mutating,
            crate::route_contract::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
        ),
        (
            crate::authentication_requirement::AuthenticationRequirement::Authenticated,
            crate::route_mutation::RouteMutation::ReadOnly,
            crate::route_contract::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES,
        ),
        (
            rule,
            crate::route_mutation::RouteMutation::ReadOnly,
            crate::route_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES,
        ),
    ]
    .into_iter()
    .fold(
        (),
        |(), (authentication_requirement, route_mutation, expected)| {
            assert_eq!(
                crate::route_error_policy::RouteErrorPolicy::Default
                    .statuses(authentication_requirement, route_mutation),
                expected
            );
        },
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
    );
    assert!(response.retry_after().is_none());
    let throttled = response.with_retry_after(Some(
        crate::transport_retry_after::TransportRetryAfter::try_from(
            constants_str::TEST_VALUE_30.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_9B6750D4),
    ));
    assert_eq!(
        throttled.retry_after().map(AsRef::as_ref),
        Some(constants_str::TEST_VALUE_30)
    );
    assert_eq!(u16::from(throttled.status()), 429u16);
    assert!(throttled.body().as_ref().is_empty());
    assert!(
        throttled
            .success_body(throttled.status())
            .is_ok_and(|body| std::ptr::eq(body, throttled.body()))
    );
    assert!(
        crate::transport_retry_after::TransportRetryAfter::try_from(
            constants_str::VALUE_60.to_owned()
        )
        .is_ok_and(|retry_after| {
            let updated = throttled.with_retry_after(Some(retry_after));
            assert_eq!(
                updated.retry_after().map(AsRef::as_ref),
                Some(constants_str::VALUE_60)
            );
            assert_eq!(u16::from(updated.status()), 429u16);
            assert!(updated.body().as_ref().is_empty());
            let cleared = updated.with_retry_after(None);
            assert!(cleared.retry_after().is_none());
            assert_eq!(u16::from(cleared.status()), 429u16);
            assert!(cleared.body().as_ref().is_empty());
            true
        })
    );
}
#[test]
fn test_http_status_wrappers_reject_values_below_protocol_range() {
    let _transport_error = crate::transport_status::TransportStatus::try_from(99u16)
        .expect_err(constants_str::VALUE_0A8708C8);
    let _problem_error = crate::api_problem_status::ApiProblemStatus::try_from(99u16)
        .expect_err(constants_str::VALUE_766AAE46);
}

#[test]
fn test_problem_decoder_rejects_non_problem_and_malformed_bodies() {
    [
        constants_str::EMPTY,
        constants_str::SPACE,
        constants_str::VALUE_F1234D75,
        constants_str::VALUE_4F53CDA1,
        constants_str::VALUE_1,
    ]
    .into_iter()
    .fold((), |(), value| {
        assert!(
            crate::transport_body::TransportBody::try_from(value.as_bytes().to_vec())
                .is_ok_and(|body| crate::decode_api_problem::decode_api_problem(&body).is_none())
        );
    });
}

#[test]
fn test_problem_decoder_accepts_trailing_whitespace_and_rejects_trailing_data() {
    let problem = crate::api_problem::ApiProblem::from_error(
        crate::api_problem_error::ApiProblemError::Authentication,
    );
    assert!(serde_json::to_vec(&problem).is_ok_and(|mut bytes| {
        bytes.extend_from_slice(constants_str::SPACE.as_bytes());
        let accepted =
            crate::transport_body::TransportBody::try_from(bytes.clone()).is_ok_and(|body| {
                crate::decode_api_problem::decode_api_problem(&body)
                    .is_some_and(|decoded| decoded == problem)
            });
        bytes.extend_from_slice(constants_str::X.as_bytes());
        accepted
            && crate::transport_body::TransportBody::try_from(bytes)
                .is_ok_and(|body| crate::decode_api_problem::decode_api_problem(&body).is_none())
    }));
}

#[test]
fn test_problem_decoder_rejects_missing_required_fields() {
    let problem = crate::api_problem::ApiProblem::from_error(
        crate::api_problem_error::ApiProblemError::Authentication,
    );
    [
        stringify!(detail),
        stringify!(violations),
        stringify!(status),
        stringify!(kind),
    ]
    .into_iter()
    .fold((), |(), field| {
        assert!(serde_json::to_value(&problem).is_ok_and(|mut value| {
            let removed = value
                .as_object_mut()
                .is_some_and(|object| object.remove(field).is_some());
            removed
                && serde_json::to_vec(&value).is_ok_and(|bytes| {
                    crate::transport_body::TransportBody::try_from(bytes).is_ok_and(|body| {
                        crate::decode_api_problem::decode_api_problem(&body).is_none()
                    })
                })
        }));
    });
}

#[test]
fn test_http_status_wrappers_accept_inclusive_boundaries_and_reject_out_of_range() {
    [
        (
            0u16,
            Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange),
        ),
        (
            99u16,
            Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange),
        ),
        (100u16, Ok(100u16)),
        (999u16, Ok(999u16)),
        (
            1_000u16,
            Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange),
        ),
        (
            u16::MAX,
            Err(crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange),
        ),
    ]
    .into_iter()
    .fold((), |(), (status, expected)| {
        assert_eq!(
            crate::transport_status::TransportStatus::try_from(status).map(u16::from),
            expected
        );
        assert_eq!(
            crate::api_problem_status::ApiProblemStatus::try_from(status).map(u16::from),
            expected
        );
    });
}

#[test]
fn test_type_contract_capabilities_cover_every_value_format() {
    let supported = crate::capability_support::CapabilitySupport::Supported;
    let unsupported = crate::capability_support::CapabilitySupport::Unsupported;
    assert!(
        [
            (crate::value_format::ValueFormat::Bool, supported, supported),
            (
                crate::value_format::ValueFormat::Bytes,
                unsupported,
                unsupported
            ),
            (crate::value_format::ValueFormat::Date, supported, supported),
            (
                crate::value_format::ValueFormat::DateTime,
                supported,
                supported
            ),
            (
                crate::value_format::ValueFormat::Float32,
                supported,
                supported
            ),
            (
                crate::value_format::ValueFormat::Float64,
                supported,
                supported
            ),
            (crate::value_format::ValueFormat::Inet, supported, supported),
            (
                crate::value_format::ValueFormat::Int16,
                supported,
                supported
            ),
            (
                crate::value_format::ValueFormat::Int32,
                supported,
                supported
            ),
            (
                crate::value_format::ValueFormat::Int64,
                supported,
                supported
            ),
            (
                crate::value_format::ValueFormat::Interval,
                unsupported,
                supported
            ),
            (crate::value_format::ValueFormat::Mac, supported, supported),
            (
                crate::value_format::ValueFormat::Range,
                unsupported,
                unsupported
            ),
            (crate::value_format::ValueFormat::Text, supported, supported),
            (crate::value_format::ValueFormat::Time, supported, supported),
            (
                crate::value_format::ValueFormat::Timestamp,
                supported,
                supported
            ),
            (
                crate::value_format::ValueFormat::TimestampTz,
                supported,
                supported
            ),
            (crate::value_format::ValueFormat::Uuid, supported, supported),
        ]
        .into_iter()
        .all(|(value_format, filtering, sorting)| {
            let contract = crate::type_contract::TypeContract::new(
                crate::input_kind::InputKind::Text,
                value_format,
                crate::nullability::Nullability::Nullable,
            );
            contract.supports_filtering() == filtering && contract.supports_sorting() == sorting
        })
    );
}

#[test]
fn test_type_contract_builders_preserve_defaults_and_unrelated_metadata() {
    let original = crate::type_contract::TypeContract::new(
        crate::input_kind::InputKind::Number,
        crate::value_format::ValueFormat::Int64,
        crate::nullability::Nullability::Nullable,
    );
    assert_eq!(original.example(), crate::value_example::ValueExample::None);
    assert_eq!(original.minimum(), crate::numeric_bound::NumericBound::None);
    assert_eq!(original.maximum(), crate::numeric_bound::NumericBound::None);
    assert_eq!(original.step(), crate::input_step::InputStep::Any);
    let minimum = crate::numeric_bound::NumericBound::Inclusive(
        crate::contract_i64::ContractI64::from(-5i64),
    );
    let maximum =
        crate::numeric_bound::NumericBound::Inclusive(crate::contract_i64::ContractI64::from(8i64));
    let changed = original
        .with_example(crate::value_example::ValueExample::Integer)
        .with_minimum(minimum)
        .with_maximum(maximum)
        .with_step(crate::input_step::InputStep::Integer);
    assert_eq!(
        changed.example(),
        crate::value_example::ValueExample::Integer
    );
    assert_eq!(changed.minimum(), minimum);
    assert_eq!(changed.maximum(), maximum);
    assert_eq!(changed.step(), crate::input_step::InputStep::Integer);
    assert_eq!(changed.input_kind(), original.input_kind());
    assert_eq!(changed.format(), original.format());
    assert_eq!(changed.nullability(), original.nullability());
    assert_eq!(
        changed
            .with_example(crate::value_example::ValueExample::None)
            .with_minimum(crate::numeric_bound::NumericBound::None)
            .with_maximum(crate::numeric_bound::NumericBound::None)
            .with_step(crate::input_step::InputStep::Any),
        original
    );
}

#[test]
fn test_action_contracts_and_route_catalogs_preserve_metadata_and_order() {
    let route = crate::route_contract::RouteContract::new(
        crate::authentication_requirement::AuthenticationRequirement::Public,
        crate::route_method::RouteMethod::Get,
        crate::mutation_kind::MutationKind::ReadOnly,
        crate::contract_str::ContractStr::from(constants_str::SLASH),
        crate::success_status::SuccessStatus::Code200,
    );
    let actions = [
        crate::operation_kind::OperationKind::CreateMany,
        crate::operation_kind::OperationKind::DeleteMany,
        crate::operation_kind::OperationKind::Read,
        crate::operation_kind::OperationKind::Update,
    ]
    .map(|operation_kind| {
        let original = crate::action_contract::ActionContract::new(operation_kind, route);
        assert_eq!(original.operation(), operation_kind);
        assert_eq!(original.route(), route);
        assert_eq!(
            original.confirmation(),
            crate::confirmation_requirement::ConfirmationRequirement::NotRequired
        );
        let confirmed = original
            .with_confirmation(crate::confirmation_requirement::ConfirmationRequirement::Required);
        assert_eq!(confirmed.operation(), operation_kind);
        assert_eq!(confirmed.route(), route);
        assert_eq!(
            confirmed.confirmation(),
            crate::confirmation_requirement::ConfirmationRequirement::Required
        );
        assert_eq!(
            confirmed.with_confirmation(
                crate::confirmation_requirement::ConfirmationRequirement::NotRequired
            ),
            original
        );
        confirmed
    });
    let ordered_actions = [
        actions[3usize],
        actions[0usize],
        actions[3usize],
        actions[1usize],
        actions[2usize],
    ];
    let action_catalog = crate::action_contracts::ActionContracts::from_max_iter(ordered_actions);
    assert_eq!(action_catalog.as_ref(), ordered_actions.as_slice());
    assert_eq!(
        crate::action_contracts::ActionContracts::try_from(ordered_actions.to_vec()),
        Ok(action_catalog)
    );
    assert!(
        crate::action_contracts::ActionContracts::from_max_iter(std::iter::empty())
            .as_ref()
            .is_empty()
    );
    let other_route = crate::route_contract::RouteContract::new(
        crate::authentication_requirement::AuthenticationRequirement::Public,
        crate::route_method::RouteMethod::Post,
        crate::mutation_kind::MutationKind::Mutating,
        crate::contract_str::ContractStr::from(constants_str::SLASH),
        crate::success_status::SuccessStatus::Code200,
    );
    let ordered_routes = [other_route, route, other_route];
    let route_catalog = crate::route_contracts::RouteContracts::from_max_iter(ordered_routes);
    assert_eq!(route_catalog.as_ref(), ordered_routes.as_slice());
    assert_eq!(
        crate::route_contracts::RouteContracts::try_from(ordered_routes.to_vec()),
        Ok(route_catalog)
    );
    assert!(
        crate::route_contracts::RouteContracts::from_max_iter(std::iter::empty())
            .as_ref()
            .is_empty()
    );
}

#[test]
fn test_contract_integer_bounds_match_standard_integer_limits() {
    assert!(
        [
            (
                crate::contract_i64::ContractI64::i16_min(),
                i64::from(i16::MIN)
            ),
            (
                crate::contract_i64::ContractI64::i16_max(),
                i64::from(i16::MAX)
            ),
            (
                crate::contract_i64::ContractI64::i32_min(),
                i64::from(i32::MIN)
            ),
            (
                crate::contract_i64::ContractI64::i32_max(),
                i64::from(i32::MAX)
            ),
            (crate::contract_i64::ContractI64::min(), i64::MIN),
            (crate::contract_i64::ContractI64::max(), i64::MAX),
        ]
        .into_iter()
        .all(|(actual, expected)| actual == crate::contract_i64::ContractI64::from(expected))
    );
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DefaultContractRouteFamily;
impl crate::route_family::RouteFamily for DefaultContractRouteFamily {
    fn coverage_descriptors() -> crate::route_coverage_descriptors::RouteCoverageDescriptors {
        crate::route_coverage_descriptors::RouteCoverageDescriptors::default()
    }
}
#[test]
fn test_route_family_defaults_preserve_empty_optional_contracts() {
    assert_eq!(
        <DefaultContractRouteFamily as crate::route_family::RouteFamily>::ROUTE_COUNT,
        constants_usize::ZERO
    );
    assert_eq!(
        <DefaultContractRouteFamily as crate::route_family::RouteFamily>::body_limit(),
        None
    );
    assert!(
        <DefaultContractRouteFamily as crate::route_family::RouteFamily>::schema_contracts()
            .as_ref()
            .is_empty()
    );
    assert!(
        <DefaultContractRouteFamily as crate::route_family::RouteFamily>::route_metadata()
            .as_ref()
            .is_empty()
    );
}
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_method_filters_match_axum_http_method_conversion() {
    assert!(
        [
            crate::route_method::RouteMethod::Connect,
            crate::route_method::RouteMethod::Delete,
            crate::route_method::RouteMethod::Get,
            crate::route_method::RouteMethod::Head,
            crate::route_method::RouteMethod::Options,
            crate::route_method::RouteMethod::Patch,
            crate::route_method::RouteMethod::Post,
            crate::route_method::RouteMethod::Put,
            crate::route_method::RouteMethod::Trace,
        ]
        .into_iter()
        .all(|route_method| {
            let actual = axum::routing::MethodFilter::from(
                crate::to_axum_method_filter::to_axum_method_filter(route_method),
            );
            axum::http::Method::from_bytes(route_method.as_str().as_ref().as_bytes()).is_ok_and(
                |method| {
                    axum::routing::MethodFilter::try_from(method)
                        .is_ok_and(|expected| actual == expected)
                },
            )
        })
    );
}

#[test]
fn test_schema_registration_replaces_stale_aliases_and_references_without_losing_unrelated_entries()
{
    let schema_name = <crate::api_problem::ApiProblem as utoipa::ToSchema>::name();
    let qualified_name = std::any::type_name::<crate::api_problem::ApiProblem>()
        .replace(constants_str::DOUBLE_COLON, constants_str::DOT);
    let crate_name = std::any::type_name::<crate::api_problem::ApiProblem>()
        .split(constants_str::DOUBLE_COLON)
        .next();
    let crate_qualified_name =
        crate_name.map(|crate_segment| format!("{crate_segment}.{schema_name}"));
    let references = [
        (
            <crate::api_problem_detail::ApiProblemDetail as utoipa::ToSchema>::name(),
            <crate::api_problem_detail::ApiProblemDetail as utoipa::PartialSchema>::schema(),
        ),
        (
            <crate::api_problem_request_id::ApiProblemRequestId as utoipa::ToSchema>::name(),
            <crate::api_problem_request_id::ApiProblemRequestId as utoipa::PartialSchema>::schema(),
        ),
        (
            <crate::api_problem_status::ApiProblemStatus as utoipa::ToSchema>::name(),
            <crate::api_problem_status::ApiProblemStatus as utoipa::PartialSchema>::schema(),
        ),
        (
            <crate::api_problem_kind::ApiProblemKind as utoipa::ToSchema>::name(),
            <crate::api_problem_kind::ApiProblemKind as utoipa::PartialSchema>::schema(),
        ),
    ];
    let stale = utoipa::openapi::RefOr::<utoipa::openapi::Schema>::Ref(
        utoipa::openapi::Ref::from_schema_name(constants_str::NEVER_PRINT_THIS_VALUE),
    );
    let mut components = utoipa::openapi::schema::Components::new();
    components.schemas = std::collections::BTreeMap::from([
        (schema_name.clone().into_owned(), stale.clone()),
        (qualified_name.clone(), stale.clone()),
        (
            constants_str::NEVER_PRINT_THIS_VALUE.to_owned(),
            stale.clone(),
        ),
    ]);
    assert!(crate_qualified_name.as_ref().is_some_and(|name| {
        let previous = components.schemas.insert(name.clone(), stale.clone());
        previous.is_none()
    }));
    let referenced_entries = references
        .iter()
        .map(|(name, _schema)| (name.clone().into_owned(), stale.clone()));
    components.schemas.extend(referenced_entries);
    let mut components_ref_mut =
        crate::utoipa_open_api_components_ref_mut::UtoipaOpenApiComponentsRefMut::from(
            &mut components,
        );
    crate::register_openapi_schema::register_openapi_schema::<crate::api_problem::ApiProblem>(
        &mut components_ref_mut,
    );
    let first_registration = serde_json::to_value(&*components_ref_mut);
    crate::register_openapi_schema::register_openapi_schema::<crate::api_problem::ApiProblem>(
        &mut components_ref_mut,
    );
    assert!(first_registration.is_ok_and(|first| {
        serde_json::to_value(&*components_ref_mut).is_ok_and(|second| first == second)
    }));
    let expected_schema = <crate::api_problem::ApiProblem as utoipa::PartialSchema>::schema();
    assert!(
        [
            Some(schema_name.as_ref()),
            Some(qualified_name.as_str()),
            crate_qualified_name.as_deref()
        ]
        .into_iter()
        .all(|optional_name| {
            optional_name.is_some_and(|name| {
                components_ref_mut.schemas.get(name).is_some_and(|schema| {
                    serde_json::to_value(schema).is_ok_and(|actual| {
                        serde_json::to_value(&expected_schema)
                            .is_ok_and(|expected| actual == expected)
                    })
                })
            })
        })
    );
    assert!(references.iter().all(|(name, schema)| {
        components_ref_mut
            .schemas
            .get(name.as_ref())
            .is_some_and(|registered| {
                serde_json::to_value(registered).is_ok_and(|actual| {
                    serde_json::to_value(schema).is_ok_and(|expected| actual == expected)
                })
            })
    }));
    assert!(
        components_ref_mut
            .schemas
            .get(constants_str::NEVER_PRINT_THIS_VALUE)
            .is_some_and(|schema| serde_json::to_value(schema).is_ok_and(|actual| {
                serde_json::to_value(&stale).is_ok_and(|expected| actual == expected)
            }))
    );
}

#[test]
fn test_client_error_display_preserves_operation_and_source_details() {
    let form_value_error = crate::create_form_value_error::create_form_value_error(
        constants_str::INVALID_FILTER_SPECIFICATION,
    );
    assert!(
        crate::transport_error::TransportError::try_from(
            constants_str::INVALID_FILTER_SPECIFICATION.to_owned()
        )
        .is_ok_and(|transport_error| {
            let cases = [
                (
                    crate::client_error::ClientError::Decode(form_value_error.clone()),
                    format!(
                        "{}{}",
                        constants_str::CLIENT_ERROR_DECODE_PREFIX,
                        constants_str::INVALID_FILTER_SPECIFICATION
                    ),
                ),
                (
                    crate::client_error::ClientError::Encode(form_value_error),
                    format!(
                        "{}{}",
                        constants_str::CLIENT_ERROR_ENCODE_PREFIX,
                        constants_str::INVALID_FILTER_SPECIFICATION
                    ),
                ),
                (
                    crate::client_error::ClientError::Transport(transport_error),
                    format!(
                        "{}{}",
                        constants_str::CLIENT_ERROR_TRANSPORT_PREFIX,
                        constants_str::INVALID_FILTER_SPECIFICATION
                    ),
                ),
                (
                    crate::client_error::ClientError::Problem(
                        crate::api_problem::ApiProblem::from_error(
                            crate::api_problem_error::ApiProblemError::Authentication,
                        ),
                    ),
                    constants_str::AUTHENTICATION_REQUIRED.to_owned(),
                ),
            ];
            cases
                .into_iter()
                .all(|(error, expected)| error.to_string() == expected)
        })
    );
}
#[test]
fn test_client_error_display_preserves_actual_expected_status_order_and_unexpected_response() {
    assert_eq!(
        crate::client_error::ClientError::UnexpectedResponse.to_string(),
        constants_str::SERVER_RETURNED_AN_ERROR_RESPONSE
    );
    let cases = [
        (
            crate::known_http_status::KnownHttpStatus::BadRequest,
            crate::known_http_status::KnownHttpStatus::Ok,
        ),
        (
            crate::known_http_status::KnownHttpStatus::InternalServerError,
            crate::known_http_status::KnownHttpStatus::Created,
        ),
    ];
    assert!(cases.into_iter().all(|(actual_status, expected_status)| {
        let actual = crate::transport_status::TransportStatus::from(actual_status);
        let expected = crate::transport_status::TransportStatus::from(expected_status);
        let error = crate::client_error::ClientError::Status { actual, expected };
        error.to_string()
            == format!(
                "{}{expected}, {}{actual}",
                constants_str::CLIENT_ERROR_EXPECTED_HTTP_PREFIX,
                constants_str::CLIENT_ERROR_RECEIVED_HTTP_PREFIX
            )
    }));
}
#[test]
fn test_refresh_interval_rejects_zero_and_preserves_positive_duration_boundaries() {
    assert_eq!(
        crate::auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration::try_from(
            std::time::Duration::ZERO,
        ),
        Err(crate::auth_session_keep_alive_error::AuthSessionKeepAliveError::ZeroInterval)
    );
    assert!(
        [std::time::Duration::from_nanos(1u64), std::time::Duration::MAX]
            .into_iter()
            .all(|duration| {
                crate::auth_session_refresh_interval_duration::AuthSessionRefreshIntervalDuration::try_from(duration)
                    .is_ok_and(|interval| interval.get() == duration)
            })
    );
}

#[test]
fn test_form_value_error_preserves_bounded_text_and_defaults_for_oversized_diagnostics() {
    assert!(
        [
            constants_usize::ZERO,
            constants_usize::ONE,
            constants_usize::VALUE_1_048_576
        ]
        .into_iter()
        .all(|length| {
            let diagnostic = constants_str::SLASH.repeat(length);
            crate::create_form_value_error::create_form_value_error(&diagnostic).to_string()
                == diagnostic
        })
    );
    let oversized_diagnostic =
        constants_str::SLASH.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE);
    assert_eq!(
        crate::create_form_value_error::create_form_value_error(oversized_diagnostic),
        crate::form_value_error::FormValueError::default()
    );
}

#[test]
fn test_parameterized_route_path_preserves_exact_byte_limits_and_owned_content() {
    assert!(
        String::from(crate::parameterized_route_path::ParameterizedRoutePath::default()).is_empty()
    );
    assert!(
        [
            (String::new(), 'x'),
            (constants_str::X.to_owned(), 'x'),
            (constants_str::X.repeat(8_192usize), 'x'),
            ('\u{00e9}'.to_string().repeat(4_096usize), '\u{00e9}'),
        ]
        .into_iter()
        .all(|(input, character)| {
            let length = input.len();
            let pointer = input.as_ptr();
            crate::parameterized_route_path::ParameterizedRoutePath::try_from(input).is_ok_and(
                |parameterized_route_path| {
                    let output = String::from(parameterized_route_path);
                    output.len() == length
                        && output.as_ptr() == pointer
                        && output.chars().all(|value| value == character)
                },
            )
        })
    );
    assert!([
        constants_str::X.repeat(8_193usize),
        '\u{00e9}'.to_string().repeat(4_097usize),
    ]
    .into_iter()
    .all(|input| matches!(
        crate::parameterized_route_path::ParameterizedRoutePath::try_from(input),
        Err(crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError::TooLong)
    )));
}

#[test]
fn test_problem_error_status_classification_preserves_fallback_boundaries_and_service_unavailable()
{
    [
        (100u16, false),
        (499u16, false),
        (500u16, true),
        (501u16, true),
        (502u16, true),
        (504u16, true),
        (599u16, true),
        (600u16, false),
        (999u16, false),
    ]
    .into_iter()
    .fold((), |(), (value, internal)| {
        assert!(
            crate::api_problem_status::ApiProblemStatus::try_from(value).is_ok_and(
                |api_problem_status| {
                    let expected = if internal {
                        crate::api_problem_error::ApiProblemError::Internal(api_problem_status)
                    } else {
                        crate::api_problem_error::ApiProblemError::RequestFailed(api_problem_status)
                    };
                    let actual =
                        crate::api_problem_error::ApiProblemError::from_status(api_problem_status);
                    actual == expected && actual.status() == api_problem_status
                }
            )
        );
    });
    assert!(
        crate::api_problem_status::ApiProblemStatus::try_from(503u16).is_ok_and(
            |api_problem_status| {
                let actual =
                    crate::api_problem_error::ApiProblemError::from_status(api_problem_status);
                actual == crate::api_problem_error::ApiProblemError::ServiceUnavailable
                    && actual.status() == api_problem_status
            }
        )
    );
}

#[test]
fn test_route_metadata_defaults_preserve_every_http_method() {
    let connect = constants_str::CONNECT.to_ascii_uppercase();
    [
        (crate::route_method::RouteMethod::Connect, connect.as_str()),
        (
            crate::route_method::RouteMethod::Delete,
            constants_str::DELETE,
        ),
        (crate::route_method::RouteMethod::Get, constants_str::GET),
        (crate::route_method::RouteMethod::Head, constants_str::HEAD),
        (
            crate::route_method::RouteMethod::Options,
            constants_str::OPTIONS,
        ),
        (
            crate::route_method::RouteMethod::Patch,
            constants_str::PATCH,
        ),
        (crate::route_method::RouteMethod::Post, constants_str::POST),
        (
            crate::route_method::RouteMethod::Put,
            constants_str::HTTP_METHOD_PUT_LABEL,
        ),
        (
            crate::route_method::RouteMethod::Trace,
            constants_str::TRACE,
        ),
    ]
    .into_iter()
    .fold((), |(), (route_method, method)| {
        let operation = crate::contract_str::ContractStr::from(constants_str::ROUTE_READ);
        let path = crate::contract_str::ContractStr::from(constants_str::SLASH);
        let metadata = crate::route_metadata::RouteMetadata::new(route_method, operation, path);
        assert_eq!(metadata.route_method(), route_method);
        assert_eq!(metadata.method().as_ref(), method);
        assert_eq!(metadata.openapi_operation_id(), operation);
        assert_eq!(metadata.path(), path);
        assert_eq!(
            metadata.authentication(),
            crate::authentication_requirement::AuthenticationRequirement::Public
        );
        assert!(metadata.error_statuses().is_empty());
        assert_eq!(
            metadata.mutation(),
            crate::route_mutation::RouteMutation::ReadOnly
        );
        assert_eq!(
            metadata.success_status(),
            crate::success_status::SuccessStatus::Code200
        );
        assert_eq!(metadata.access(), crate::route_access::RouteAccess::Public);
        assert_eq!(
            metadata.contract(),
            crate::route_contract::RouteContract::new(
                crate::authentication_requirement::AuthenticationRequirement::Public,
                route_method,
                crate::mutation_kind::MutationKind::ReadOnly,
                path,
                crate::success_status::SuccessStatus::Code200,
            )
        );
    });
}

#[test]
fn test_route_metadata_policy_projects_access_and_contract_for_every_authentication_mutation_status()
 {
    [
        (
            crate::authentication_requirement::AuthenticationRequirement::Public,
            crate::route_access::RouteAccess::Public,
        ),
        (
            crate::authentication_requirement::AuthenticationRequirement::Authenticated,
            crate::route_access::RouteAccess::Authenticated,
        ),
        (
            crate::authentication_requirement::AuthenticationRequirement::Rule(
                crate::contract_str::ContractStr::from(constants_str::X),
            ),
            crate::route_access::RouteAccess::Authenticated,
        ),
    ]
    .into_iter()
    .fold((), |(), (authentication_requirement, access)| {
        [
            (
                crate::route_mutation::RouteMutation::ReadOnly,
                crate::mutation_kind::MutationKind::ReadOnly,
            ),
            (
                crate::route_mutation::RouteMutation::Mutating,
                crate::mutation_kind::MutationKind::Mutating,
            ),
        ]
        .into_iter()
        .fold((), |(), (route_mutation, mutation_kind)| {
            [
                crate::success_status::SuccessStatus::Code200,
                crate::success_status::SuccessStatus::Code201,
                crate::success_status::SuccessStatus::Code204,
            ]
            .into_iter()
            .fold((), |(), success_status| {
                let operation = crate::contract_str::ContractStr::from(constants_str::ROUTE_READ);
                let path = crate::contract_str::ContractStr::from(constants_str::ROUTE);
                let metadata = crate::route_metadata::RouteMetadata::new_with_policy(
                    authentication_requirement,
                    crate::route_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
                    crate::route_method::RouteMethod::Patch,
                    route_mutation,
                    operation,
                    path,
                    success_status,
                );
                assert_eq!(metadata.authentication(), authentication_requirement);
                assert_eq!(
                    metadata.error_statuses(),
                    crate::route_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES
                );
                assert_eq!(metadata.openapi_operation_id(), operation);
                assert_eq!(metadata.path(), path);
                assert_eq!(
                    metadata.route_method(),
                    crate::route_method::RouteMethod::Patch
                );
                assert_eq!(metadata.method().as_ref(), constants_str::PATCH);
                assert_eq!(metadata.mutation(), route_mutation);
                assert_eq!(metadata.success_status(), success_status);
                assert_eq!(metadata.access(), access);
                assert_eq!(
                    metadata.contract(),
                    crate::route_contract::RouteContract::new(
                        authentication_requirement,
                        crate::route_method::RouteMethod::Patch,
                        mutation_kind,
                        path,
                        success_status
                    )
                );
            });
        });
    });
}

#[test]
fn test_route_error_status_variants_have_independent_numeric_http_codes() {
    [
        (
            crate::route_error_status::RouteErrorStatus::Authentication,
            401u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::Authorization,
            403u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::Conflict,
            409u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::Internal,
            500u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::MethodNotAllowed,
            405u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
            413u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::RateLimited,
            429u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::ServiceUnavailable,
            503u16,
        ),
        (
            crate::route_error_status::RouteErrorStatus::Validation,
            422u16,
        ),
    ]
    .into_iter()
    .fold((), |(), (route_error_status, expected)| {
        assert_eq!(u16::from(route_error_status.transport_status()), expected);
    });
}

#[test]
fn test_success_status_variants_have_independent_numeric_http_codes() {
    [
        (crate::success_status::SuccessStatus::Code200, 200u16),
        (crate::success_status::SuccessStatus::Code201, 201u16),
        (crate::success_status::SuccessStatus::Code204, 204u16),
    ]
    .into_iter()
    .fold((), |(), (success_status, expected)| {
        assert_eq!(u16::from(success_status.transport_status()), expected);
    });
}

#[test]
fn test_matching_transport_status_returns_problem_shaped_body_as_borrowed_success() {
    let problem = crate::api_problem::ApiProblem::from_error(
        crate::api_problem_error::ApiProblemError::Authentication,
    );
    assert!(serde_json::to_vec(&problem).is_ok_and(|bytes| {
        crate::transport_body::TransportBody::try_from(bytes).is_ok_and(|body| {
            let status = crate::transport_status::TransportStatus::from(
                crate::known_http_status::KnownHttpStatus::Ok,
            );
            let response = crate::transport_response::TransportResponse::new(body, status);
            crate::decode_api_problem::decode_api_problem(response.body())
                .is_some_and(|decoded| decoded == problem)
                && response
                    .success_body(status)
                    .is_ok_and(|borrowed| std::ptr::eq(borrowed, response.body()))
        })
    }));
}

#[test]
fn test_form_value_preserves_empty_ascii_and_utf8_at_exact_byte_limit() {
    let maximum = constants_usize::VALUE_1_048_576;
    assert_eq!(
        crate::form_value::FormValue::default().as_ref(),
        constants_str::EMPTY
    );
    [
        (constants_str::EMPTY.to_owned(), 'x'),
        (constants_str::X.to_owned(), 'x'),
        (constants_str::X.repeat(maximum), 'x'),
        (
            '\u{e9}'.to_string().repeat(maximum.div_euclid(2usize)),
            '\u{e9}',
        ),
    ]
    .into_iter()
    .fold((), |(), (value, expected_character)| {
        let length = value.len();
        assert!(
            crate::form_value::FormValue::try_from(value).is_ok_and(|form_value| {
                let text = form_value.as_ref();
                text.len() == length
                    && text
                        .chars()
                        .all(|character| character == expected_character)
            })
        );
    });
    [
        constants_str::X.repeat(maximum.saturating_add(1usize)),
        '\u{e9}'.to_string().repeat(maximum.div_euclid(2usize).saturating_add(1usize)),
    ].into_iter().fold((), |(), value| {
        let length = value.len();
        assert!(matches!(crate::form_value::FormValue::try_from(value), Err(crate::form_value::FormValueTryFromStringError::TooLong { len, max }) if len == length && max == maximum));
    });
}

fn field_contract_builder_fixture() -> crate::field_contract::FieldContract {
    crate::field_contract::FieldContract::new(
        crate::field_name::FieldName::from(crate::contract_str::ContractStr::from(
            constants_str::X,
        )),
        crate::field_label::FieldLabel::from(crate::contract_str::ContractStr::from(
            constants_str::FIELD,
        )),
        crate::type_contract::TypeContract::new(
            crate::input_kind::InputKind::Text,
            crate::value_format::ValueFormat::Text,
            crate::nullability::Nullability::Nullable,
        ),
    )
}

#[test]
fn test_field_contract_defaults_and_independent_capability_builders() {
    let original = field_contract_builder_fixture();
    let enabled = crate::field_capability::FieldCapability::Enabled;
    let disabled = crate::field_capability::FieldCapability::Disabled;
    let flags = |field_contract: crate::field_contract::FieldContract| {
        [
            field_contract.creatable(),
            field_contract.filterable(),
            field_contract.readable(),
            field_contract.sortable(),
            field_contract.updatable(),
        ]
    };
    assert_eq!(flags(original), [disabled; 5usize]);
    assert!(original.filters().is_empty());
    assert_eq!(
        original.order(),
        crate::field_order::FieldOrder::from(0usize)
    );
    assert_eq!(
        original.placeholder(),
        crate::field_placeholder::FieldPlaceholder::None
    );
    assert_eq!(
        original.primary_key(),
        crate::primary_key_kind::PrimaryKeyKind::NonPrimary
    );
    assert_eq!(
        original.visibility(),
        crate::field_visibility::FieldVisibility::Visible
    );
    [
        (
            original.with_creatable(enabled),
            [enabled, disabled, disabled, disabled, disabled],
        ),
        (
            original.with_filterable(enabled),
            [disabled, enabled, disabled, disabled, disabled],
        ),
        (
            original.with_readable(enabled),
            [disabled, disabled, enabled, disabled, disabled],
        ),
        (
            original.with_sortable(enabled),
            [disabled, disabled, disabled, enabled, disabled],
        ),
        (
            original.with_updatable(enabled),
            [disabled, disabled, disabled, disabled, enabled],
        ),
    ]
    .into_iter()
    .fold((), |(), (changed, expected)| {
        assert_eq!(flags(changed), expected);
        assert_eq!(changed.name(), original.name());
        assert_eq!(changed.label(), original.label());
        assert_eq!(changed.type_contract(), original.type_contract());
        assert_eq!(changed.filters(), original.filters());
        assert_eq!(changed.order(), original.order());
        assert_eq!(changed.placeholder(), original.placeholder());
        assert_eq!(changed.primary_key(), original.primary_key());
        assert_eq!(changed.visibility(), original.visibility());
        assert_eq!(
            changed
                .with_creatable(disabled)
                .with_filterable(disabled)
                .with_readable(disabled)
                .with_sortable(disabled)
                .with_updatable(disabled),
            original
        );
    });
}

#[test]
fn test_field_contract_optional_metadata_builders_preserve_fields_and_restore_defaults() {
    let original = field_contract_builder_fixture();
    let filters = crate::filter_contracts::FilterContracts::from(
        &[
            crate::filter_operation::FilterOperation::Eq,
            crate::filter_operation::FilterOperation::Regex,
        ][..],
    );
    let order = crate::field_order::FieldOrder::from(7usize);
    let placeholder = crate::field_placeholder::FieldPlaceholder::Value(
        crate::contract_str::ContractStr::from(constants_str::X),
    );
    [
        (
            original.with_filters(filters),
            filters,
            original.order(),
            original.placeholder(),
            original.primary_key(),
            original.visibility(),
        ),
        (
            original.with_order(order),
            crate::filter_contracts::FilterContracts::from(
                crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS,
            ),
            order,
            original.placeholder(),
            original.primary_key(),
            original.visibility(),
        ),
        (
            original.with_placeholder(placeholder),
            crate::filter_contracts::FilterContracts::from(
                crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS,
            ),
            original.order(),
            placeholder,
            original.primary_key(),
            original.visibility(),
        ),
        (
            original.with_primary_key(crate::primary_key_kind::PrimaryKeyKind::Primary),
            crate::filter_contracts::FilterContracts::from(
                crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS,
            ),
            original.order(),
            original.placeholder(),
            crate::primary_key_kind::PrimaryKeyKind::Primary,
            original.visibility(),
        ),
        (
            original.with_visibility(crate::field_visibility::FieldVisibility::Hidden),
            crate::filter_contracts::FilterContracts::from(
                crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS,
            ),
            original.order(),
            original.placeholder(),
            original.primary_key(),
            crate::field_visibility::FieldVisibility::Hidden,
        ),
    ]
    .into_iter()
    .fold(
        (),
        |(),
         (
            field,
            expected_filters,
            expected_order,
            expected_placeholder,
            expected_primary_key,
            expected_visibility,
        )| {
            assert_eq!(field.filters(), expected_filters.as_ref());
            assert_eq!(field.order(), expected_order);
            assert_eq!(field.placeholder(), expected_placeholder);
            assert_eq!(field.primary_key(), expected_primary_key);
            assert_eq!(field.visibility(), expected_visibility);
            assert_eq!(field.name(), original.name());
            assert_eq!(field.label(), original.label());
            assert_eq!(field.type_contract(), original.type_contract());
            assert_eq!(
                [
                    field.creatable(),
                    field.filterable(),
                    field.readable(),
                    field.sortable(),
                    field.updatable()
                ],
                [crate::field_capability::FieldCapability::Disabled; 5usize]
            );
        },
    );
    let changed = original
        .with_filters(filters)
        .with_order(order)
        .with_placeholder(placeholder)
        .with_primary_key(crate::primary_key_kind::PrimaryKeyKind::Primary)
        .with_visibility(crate::field_visibility::FieldVisibility::Hidden);
    assert_eq!(changed.filters(), filters.as_ref());
    assert_eq!(changed.order(), order);
    assert_eq!(changed.placeholder(), placeholder);
    assert_eq!(
        changed.primary_key(),
        crate::primary_key_kind::PrimaryKeyKind::Primary
    );
    assert_eq!(
        changed.visibility(),
        crate::field_visibility::FieldVisibility::Hidden
    );
    assert_eq!(changed.name(), original.name());
    assert_eq!(changed.label(), original.label());
    assert_eq!(changed.type_contract(), original.type_contract());
    assert_eq!(
        [
            changed.creatable(),
            changed.filterable(),
            changed.readable(),
            changed.sortable(),
            changed.updatable()
        ],
        [crate::field_capability::FieldCapability::Disabled; 5usize]
    );
    assert_eq!(
        changed
            .with_filters(crate::filter_contracts::FilterContracts::from(
                crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS
            ))
            .with_order(original.order())
            .with_placeholder(original.placeholder())
            .with_primary_key(original.primary_key())
            .with_visibility(original.visibility()),
        original
    );
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DefaultMetadataProjectionContractRouteFamily;
impl crate::route_family::RouteFamily for DefaultMetadataProjectionContractRouteFamily {
    const ROUTE_COUNT: usize = 2usize;
    fn coverage_descriptors() -> crate::route_coverage_descriptors::RouteCoverageDescriptors {
        crate::route_coverage_descriptors::RouteCoverageDescriptors::from_max_iter([
            (crate::route_method::RouteMethod::Post, constants_str::X),
            (crate::route_method::RouteMethod::Get, constants_str::ROUTE_READ),
        ].into_iter().map(|(route_method, operation)| {
            crate::route_coverage_descriptor::RouteCoverageDescriptor::new(
                crate::route_metadata::RouteMetadata::new(route_method, crate::contract_str::ContractStr::from(operation), crate::contract_str::ContractStr::from(constants_str::ROUTE)),
                crate::route_access::RouteAccess::Public,
                crate::route_mutation::RouteMutation::ReadOnly,
                crate::route_coverage_evidence::RouteCoverageEvidence::new(&[crate::route_coverage_obligation::RouteCoverageObligation::IntegrationFixture, crate::route_coverage_obligation::RouteCoverageObligation::OpenApiOperation, crate::route_coverage_obligation::RouteCoverageObligation::PayloadValidation]),
            )
        }))
    }
}

#[test]
fn test_route_family_default_metadata_projects_nonempty_descriptors_in_order() {
    let descriptors = <DefaultMetadataProjectionContractRouteFamily as crate::route_family::RouteFamily>::coverage_descriptors();
    let metadata = <DefaultMetadataProjectionContractRouteFamily as crate::route_family::RouteFamily>::route_metadata();
    assert_eq!(metadata.as_ref().len(), 2usize);
    assert_eq!(metadata.as_ref().len(), descriptors.as_ref().len());
    assert!(
        metadata
            .as_ref()
            .iter()
            .zip(descriptors.as_ref())
            .all(|(actual, descriptor)| actual == descriptor.get_metadata())
    );
    assert_eq!(
        metadata.as_ref().first().map(|route| route.route_method()),
        Some(crate::route_method::RouteMethod::Post)
    );
    assert_eq!(
        metadata.as_ref().last().map(|route| route.route_method()),
        Some(crate::route_method::RouteMethod::Get)
    );
    assert_eq!(
        crate::validate_route_coverage::validate_route_coverage(descriptors.as_ref()),
        Ok(())
    );
}

#[test]
fn test_filter_operation_value_shapes_match_independent_contract_matrix() {
    let cases = [
        (
            crate::filter_operation::FilterOperation::Between,
            crate::filter_value_shape::FilterValueShape::Range,
        ),
        (
            crate::filter_operation::FilterOperation::CurrentDate,
            crate::filter_value_shape::FilterValueShape::None,
        ),
        (
            crate::filter_operation::FilterOperation::CurrentTime,
            crate::filter_value_shape::FilterValueShape::None,
        ),
        (
            crate::filter_operation::FilterOperation::CurrentTimestamp,
            crate::filter_value_shape::FilterValueShape::None,
        ),
        (
            crate::filter_operation::FilterOperation::GreaterThanCurrentDate,
            crate::filter_value_shape::FilterValueShape::None,
        ),
        (
            crate::filter_operation::FilterOperation::GreaterThanCurrentTime,
            crate::filter_value_shape::FilterValueShape::None,
        ),
        (
            crate::filter_operation::FilterOperation::GreaterThanCurrentTimestamp,
            crate::filter_value_shape::FilterValueShape::None,
        ),
        (
            crate::filter_operation::FilterOperation::EqToEncodedStringRepresentation,
            crate::filter_value_shape::FilterValueShape::EncodedText,
        ),
        (
            crate::filter_operation::FilterOperation::In,
            crate::filter_value_shape::FilterValueShape::List,
        ),
        (
            crate::filter_operation::FilterOperation::Regex,
            crate::filter_value_shape::FilterValueShape::Regex,
        ),
        (
            crate::filter_operation::FilterOperation::AdjacentWithRange,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::Before,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::Eq,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::ExcludedUpperBound,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::FindRangesThatFullyContainTheGivenRange,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::FindRangesWithinGivenRange,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::GreaterThan,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::GreaterThanExcludedUpperBound,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::GreaterThanIncludedLowerBound,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::IncludedLowerBound,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::OverlapWithRange,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::RangeLen,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::StrictlyToLeftOfRange,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
        (
            crate::filter_operation::FilterOperation::StrictlyToRightOfRange,
            crate::filter_value_shape::FilterValueShape::Scalar,
        ),
    ];
    assert!(
        cases
            .into_iter()
            .all(
                |(filter_operation, filter_value_shape)| filter_operation.value_shape()
                    == filter_value_shape
            )
    );
}

#[test]
fn test_required_route_categories_cover_every_capability_combination_in_order() {
    let cases = [
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::None,
                crate::route_json_body_usage::RouteJsonBodyUsage::None,
                crate::route_response_kind::RouteResponseKind::Buffered,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::None,
                crate::route_json_body_usage::RouteJsonBodyUsage::None,
                crate::route_response_kind::RouteResponseKind::Streaming,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::None,
                crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody,
                crate::route_response_kind::RouteResponseKind::Buffered,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::None,
                crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody,
                crate::route_response_kind::RouteResponseKind::Streaming,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::Database,
                crate::route_json_body_usage::RouteJsonBodyUsage::None,
                crate::route_response_kind::RouteResponseKind::Buffered,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::Database,
                crate::route_json_body_usage::RouteJsonBodyUsage::None,
                crate::route_response_kind::RouteResponseKind::Streaming,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::Database,
                crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody,
                crate::route_response_kind::RouteResponseKind::Buffered,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
            ][..],
        ),
        (
            crate::route_test_capabilities::RouteTestCapabilities::new(
                crate::route_database_usage::RouteDatabaseUsage::Database,
                crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody,
                crate::route_response_kind::RouteResponseKind::Streaming,
            ),
            &[
                crate::route_test_category::RouteTestCategory::FixtureHook,
                crate::route_test_category::RouteTestCategory::Metadata,
                crate::route_test_category::RouteTestCategory::DatabaseFixture,
                crate::route_test_category::RouteTestCategory::JsonRoundTrip,
                crate::route_test_category::RouteTestCategory::StreamingResponse,
            ][..],
        ),
    ];
    assert!(
        cases
            .into_iter()
            .all(|(route_test_capabilities, expected_categories)| {
                bounded_types::bounded_vec::BoundedVec::from(
                    crate::required_test_categories::required_test_categories(
                        route_test_capabilities,
                    ),
                )
                .into_inner()
                    == expected_categories
            })
    );
}

#[test]
fn test_field_collections_preserve_nonempty_order_duplicates_and_empty_iterators() {
    let values = [
        field_contract_builder_fixture().with_order(crate::field_order::FieldOrder::from(2usize)),
        field_contract_builder_fixture().with_order(crate::field_order::FieldOrder::from(1usize)),
        field_contract_builder_fixture().with_order(crate::field_order::FieldOrder::from(2usize)),
    ];
    let checked = crate::field_contracts::FieldContracts::try_from(values.to_vec());
    assert!(checked.is_ok_and(|field_contracts| {
        let iterated =
            crate::field_contracts::FieldContracts::from_max_iter(values.iter().copied());
        field_contracts.as_ref() == values && iterated.as_ref() == values
    }));
    assert!(
        crate::field_contracts::FieldContracts::from_max_iter(std::iter::empty())
            .as_ref()
            .is_empty()
    );
}

#[test]
fn test_contract_static_text_preserves_borrowed_display_and_owned_content() {
    assert!(
        [constants_str::EMPTY, constants_str::X, constants_str::ROUTE]
            .into_iter()
            .all(|text| {
                let contract_str = crate::contract_str::ContractStr::from(text);
                assert!(std::ptr::eq(contract_str.as_ref().as_ptr(), text.as_ptr()));
                contract_str.as_ref() == text
                    && contract_str.to_string() == text
                    && String::from(contract_str) == text
            })
    );
}

#[test]
fn test_problem_violation_lists_preserve_order_at_valid_count_boundaries() {
    [0usize, 1usize, 128usize]
        .into_iter()
        .fold((), |(), count| {
            let payload = serde_json::Value::Array(
                (0usize..count)
                    .map(|index| {
                        serde_json::json!({
                            (stringify!(detail)): format!("{}{index}", constants_str::X),
                            (stringify!(field)): index.to_string(),
                        })
                    })
                    .collect(),
            );
            assert!(serde_json::to_vec(&payload).is_ok_and(|bytes| {
                serde_json::from_slice::<crate::api_problem_violations::ApiProblemViolations>(
                    &bytes,
                )
                .is_ok_and(|violations| {
                    serde_json::to_value(violations).is_ok_and(|encoded| encoded == payload)
                })
            }));
        });
}

#[test]
fn test_problem_violation_text_preserves_exact_utf8_limits_and_rejects_overflow() {
    [
        (
            constants_str::X.repeat(1_024usize),
            constants_str::X.repeat(128usize),
        ),
        (
            char::MAX.to_string().repeat(256usize),
            char::MAX.to_string().repeat(32usize),
        ),
    ]
    .into_iter()
    .fold((), |(), (detail, field)| {
        let oversized_detail = format!("{detail}{}", constants_str::X);
        let oversized_field = format!("{field}{}", constants_str::X);
        let payload = |detail_text, field_text| {
            serde_json::json!({
                (stringify!(detail)): detail_text,
                (stringify!(field)): field_text,
            })
        };
        let valid = payload(&detail, &field);
        assert!(serde_json::to_vec(&valid).is_ok_and(|bytes| {
            serde_json::from_slice::<crate::api_problem_violation::ApiProblemViolation>(&bytes)
                .is_ok_and(|violation| {
                    serde_json::to_value(violation).is_ok_and(|encoded| encoded == valid)
                })
        }));
        [
            payload(&oversized_detail, &field),
            payload(&detail, &oversized_field),
        ]
        .into_iter()
        .fold((), |(), invalid| {
            assert!(
                serde_json::from_value::<crate::api_problem_violation::ApiProblemViolation>(
                    invalid
                )
                .is_err_and(|error| error.is_data())
            );
        });
    });
}

#[test]
fn test_problem_decoder_preserves_optional_request_ids_at_utf8_boundaries() {
    let original = crate::api_problem::ApiProblem::from_error(
        crate::api_problem_error::ApiProblemError::Authentication,
    );
    [
        serde_json::Value::Null,
        serde_json::Value::from(constants_str::EMPTY),
        serde_json::Value::from(constants_str::X.repeat(128usize)),
        serde_json::Value::from(char::MAX.to_string().repeat(32usize)),
    ]
    .into_iter()
    .fold((), |(), request_id| {
        assert!(serde_json::to_value(&original).is_ok_and(|mut payload| {
            let Some(request_id_value) = payload.get_mut(stringify!(request_id)) else {
                return false;
            };
            *request_id_value = request_id;
            serde_json::to_vec(&payload).is_ok_and(|bytes| {
                crate::transport_body::TransportBody::try_from(bytes).is_ok_and(|transport_body| {
                    crate::decode_api_problem::decode_api_problem(&transport_body).is_some_and(
                        |problem| {
                            serde_json::to_value(problem).is_ok_and(|encoded| encoded == payload)
                        },
                    )
                })
            })
        }));
    });
    [
        serde_json::Value::from(constants_str::X.repeat(129usize)),
        serde_json::Value::from(1u64),
    ]
    .into_iter()
    .fold((), |(), invalid_request_id| {
        assert!(serde_json::to_value(&original).is_ok_and(|mut payload| {
            let Some(request_id_value) = payload.get_mut(stringify!(request_id)) else {
                return false;
            };
            *request_id_value = invalid_request_id;
            serde_json::to_vec(&payload).is_ok_and(|bytes| {
                crate::transport_body::TransportBody::try_from(bytes).is_ok_and(|transport_body| {
                    crate::decode_api_problem::decode_api_problem(&transport_body).is_none()
                })
            })
        }));
    });
}

#[test]
fn test_filter_wire_json_preserves_exact_encoded_text_limit_and_rejects_overflow() {
    let maximum = constants_usize::VALUE_1_048_576;
    [
        constants_str::X.repeat(maximum.saturating_sub(2usize)),
        format!(
            "{}{}",
            char::MAX.to_string().repeat(262_143usize),
            constants_str::X.repeat(2usize)
        ),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(serde_json::to_string(&text).is_ok_and(|encoded| {
            assert_eq!(encoded.len(), maximum);
            crate::filter_wire_json::FilterWireJson::try_from(encoded).is_ok_and(
                |filter_wire_json| {
                    serde_json::from_str::<String>(filter_wire_json.as_ref())
                        .is_ok_and(|decoded| decoded == text)
                },
            )
        }));
    });
    assert!(serde_json::to_string(&constants_str::X.repeat(maximum.saturating_sub(1usize))).is_ok_and(|encoded| {
        matches!(crate::filter_wire_json::FilterWireJson::try_from(encoded), Err(crate::filter_wire_json::FilterWireJsonTryFromStringError::TooLong { len, max }) if len == maximum + 1usize && max == maximum)
    }));
}

#[test]
fn test_page_contract_preserves_collections_and_distinct_path_title_fields() {
    let route_contract = crate::route_contract::RouteContract::new(
        crate::authentication_requirement::AuthenticationRequirement::Public,
        crate::route_method::RouteMethod::Get,
        crate::mutation_kind::MutationKind::ReadOnly,
        crate::contract_str::ContractStr::from(constants_str::X),
        crate::success_status::SuccessStatus::Code200,
    );
    let action_contract = crate::action_contract::ActionContract::new(
        crate::operation_kind::OperationKind::Read,
        route_contract,
    );
    let field_contract = field_contract_builder_fixture();
    let path = crate::contract_str::ContractStr::from(constants_str::PATH_ALT_5);
    let title = crate::contract_str::ContractStr::from(constants_str::NAME);
    let page_contract = crate::page_contract::PageContract::new(
        crate::action_contracts::ActionContracts::from_max_iter([action_contract]),
        crate::field_contracts::FieldContracts::from_max_iter([field_contract]),
        path,
        crate::route_contracts::RouteContracts::from_max_iter([route_contract]),
        title,
    );
    assert_eq!(page_contract.actions().as_ref(), [action_contract]);
    assert_eq!(page_contract.fields().as_ref(), [field_contract]);
    assert_eq!(page_contract.routes().as_ref(), [route_contract]);
    assert_eq!(page_contract.path(), path);
    assert_eq!(page_contract.title(), title);
}

#[test]
fn test_input_kind_and_filter_shape_wire_values_preserve_every_variant() {
    [
        (crate::input_kind::InputKind::Checkbox, stringify!(checkbox)),
        (crate::input_kind::InputKind::Date, stringify!(date)),
        (
            crate::input_kind::InputKind::DateTime,
            stringify!(date_time),
        ),
        (crate::input_kind::InputKind::Number, stringify!(number)),
        (crate::input_kind::InputKind::Text, stringify!(text)),
        (crate::input_kind::InputKind::Time, stringify!(time)),
        (crate::input_kind::InputKind::Uuid, stringify!(uuid)),
    ]
    .into_iter()
    .fold((), |(), (input_kind, expected)| {
        assert!(serde_json::to_value(input_kind).is_ok_and(|json| {
            json == expected
                && serde_json::from_value::<crate::input_kind::InputKind>(json)
                    .is_ok_and(|decoded| decoded == input_kind)
        }));
    });
    [
        (
            crate::filter_value_shape::FilterValueShape::EncodedText,
            stringify!(encoded_text),
        ),
        (
            crate::filter_value_shape::FilterValueShape::List,
            stringify!(list),
        ),
        (
            crate::filter_value_shape::FilterValueShape::None,
            stringify!(none),
        ),
        (
            crate::filter_value_shape::FilterValueShape::Range,
            stringify!(range),
        ),
        (
            crate::filter_value_shape::FilterValueShape::Regex,
            stringify!(regex),
        ),
        (
            crate::filter_value_shape::FilterValueShape::Scalar,
            stringify!(scalar),
        ),
    ]
    .into_iter()
    .fold((), |(), (filter_value_shape, expected)| {
        assert!(serde_json::to_value(filter_value_shape).is_ok_and(|json| {
            json == expected
                && serde_json::from_value::<crate::filter_value_shape::FilterValueShape>(json)
                    .is_ok_and(|decoded| decoded == filter_value_shape)
        }));
    });
}
