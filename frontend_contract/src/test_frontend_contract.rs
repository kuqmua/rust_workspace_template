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
