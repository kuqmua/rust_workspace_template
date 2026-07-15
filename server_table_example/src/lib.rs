#![allow(clippy::needless_for_each)] // utoipa 4 OpenApi derive expands iterator callbacks at crate scope
#![cfg_attr(not(test), allow(unused_crate_dependencies))] // HTTP contract fixtures use server_app_state, tokio, and tower only in this crate's test target
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // utoipa 4 derives component registration with iterator callbacks
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "cm_max_items": 2,
    "create_exclude_fields": ["revision"],
    "idempotent_mutations": true,
    "optimistic_revision_field": "revision",
    "permission_prefix": "table_example",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False",
    "um_max_items": 2
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
pub struct TableExample {
    #[generate_pg_table_primary_key]
    #[generate_pg_table_frontend(label = "Identifier", order = 3, sortable)]
    pub primary_key_column: pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
    #[generate_pg_table_frontend(label = "Small number", order = 0, sortable)]
    pub column_0: pg_types_numeric::I16AsNonNullInt2,
    #[generate_pg_table_frontend(filterable, order = 1, placeholder = "Optional number")]
    pub column_1: pg_types_numeric::OptionalI16AsNullableInt2,
    #[generate_pg_table_frontend(hidden, order = 2)]
    pub column_2: pg_types_numeric::I32AsNonNullInt4,
    #[generate_pg_table_frontend(hidden, order = 4)]
    pub revision: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
}
#[cfg(test)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::indexing_slicing,
    clippy::needless_for_each,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::while_let_loop
)] // generated HTTP matrix and compact recursive JSON assertions stay grouped by the contract they verify
mod tests {
    #[tokio::test]
    async fn generated_negative_http_contracts_reject_before_database_io() {
        let app_state = std::sync::Arc::new(server_app_state::mk_test_server_app_state());
        let router = super::TableExample::routes(app_state);
        let mut contracts = super::TableExampleRouteContract::ALL.into_iter();
        loop {
            let Some(contract) = contracts.next() else {
                break;
            };
            let method = match contract.http_method() {
                super::TableExampleHttpMethod::Delete => http::Method::DELETE,
                super::TableExampleHttpMethod::Patch => http::Method::PATCH,
                super::TableExampleHttpMethod::Post => http::Method::POST,
            };
            let response = tower::ServiceExt::oneshot(
                router.clone(),
                http::Request::builder()
                    .method(method)
                    .uri(contract.path())
                    .header(http::header::CONTENT_TYPE, str_constants::text::TEXT_PLAIN)
                    .body(axum::body::Body::from(str_constants::text::TEXT_ALT_14))
                    .expect("dc39ba13"),
            )
            .await
            .expect("aeb6ad70");
            assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
            assert_eq!(
                response.headers().get(http::header::CONTENT_TYPE),
                Some(&http::HeaderValue::from_static("application/problem+json"))
            );
            let body = axum::body::to_bytes(response.into_body(), 4_096usize)
                .await
                .expect("bf4bcc30");
            let problem =
                serde_json::from_slice::<frontend_contract::ApiProblem>(&body).expect("8da011ba");
            assert_eq!(u16::from(problem.status()), 400u16);
        }
        let cm_payload: super::TableExampleCmPayload =
            pg_crud_common::DefaultSomeOneElement::default_some_one_element();
        let valid_body = serde_json::to_vec(&cm_payload).expect("29fc2f21");
        let missing_key = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::POST)
                .uri(str_constants::text::TABLE_EXAMPLE_CM)
                .header(
                    http::header::CONTENT_TYPE,
                    str_constants::text::APPLICATION_JSON,
                )
                .body(axum::body::Body::from(valid_body.clone()))
                .expect("d02ba9f0"),
        )
        .await
        .expect("81f86e3f");
        assert_eq!(missing_key.status(), http::StatusCode::BAD_REQUEST);
        let wrong_content_type = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::POST)
                .uri(str_constants::text::TABLE_EXAMPLE_CM)
                .header(http::header::CONTENT_TYPE, str_constants::text::TEXT_PLAIN)
                .header(
                    str_constants::text::IDEMPOTENCY_KEY_ALT,
                    str_constants::text::NEGATIVE_CONTENT_TYPE,
                )
                .body(axum::body::Body::from(valid_body.clone()))
                .expect("2f6ee062"),
        )
        .await
        .expect("503936ec");
        assert_eq!(wrong_content_type.status(), http::StatusCode::BAD_REQUEST);
        let malformed = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::POST)
                .uri(str_constants::text::TABLE_EXAMPLE_CM)
                .header(
                    http::header::CONTENT_TYPE,
                    str_constants::text::APPLICATION_JSON,
                )
                .header(
                    str_constants::text::IDEMPOTENCY_KEY_ALT,
                    str_constants::text::NEGATIVE_MALFORMED,
                )
                .body(axum::body::Body::from(str_constants::text::TEXT_ALT_13))
                .expect("21af9e85"),
        )
        .await
        .expect("cc0e9ff2");
        assert_eq!(malformed.status(), http::StatusCode::BAD_REQUEST);
        assert_eq!(
            malformed.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static("application/problem+json"))
        );
        let malformed_body = axum::body::to_bytes(malformed.into_body(), 4_096usize)
            .await
            .expect("7ae01090");
        let malformed_problem =
            serde_json::from_slice::<frontend_contract::ApiProblem>(&malformed_body)
                .expect("56e16453");
        assert_eq!(
            malformed_problem.kind(),
            frontend_contract::ApiProblemKind::InvalidRequest
        );
        let duplicate_key = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::POST)
                .uri(str_constants::text::TABLE_EXAMPLE_CM)
                .header(
                    http::header::CONTENT_TYPE,
                    str_constants::text::APPLICATION_JSON,
                )
                .header(
                    str_constants::text::IDEMPOTENCY_KEY_ALT,
                    str_constants::text::DUPLICATE_A,
                )
                .header(
                    str_constants::text::IDEMPOTENCY_KEY_ALT,
                    str_constants::text::DUPLICATE_B,
                )
                .body(axum::body::Body::from(valid_body))
                .expect("aa9ff040"),
        )
        .await
        .expect("f1a92b49");
        assert_eq!(duplicate_key.status(), http::StatusCode::BAD_REQUEST);
        let missing_revision = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::PATCH)
                .uri(str_constants::text::TABLE_EXAMPLE_UO)
                .header(
                    http::header::CONTENT_TYPE,
                    str_constants::text::APPLICATION_JSON,
                )
                .header(
                    str_constants::text::IDEMPOTENCY_KEY_ALT,
                    str_constants::text::MISSING_REVISION,
                )
                .body(axum::body::Body::from(str_constants::text::TEXT_ALT_14))
                .expect("19855efd"),
        )
        .await
        .expect("230693f3");
        assert_eq!(
            missing_revision.status(),
            http::StatusCode::PRECONDITION_REQUIRED
        );
        let oversized = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::POST)
                .uri(str_constants::text::TABLE_EXAMPLE_CM)
                .header(
                    http::header::CONTENT_TYPE,
                    str_constants::text::APPLICATION_JSON,
                )
                .header(
                    str_constants::text::IDEMPOTENCY_KEY_ALT,
                    str_constants::text::NEGATIVE_OVERSIZED,
                )
                .body(axum::body::Body::from(
                    str_constants::text::X.repeat(2_048usize),
                ))
                .expect("aed15d30"),
        )
        .await
        .expect("0d8df630");
        assert_eq!(oversized.status(), http::StatusCode::PAYLOAD_TOO_LARGE);
        let unsupported_method = tower::ServiceExt::oneshot(
            router,
            http::Request::builder()
                .method(http::Method::GET)
                .uri(str_constants::text::TABLE_EXAMPLE_CM)
                .body(axum::body::Body::empty())
                .expect("76f6f737"),
        )
        .await
        .expect("fb5aee1d");
        assert_eq!(
            unsupported_method.status(),
            http::StatusCode::METHOD_NOT_ALLOWED
        );
    }
    #[derive(Clone, Copy, Debug)]
    struct TestTransport;
    impl frontend_contract::Transport for TestTransport {
        fn send(
            &self,
            _request: frontend_contract::TransportRequest,
        ) -> std::pin::Pin<
            Box<
                dyn Future<
                        Output = Result<
                            frontend_contract::TransportResponse,
                            frontend_contract::TransportError,
                        >,
                    > + '_,
            >,
        > {
            Box::pin(async {
                Ok(frontend_contract::TransportResponse::new(
                    frontend_contract::TransportBody::from(Vec::new()),
                    frontend_contract::TransportStatus::from(200u16),
                ))
            })
        }
    }
    fn collect_component_refs(
        value: &serde_json::Value,
        refs: &mut std::collections::BTreeSet<String>,
    ) {
        match value {
            serde_json::Value::Array(values) => values
                .iter()
                .for_each(|value| collect_component_refs(value, refs)),
            serde_json::Value::Object(values) => values.iter().for_each(|(key, value)| {
                if key == str_constants::text::DOLLAR_REF
                    && let Some(name) = value.as_str().and_then(|value| {
                        value.strip_prefix(str_constants::text::COMPONENTS_SCHEMAS)
                    })
                {
                    let _inserted = refs.insert(name.to_owned());
                }
                collect_component_refs(value, refs);
            }),
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    #[test]
    fn generated_open_api_contains_all_crud_paths() {
        let doc = serde_json::to_value(super::TableExampleOpenApi::open_api()).expect("3176b0d5");
        [
            (str_constants::text::CM, str_constants::text::POST_ALT),
            (str_constants::text::CO, str_constants::text::POST_ALT),
            (str_constants::text::RM, str_constants::text::POST_ALT),
            (str_constants::text::RO, str_constants::text::POST_ALT),
            (str_constants::text::UO, str_constants::text::PATCH_ALT),
            (
                str_constants::text::DM,
                str_constants::pg_crud::DELETE_PERMISSION_ACTION,
            ),
            (
                str_constants::text::DLO,
                str_constants::pg_crud::DELETE_PERMISSION_ACTION,
            ),
        ]
        .into_iter()
        .for_each(|(operation, method)| {
            let operation_doc = doc
                .pointer(&format!(
                    "{}{operation}/{method}",
                    str_constants::test_values::OPEN_API_TABLE_EXAMPLE_PATH_PREFIX
                ))
                .expect("8ba5f1e7");
            assert_eq!(operation_doc["operationId"], operation);
            assert!(operation_doc["responses"].get("400").is_some());
            assert!(operation_doc["responses"].get("413").is_some());
            assert!(operation_doc["responses"].get("401").is_some());
            assert!(operation_doc["responses"].get("403").is_some());
            assert!(operation_doc["responses"].get("409").is_some());
            assert!(operation_doc["responses"].get("422").is_some());
            assert!(operation_doc["responses"].get("429").is_some());
            assert!(operation_doc["responses"].get("500").is_some());
            assert_eq!(
                operation_doc["security"][0]["admin_cookie"],
                serde_json::json!([])
            );
            assert_eq!(
                operation_doc["security"][1]["admin_csrf"],
                serde_json::json!([])
            );
            assert!(operation_doc["responses"].get("default").is_none());
            let success_status =
                if operation == str_constants::text::CM || operation == str_constants::text::CO {
                    str_constants::text::VALUE_201
                } else {
                    str_constants::text::VALUE_200
                };
            assert!(operation_doc["responses"].get(success_status).is_some());
            operation_doc[str_constants::text::RESPONSES]
                .as_object()
                .expect("7c9b7f2b")
                .iter()
                .filter(|(status, _response)| status.as_str() != success_status)
                .for_each(|(_status, response)| {
                    assert_eq!(
                        response.pointer("/content/application~1json/schema/$ref"),
                        Some(&serde_json::Value::String(
                            "#/components/schemas/frontend_contract.ApiProblem".to_owned()
                        ))
                    );
                });
            if operation == str_constants::text::UO {
                assert!(operation_doc["responses"].get("412").is_some());
                assert!(operation_doc["responses"].get("428").is_some());
                assert!(
                    operation_doc["parameters"]
                        .as_array()
                        .is_some_and(|parameters| parameters
                            .iter()
                            .any(|parameter| parameter["name"] == "If-Match"))
                );
            }
        });
        assert!(
            doc.pointer(&format!(
                "{}um",
                str_constants::test_values::OPEN_API_TABLE_EXAMPLE_PATH_PREFIX
            ))
            .is_none()
        );
        let schemas = doc[str_constants::text::COMPONENTS][str_constants::text::SCHEMAS]
            .as_object()
            .expect("95ec6823");
        assert_eq!(
            doc["components"]["securitySchemes"]["admin_cookie"]["in"],
            "cookie"
        );
        assert_eq!(
            doc["components"]["securitySchemes"]["admin_csrf"]["in"],
            "header"
        );
        assert!(!schemas.is_empty());
        let mut refs = std::collections::BTreeSet::new();
        collect_component_refs(&doc, &mut refs);
        let missing_refs = refs
            .iter()
            .filter(|name| !schemas.contains_key(*name))
            .collect::<Vec<_>>();
        assert!(
            missing_refs.is_empty(),
            "missing component schemas: {missing_refs:?}"
        );
        assert!(
            schemas["TableExampleCreate"]["properties"]
                .get("primary_key_column")
                .is_none()
        );
        [str_constants::text::TABLEEXAMPLEREAD, str_constants::text::TABLEEXAMPLEUPDATE]
            .into_iter()
            .for_each(|schema_name| {
                let nullable_projection_or_patch = &schemas[schema_name][str_constants::text::PROPERTIES][str_constants::text::COLUMN_1];
                assert_ne!(nullable_projection_or_patch["nullable"], true);
                assert!(
                    nullable_projection_or_patch.pointer("/properties/v").is_some()
                        || nullable_projection_or_patch
                            .pointer("/allOf/0/properties/v")
                            .is_some(),
                    "projection or patch schema is not an inline value object: {nullable_projection_or_patch}"
                );
                assert!(!schemas[schema_name]["required"]
                    .as_array()
                    .is_some_and(|required| required.iter().any(|field| field == "column_1")));
            });
        [
            str_constants::text::PG_CRUD_COMMON_PGTYPE_READ,
            str_constants::text::PG_CRUD_COMMON_PGTYPE_SELECT,
            str_constants::text::WHERE_FILTERS_PGTYPEWHEREEQ,
            str_constants::text::WHERE_FILTERS_PGTYPEWHEREBETWEEN,
            str_constants::text::WHERE_FILTERS_PGTYPEWHEREGREATERTHAN,
            str_constants::text::WHERE_FILTERS_PGTYPEWHEREIN,
        ]
        .into_iter()
        .for_each(|schema_name| {
            assert!(
                schemas[schema_name]["oneOf"]
                    .as_array()
                    .is_some_and(|items| !items.is_empty())
            );
        });
    }
    #[test]
    fn generated_route_permissions_follow_operation_semantics() {
        super::TableExampleRouteContract::ALL
            .into_iter()
            .for_each(|contract| {
                let expected = match contract.operation() {
                    super::TableExampleOperation::Cm | super::TableExampleOperation::Co => {
                        str_constants::text::TABLE_EXAMPLE_CREATE
                    }
                    super::TableExampleOperation::Rm | super::TableExampleOperation::Ro => {
                        str_constants::text::TABLE_EXAMPLE_READ
                    }
                    super::TableExampleOperation::Um | super::TableExampleOperation::Uo => {
                        str_constants::text::TABLE_EXAMPLE_UPDATE
                    }
                    super::TableExampleOperation::Dlo | super::TableExampleOperation::Dm => {
                        str_constants::text::TABLE_EXAMPLE_DELETE
                    }
                };
                assert_eq!(
                    contract.authentication(),
                    super::TableExampleAuthenticationRequirement::Permission(expected)
                );
            });
    }
    #[test]
    fn generated_frontend_fields_follow_table_and_crud_contract() {
        let fields = super::TableExample::frontend_fields();
        assert_eq!(fields.as_ref().len(), 5usize);
        let first = fields.as_ref().first().expect("fead1583");
        assert_eq!(first.name().as_ref(), "column_0");
        assert_eq!(first.label().as_ref(), "Small number");
        assert_eq!(
            first.sortable(),
            frontend_contract::FieldCapability::Enabled
        );
        let primary_key = fields
            .as_ref()
            .iter()
            .find(|field| field.primary_key() == frontend_contract::PrimaryKeyKind::Primary)
            .expect("0a4fe013");
        assert_eq!(primary_key.name().as_ref(), "primary_key_column");
        assert_eq!(primary_key.label().as_ref(), "Identifier");
        assert_eq!(
            primary_key.primary_key(),
            frontend_contract::PrimaryKeyKind::Primary
        );
        assert_eq!(
            primary_key.creatable(),
            frontend_contract::FieldCapability::Disabled
        );
        assert_eq!(
            primary_key.readable(),
            frontend_contract::FieldCapability::Enabled
        );
        let nullable = fields.as_ref().get(1usize).expect("0cb93d7f");
        assert_eq!(nullable.name().as_ref(), "column_1");
        assert_eq!(
            nullable.type_contract().nullability(),
            frontend_contract::Nullability::Nullable
        );
        assert_eq!(
            nullable.updatable(),
            frontend_contract::FieldCapability::Enabled
        );
        assert_eq!(
            nullable.filterable(),
            frontend_contract::FieldCapability::Enabled
        );
        assert_eq!(
            nullable.placeholder(),
            frontend_contract::FieldPlaceholder::Value(frontend_contract::ContractStr::from(
                "Optional number"
            ))
        );
        let hidden = fields.as_ref().get(2usize).expect("0685ff24");
        assert_eq!(
            hidden.visibility(),
            frontend_contract::FieldVisibility::Hidden
        );
    }
    #[test]
    fn generated_frontend_routes_share_operation_model() {
        let routes = super::TableExampleRouteContract::frontend_contracts();
        assert_eq!(
            routes.as_ref().len(),
            super::TableExampleRouteContract::ALL.len()
        );
        assert!(routes.as_ref().iter().all(|route| matches!(
            route.authentication(),
            frontend_contract::AuthenticationRequirement::Permission(_)
        )));
        assert!(
            routes
                .as_ref()
                .iter()
                .any(|route| route.mutation() == frontend_contract::MutationKind::Mutating)
        );
        assert!(
            routes
                .as_ref()
                .iter()
                .any(|route| route.mutation() == frontend_contract::MutationKind::ReadOnly)
        );
    }
    #[test]
    fn generated_frontend_page_uses_routes_fields_and_actions() {
        let page = super::TableExample::frontend_page();
        assert_eq!(page.path().as_ref(), "/table_example");
        assert_eq!(page.title().as_ref(), "Table Example");
        assert_eq!(page.fields().as_ref().len(), 5usize);
        assert_eq!(
            page.actions().as_ref().len(),
            super::TableExampleRouteContract::ALL.len()
        );
        assert!(page.actions().as_ref().iter().any(|action| {
            action.operation() == frontend_contract::OperationKind::DeleteOne
                && action.confirmation() == frontend_contract::ConfirmationRequirement::Required
        }));
    }
    #[test]
    fn generated_frontend_forms_parse_typed_payloads_and_report_field() {
        let create = super::TableExampleCreate::try_from(super::TableExampleCreateForm {
            column_0: frontend_contract::FormValue::try_from(
                str_constants::text::VALUE_12.to_owned(),
            )
            .expect("8f6b2f31"),
            column_1: frontend_contract::FormValue::try_from(String::new()).expect("274d2e0c"),
            column_2: frontend_contract::FormValue::try_from(
                str_constants::text::VALUE_34.to_owned(),
            )
            .expect("98c9cd5e"),
        });
        let _create = create.expect("af5a7ec4");
        let error = super::TableExampleCreate::try_from(super::TableExampleCreateForm {
            column_0: frontend_contract::FormValue::try_from(
                str_constants::text::NOT_A_NUMBER.to_owned(),
            )
            .expect("a6413c9d"),
            column_1: frontend_contract::FormValue::try_from(String::new()).expect("1970fd5b"),
            column_2: frontend_contract::FormValue::try_from(
                str_constants::text::VALUE_34.to_owned(),
            )
            .expect("fd5e40c9"),
        })
        .expect_err(str_constants::text::C563853A);
        assert_eq!(error.field().as_ref(), "column_0");
        let update = super::TableExampleUpdate::try_from(super::TableExampleUpdateForm {
            primary_key_column: frontend_contract::FormValue::try_from(
                str_constants::text::VALUE_550E8400_E29B_41D4_A716_446655440000.to_owned(),
            )
            .expect("5b8439c1"),
            column_0: Some(
                frontend_contract::FormValue::try_from(str_constants::text::VALUE_13.to_owned())
                    .expect("4bd3fc27"),
            ),
            column_1: None,
            column_2: None,
            revision: None,
        });
        let _update = update.expect("c5d0bf17");
    }
    #[test]
    fn generated_frontend_client_accepts_transport_adapter() {
        let client = super::TableExampleFrontendApiClient::new(TestTransport);
        let cloned = client.clone();
        assert_eq!(format!("{client:?}"), format!("{cloned:?}"));
    }
}
