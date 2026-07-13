#![allow(clippy::needless_for_each)] // utoipa 4 OpenApi derive expands iterator callbacks at crate scope
#![cfg_attr(not(test), allow(unused_crate_dependencies))] // HTTP contract fixtures use server_app_state, tokio, and tower only in this crate's test target
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // utoipa 4 derives component registration with iterator callbacks
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "cm_max_items": 2,
    "create_exclude_fields": ["revision"],
    "idempotent_mutations": true,
    "optimistic_revision_field": "revision",
    "permission_prefix": "tbl_example",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False",
    "um_max_items": 2
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct TblExample {
    #[gen_pg_tbl_pk]
    #[gen_pg_tbl_frontend(label = "Identifier", order = 3, sortable)]
    pub pk_col: pg_types_text_misc::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    #[gen_pg_tbl_frontend(label = "Small number", order = 0, sortable)]
    pub col_0: pg_types_numeric::I16AsNnInt2,
    #[gen_pg_tbl_frontend(filterable, order = 1, placeholder = "Optional number")]
    pub col_1: pg_types_numeric::OptI16AsNlInt2,
    #[gen_pg_tbl_frontend(hidden, order = 2)]
    pub col_2: pg_types_numeric::I32AsNnInt4,
    #[gen_pg_tbl_frontend(hidden, order = 4)]
    pub revision: pg_types_numeric::I64AsNnBigSerialInitByPg,
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
        let router = super::TblExample::routes(app_state);
        let mut contracts = super::TblExampleRouteContract::ALL.into_iter();
        loop {
            let Some(contract) = contracts.next() else {
                break;
            };
            let method = match contract.http_method() {
                super::TblExampleHttpMethod::Delete => http::Method::DELETE,
                super::TblExampleHttpMethod::Patch => http::Method::PATCH,
                super::TblExampleHttpMethod::Post => http::Method::POST,
            };
            let response = tower::ServiceExt::oneshot(
                router.clone(),
                http::Request::builder()
                    .method(method)
                    .uri(contract.path())
                    .header(http::header::CONTENT_TYPE, "text/plain")
                    .body(axum::body::Body::from("{}"))
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
        let cm_payload: super::TblExampleCmPayload = pg_crud_cmn::DfltSomeOneEl::dflt_some_one_el();
        let valid_body = serde_json::to_vec(&cm_payload).expect("29fc2f21");
        let missing_key = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .method(http::Method::POST)
                .uri("/tbl_example/cm")
                .header(http::header::CONTENT_TYPE, "application/json")
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
                .uri("/tbl_example/cm")
                .header(http::header::CONTENT_TYPE, "text/plain")
                .header("idempotency-key", "negative-content-type")
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
                .uri("/tbl_example/cm")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header("idempotency-key", "negative-malformed")
                .body(axum::body::Body::from("{"))
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
                .uri("/tbl_example/cm")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header("idempotency-key", "duplicate-a")
                .header("idempotency-key", "duplicate-b")
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
                .uri("/tbl_example/uo")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header("idempotency-key", "missing-revision")
                .body(axum::body::Body::from("{}"))
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
                .uri("/tbl_example/cm")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header("idempotency-key", "negative-oversized")
                .body(axum::body::Body::from("x".repeat(2_048usize)))
                .expect("aed15d30"),
        )
        .await
        .expect("0d8df630");
        assert_eq!(oversized.status(), http::StatusCode::PAYLOAD_TOO_LARGE);
        let unsupported_method = tower::ServiceExt::oneshot(
            router,
            http::Request::builder()
                .method(http::Method::GET)
                .uri("/tbl_example/cm")
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
                            frontend_contract::TransportEr,
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
                if key == "$ref"
                    && let Some(name) = value
                        .as_str()
                        .and_then(|value| value.strip_prefix("#/components/schemas/"))
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
        let doc = serde_json::to_value(super::TblExampleOpenApi::open_api()).expect("3176b0d5");
        [
            ("cm", "post"),
            ("co", "post"),
            ("rm", "post"),
            ("ro", "post"),
            ("uo", "patch"),
            ("dm", "delete"),
            ("dlo", "delete"),
        ]
        .into_iter()
        .for_each(|(operation, method)| {
            let operation_doc = doc
                .pointer(&format!("/paths/~1tbl_example~1{operation}/{method}"))
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
            let success_status = if operation == "cm" || operation == "co" {
                "201"
            } else {
                "200"
            };
            assert!(operation_doc["responses"].get(success_status).is_some());
            operation_doc["responses"]
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
            if operation == "uo" {
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
        assert!(doc.pointer("/paths/~1tbl_example~1um").is_none());
        let schemas = doc["components"]["schemas"].as_object().expect("95ec6823");
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
            schemas["TblExampleCr"]["properties"]
                .get("pk_col")
                .is_none()
        );
        ["TblExampleRd", "TblExampleUpd"]
            .into_iter()
            .for_each(|schema_name| {
                let nullable_projection_or_patch = &schemas[schema_name]["properties"]["col_1"];
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
                    .is_some_and(|required| required.iter().any(|field| field == "col_1")));
            });
        [
            "pg_crud_cmn.PgType.Rd",
            "pg_crud_cmn.PgType.Sel",
            "wh_flts.PgTypeWhEq",
            "wh_flts.PgTypeWhBtwn",
            "wh_flts.PgTypeWhGreaterThan",
            "wh_flts.PgTypeWhIn",
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
        super::TblExampleRouteContract::ALL
            .into_iter()
            .for_each(|contract| {
                let expected = match contract.operation() {
                    super::TblExampleOperation::Cm | super::TblExampleOperation::Co => {
                        "tbl_example:create"
                    }
                    super::TblExampleOperation::Rm | super::TblExampleOperation::Ro => {
                        "tbl_example:read"
                    }
                    super::TblExampleOperation::Um | super::TblExampleOperation::Uo => {
                        "tbl_example:update"
                    }
                    super::TblExampleOperation::Dlo | super::TblExampleOperation::Dm => {
                        "tbl_example:delete"
                    }
                };
                assert_eq!(
                    contract.authentication(),
                    super::TblExampleAuthenticationRequirement::Permission(expected)
                );
            });
    }
    #[test]
    fn generated_frontend_fields_follow_table_and_crud_contract() {
        let fields = super::TblExample::frontend_fields();
        assert_eq!(fields.as_ref().len(), 5usize);
        let first = fields.as_ref().first().expect("fead1583");
        assert_eq!(first.name().as_ref(), "col_0");
        assert_eq!(first.label().as_ref(), "Small number");
        assert_eq!(
            first.sortable(),
            frontend_contract::FieldCapability::Enabled
        );
        let pk = fields
            .as_ref()
            .iter()
            .find(|field| field.primary_key() == frontend_contract::PrimaryKeyKind::Primary)
            .expect("0a4fe013");
        assert_eq!(pk.name().as_ref(), "pk_col");
        assert_eq!(pk.label().as_ref(), "Identifier");
        assert_eq!(pk.primary_key(), frontend_contract::PrimaryKeyKind::Primary);
        assert_eq!(pk.creatable(), frontend_contract::FieldCapability::Disabled);
        assert_eq!(pk.readable(), frontend_contract::FieldCapability::Enabled);
        let nullable = fields.as_ref().get(1usize).expect("0cb93d7f");
        assert_eq!(nullable.name().as_ref(), "col_1");
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
        let routes = super::TblExampleRouteContract::frontend_contracts();
        assert_eq!(
            routes.as_ref().len(),
            super::TblExampleRouteContract::ALL.len()
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
        let page = super::TblExample::frontend_page();
        assert_eq!(page.path().as_ref(), "/tbl_example");
        assert_eq!(page.title().as_ref(), "Tbl Example");
        assert_eq!(page.fields().as_ref().len(), 5usize);
        assert_eq!(
            page.actions().as_ref().len(),
            super::TblExampleRouteContract::ALL.len()
        );
        assert!(page.actions().as_ref().iter().any(|action| {
            action.operation() == frontend_contract::OperationKind::DeleteOne
                && action.confirmation() == frontend_contract::ConfirmationRequirement::Required
        }));
    }
    #[test]
    fn generated_frontend_forms_parse_typed_payloads_and_report_field() {
        let create = super::TblExampleCr::try_from(super::TblExampleCreateForm {
            col_0: frontend_contract::FormValue::try_from("12".to_owned()).expect("8f6b2f31"),
            col_1: frontend_contract::FormValue::try_from(String::new()).expect("274d2e0c"),
            col_2: frontend_contract::FormValue::try_from("34".to_owned()).expect("98c9cd5e"),
        });
        let _create = create.expect("af5a7ec4");
        let error = super::TblExampleCr::try_from(super::TblExampleCreateForm {
            col_0: frontend_contract::FormValue::try_from("not-a-number".to_owned())
                .expect("a6413c9d"),
            col_1: frontend_contract::FormValue::try_from(String::new()).expect("1970fd5b"),
            col_2: frontend_contract::FormValue::try_from("34".to_owned()).expect("fd5e40c9"),
        })
        .expect_err("c563853a");
        assert_eq!(error.field().as_ref(), "col_0");
        let update = super::TblExampleUpd::try_from(super::TblExampleUpdateForm {
            pk_col: frontend_contract::FormValue::try_from(
                "550e8400-e29b-41d4-a716-446655440000".to_owned(),
            )
            .expect("5b8439c1"),
            col_0: Some(frontend_contract::FormValue::try_from("13".to_owned()).expect("4bd3fc27")),
            col_1: None,
            col_2: None,
            revision: None,
        });
        let _update = update.expect("c5d0bf17");
    }
    #[test]
    fn generated_frontend_client_accepts_transport_adapter() {
        let client = super::TblExampleFrontendApiClient::new(TestTransport);
        let cloned = client.clone();
        assert_eq!(format!("{client:?}"), format!("{cloned:?}"));
    }
}
