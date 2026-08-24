#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the runtime reviewed-duplicate inventory must precede its analyzer types without becoming a forbidden string constant"
)]

#[allow(
    clippy::single_call_fn,
    reason = "the named runtime constructor isolates the exact reviewed inventory while avoiding string constants outside str_constants"
)]
fn reviewed_duplicate_groups() -> Vec<ReviewedDuplicateGroup> {
    vec![
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/domain_analysis.rs::visit_item_impl\n../tests/src/code_style/domain_analysis.rs::visit_item_struct",
            reason: "syn Visit requires separate callbacks for impl and struct items; both delegate to the same visitor state",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/operational_invariants.rs::try_from\n../pg_crud_pg_crud_common/src/sql_identifier.rs::try_from\n../pg_crud_pg_crud_common/src/sql_identifier.rs::try_from",
            reason: "TryFrom implementations are domain boundaries with distinct wrapper and error types",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/mod.rs::item_impl_is_from_string\n../tests/src/code_style/mod.rs::item_impl_is_try_from_string",
            reason: "the predicates inspect distinct conversion traits while deliberately sharing structural matching rules",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/source_analysis.rs::visit_expr_lit\n../tests/src/code_style/source_analysis.rs::visit_expr_lit",
            reason: "independent AST analyses must implement the same syn Visit callback",
        },
        ReviewedDuplicateGroup {
            locations: "../git_info/src/lib.rs::try_from\n../git_info/src/lib.rs::try_from",
            reason: "separate repository-domain wrappers retain distinct validation errors",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/domain_analysis.rs::visit_item\n../tests/src/code_style/runtime_analysis.rs::visit_item\n../tests/src/code_style/runtime_analysis.rs::visit_item\n../tests/src/code_style/runtime_analysis.rs::visit_item\n../tests/src/code_style/runtime_analysis.rs::visit_item\n../tests/src/code_style/source_analysis.rs::visit_item\n../tests/src/code_style/source_analysis.rs::visit_item",
            reason: "independent policy visitors collect different facts through the required syn Visit item callback",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/filter_bind_plan.rs::try_from\n../pg_crud_pg_types_generate_pg_types_src/src/source.rs::try_from\n../pg_crud_pg_types_generate_pg_types_src/src/source.rs::try_from\n../server_runtime_http/src/metrics_layer.rs::try_from",
            reason: "mechanical TryFrom adapters call type-specific invariant constructors and preserve domain-specific errors",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/auth/html.rs::delete_role\n../server_admin/src/auth/html.rs::delete_user",
            reason: "route handlers are separate Axum registration targets and delegate authentication through authenticated_action",
        },
        ReviewedDuplicateGroup {
            locations: "../server_runtime_http/src/batched_cleanup.rs::try_from\n../server_runtime_http/src/limits.rs::try_from",
            reason: "positive-value domain boundaries expose distinct public errors; the shared shape is only trait glue",
        },
        ReviewedDuplicateGroup {
            locations: "../config_lib/src/lib.rs::try_from\n../config_lib/src/lib.rs::try_from\n../pg_crud_pg_table/src/lib.rs::try_from\n../pg_crud_pg_table/src/lib.rs::try_from\n../tests/src/domain_type_policy_fixture.rs::try_from",
            reason: "conversion adapters map external values into unrelated domain wrappers and error contracts",
        },
        ReviewedDuplicateGroup {
            locations: "../server_runtime_http/src/pg_rate_limit.rs::try_from\n../server_runtime_http/src/pg_rate_limit.rs::try_from",
            reason: "rate-limit wrappers have separate domain meanings and validation error variants",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_table/src/lib.rs::try_from\n../pg_crud_pg_table/src/lib.rs::try_from",
            reason: "generated table metadata wrappers require separate TryFrom trait implementations",
        },
        ReviewedDuplicateGroup {
            locations: "../bounded_types/src/btree_map.rs::try_from\n../bounded_types/src/hash_map.rs::try_from",
            reason: "collection-specific trait adapters already reuse validate_len; their concrete map types cannot share an impl",
        },
        ReviewedDuplicateGroup {
            locations: "../server_runtime_core/src/lease_registry.rs::try_from\n../server_runtime_core/src/lease_registry.rs::try_from",
            reason: "lease domain wrappers preserve distinct types and error contracts",
        },
        ReviewedDuplicateGroup {
            locations: "../frontend_contract/src/lib.rs::validate\n../pg_crud_pg_table/src/lib.rs::validate",
            reason: "derive validators live at separate macro expansion boundaries and construct different domain errors",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/auth.rs::from_request_parts\n../server_admin/src/auth.rs::from_request_parts",
            reason: "Axum requires one FromRequestParts implementation per extractor result type",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/mod.rs::attr_has_bounded_string_derive\n../tests/src/code_style/mod.rs::attr_has_newtype_from_option",
            reason: "attribute predicates intentionally inspect different derive paths using the same syntax traversal",
        },
        ReviewedDuplicateGroup {
            locations: "../frontend_contract/src/problem.rs::validate\n../frontend_contract_validation/src/openapi_validation.rs::validate\n../frontend_contract_validation/src/route_contract_validation.rs::validate",
            reason: "validators enforce unrelated contracts and return their own domain-specific errors",
        },
        ReviewedDuplicateGroup {
            locations: "../location_lib/src/location.rs::validate\n../location_lib/src/location.rs::validate\n../macros_helpers/src/generate_field_location_new_token_stream.rs::validate\n../macros_helpers/src/generate_field_location_new_token_stream.rs::validate",
            reason: "location newtypes and generated tokens each require a local validator at their invariant boundary",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/auth/html.rs::user_ban\n../server_admin/src/auth/html.rs::user_password",
            reason: "separate route handlers delegate shared CSRF and result handling through authenticated_action",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/advisory_lock.rs::try_from\n../pg_crud_pg_crud_common/src/operational_invariants.rs::try_from",
            reason: "positive-value conversions define unrelated PostgreSQL domain types and public errors",
        },
        ReviewedDuplicateGroup {
            locations: "../config_lib/src/lib.rs::try_from\n../server_admin_core/src/lib.rs::try_from",
            reason: "configuration and administrator identifiers require separate domain conversion boundaries",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/runtime_analysis.rs::visit_impl_item_fn\n../tests/src/code_style/runtime_analysis.rs::visit_item_fn\n../tests/src/code_style/runtime_analysis.rs::visit_trait_item_fn",
            reason: "syn Visit exposes free, impl, and trait functions through distinct required callbacks",
        },
        ReviewedDuplicateGroup {
            locations: "../git_info/src/lib.rs::validate\n../git_info/src/lib.rs::validate",
            reason: "two git metadata wrappers validate the same character policy but retain separate domain types",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_macros_common/src/lib.rs::generate_impl_pg_crud_common_default_some_one_element_max_page_size_token_stream\n../pg_crud_pg_crud_macros_common/src/lib.rs::generate_impl_pg_crud_common_default_some_one_element_token_stream",
            reason: "macro entry points emit different trait implementations and must remain separately addressable",
        },
        ReviewedDuplicateGroup {
            locations: "../server_runtime_core/src/lease_registry.rs::try_from\n../server_runtime_http/src/http_client.rs::try_from\n../server_runtime_http/src/http_client.rs::try_from\n../server_runtime_http/src/lifecycle.rs::try_from\n../server_runtime_http/src/lifecycle.rs::try_from",
            reason: "duration wrappers own independent invariants and error types at separate crate domain boundaries",
        },
        ReviewedDuplicateGroup {
            locations: "../newtype/src/lib.rs::bounded_string\n../newtype/src/lib.rs::enum_from_str",
            reason: "derive macro parsers consume different attributes but use the same syn error propagation skeleton",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/cursor.rs::try_from\n../pg_crud_pg_crud_common/src/cursor.rs::try_from",
            reason: "cursor wire formats have separate domain wrappers and decoding error variants",
        },
        ReviewedDuplicateGroup {
            locations: "../server_runtime_core/src/secret_text.rs::try_from\n../server_runtime_core/src/secret_text.rs::try_from",
            reason: "secret wrappers enforce different policies while keeping their concrete errors and redaction types",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/mod.rs::item_struct_derives_conversion\n../tests/src/code_style/mod.rs::item_struct_derives_try_from",
            reason: "derive-policy predicates check different conversion capabilities with common syntax matching",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/lib.rs::try_from\n../server_runtime_http/src/pg_rate_limit.rs::try_from",
            reason: "database count conversions target unrelated bounded domain types and errors",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/mod.rs::item_impl_is_from\n../tests/src/code_style/mod.rs::item_impl_is_try_from",
            reason: "trait predicates distinguish From and TryFrom while sharing the same AST shape",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/source_analysis.rs::visit_item_struct\n../tests/src/code_style/source_analysis.rs::visit_item_struct\n../tests/src/code_style/source_analysis.rs::visit_item_struct",
            reason: "independent source policies inspect structs through the required syn Visit callback",
        },
        ReviewedDuplicateGroup {
            locations: "../config_lib/src/http.rs::try_from\n../config_lib/src/pg_pool.rs::try_from",
            reason: "HTTP and PostgreSQL configuration values expose separate parsing errors and wrapper types",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/query_fragment.rs::try_from\n../pg_crud_pg_crud_common/src/query_pagination.rs::try_from\n../pg_crud_pg_crud_common/src/query_pagination.rs::try_from",
            reason: "query fragment and pagination wrappers retain distinct SQL-domain invariants and errors",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/auth.rs::validate\n../server_admin/src/auth.rs::validate",
            reason: "administrator token TTL wrappers require separate derive validators and domain types",
        },
        ReviewedDuplicateGroup {
            locations: "../frontend_contract/src/route.rs::try_from\n../server_runtime_http/src/path_policy.rs::try_from",
            reason: "frontend routes and proxy paths have separate public domain contracts despite similar conversion flow",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/repository.rs::into_parts\n../server_admin/src/repository.rs::into_parts",
            reason: "repository record destructuring preserves two unrelated domain tuple contracts",
        },
        ReviewedDuplicateGroup {
            locations: "../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_const_try_new_for_identifier_token_stream\n../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_pub_const_try_new_for_identifier_token_stream\n../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_pub_try_new_for_identifier_token_stream\n../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_try_new_for_identifier_token_stream",
            reason: "stable public code-generation adapters already delegate implementation wrapping and modified constructors to shared helpers",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin_frontend/src/ssr.rs::try_from\n../server_admin_frontend/src/ssr.rs::try_from",
            reason: "bounded SSR text and HTML wrappers retain distinct public types and conversion errors",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/rbac.rs::as_str\n../server_admin/src/rbac.rs::as_str",
            reason: "separate audit action and resource enums require exhaustive domain-specific wire mappings",
        },
        ReviewedDuplicateGroup {
            locations: "../generate_quotes/src/lib.rs::binary_double_quote_style\n../generate_quotes/src/lib.rs::double_quote_style",
            reason: "quote style declarations already delegate construction and retain distinct prefix and diagnostic metadata",
        },
        ReviewedDuplicateGroup {
            locations: "../server_runtime_http/src/geojson.rs::validate_geo_json\n../server_runtime_http/src/geojson.rs::validate_geo_json\n../server_runtime_http/src/geojson.rs::validate_geo_json",
            reason: "derive validators are required on three distinct GeoJSON domain wrapper boundaries",
        },
        ReviewedDuplicateGroup {
            locations: "../bounded_types/src/string.rs::try_from\n../bounded_types/src/vector.rs::try_from",
            reason: "string and vector conversion adapters expose distinct collection types and errors while reusing bounded validation",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/bounded_btree_map.rs::deserialize\n../pg_crud_pg_crud_common/src/bounded_vec.rs::deserialize\n../pg_crud_where_filters/src/lib.rs::deserialize",
            reason: "serde requires concrete deserializers for distinct bounded domain collections; each delegates validation to its wrapper",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/advanced_policy.rs::visit_expr_loop\n../tests/src/code_style/advanced_policy.rs::visit_expr_while\n../tests/src/code_style/runtime_analysis.rs::visit_expr_async",
            reason: "independent syntax policies implement required syn Visit callbacks for different control-flow constructs",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/advanced_policy.rs::visit_expr_await\n../tests/src/code_style/advanced_policy.rs::visit_macro",
            reason: "one policy visitor records separate await and macro syntax through required syn callbacks",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/mod.rs::item_impl_contains_len_call\n../tests/src/code_style/mod.rs::len_checked_function_names",
            reason: "policy predicates inspect different syntax owners and only share the required AST traversal shape",
        },
        ReviewedDuplicateGroup {
            locations: "../generate_quotes/src/lib.rs::binary_single_quote_style\n../generate_quotes/src/lib.rs::single_quote_style",
            reason: "quote style declarations already delegate construction and retain distinct prefix and diagnostic metadata",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/db_schema_conformance.rs::schema_text\n../pg_crud_pg_crud_common/src/db_schema_conformance.rs::schema_text",
            reason: "schema name and type wrappers preserve distinct domain boundaries and typed validation errors",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/src/code_style/domain_analysis.rs::visit_item_enum\n../tests/src/code_style/domain_analysis.rs::visit_item_struct\n../tests/src/code_style/domain_analysis.rs::visit_item_trait\n../tests/src/code_style/domain_analysis.rs::visit_item_union",
            reason: "syn exposes each domain declaration kind through a distinct required callback that delegates to shared field analysis",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_table_generate_pg_table_src/src/source.rs::generate_pg_table_attr_error_variants\n../pg_crud_pg_table_generate_pg_table_src/src/source.rs::generate_pg_table_attr_logic\n../server_admin/src/auth.rs::get",
            reason: "identifier-normalized token emitters and an unrelated typed accessor coincide structurally but have different behavior and owners",
        },
        ReviewedDuplicateGroup {
            locations: "../newtype/src/lib.rs::to_err_string\n../newtype/src/lib.rs::to_err_string_as_ref_str\n../newtype/src/lib.rs::to_err_string_debug",
            reason: "distinct derive entry points emit different conversion expressions through the same proc-macro parsing contract",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_macros_common/src/lib.rs::generate_impl_pg_crud_default_some_one_element_max_page_size_token_stream\n../pg_crud_pg_crud_macros_common/src/lib.rs::generate_impl_pg_crud_default_some_one_element_token_stream",
            reason: "separate stable macro entry points emit distinct default traits while sharing the surrounding token construction shape",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/lib.rs::visit_str\n../pg_crud_where_filters/src/lib.rs::visit_str",
            reason: "independent serde visitors must implement the same required string callback for unrelated wire types",
        },
        ReviewedDuplicateGroup {
            locations: "../tests/trybuild/route_contract_wrong_request.rs::metadata\n../tests/trybuild/route_contract_wrong_response.rs::metadata\n../tests/trybuild/route_contract_wrong_route.rs::metadata\n../tests/trybuild/route_contract_wrong_route.rs::metadata\n../tests/trybuild/route_contract_wrong_transport.rs::metadata",
            reason: "compile-fail fixtures deliberately reproduce invalid trait metadata implementations for distinct diagnostics",
        },
        ReviewedDuplicateGroup {
            locations: "../pg_crud_pg_crud_common/src/pg_values.rs::to_query_str\n../pg_crud_pg_crud_macros_common/src/lib.rs::non_null_or_nullable_str\n../pg_crud_pg_crud_macros_common/src/lib.rs::sc_str\n../pg_crud_pg_crud_macros_common/src/lib.rs::to_path\n../pg_crud_where_filters/src/lib.rs::postgreql_syntax",
            reason: "identifier normalization makes unrelated small enum-to-domain-value mappings structurally equal despite distinct return types and semantics",
        },
        ReviewedDuplicateGroup {
            locations: "../frontend_contract/src/route.rs::metadata\n../tests/trybuild/route_contract_wrong_path_parameter.rs::metadata",
            reason: "the compile-fail fixture intentionally mirrors route metadata before introducing its invalid path parameter contract",
        },
        ReviewedDuplicateGroup {
            locations: "../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_const_new_for_identifier_token_stream\n../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_new_for_identifier_token_stream\n../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_pub_const_new_for_identifier_token_stream\n../macros_helpers/src/generate_new_or_try_new.rs::generate_impl_pub_new_for_identifier_token_stream",
            reason: "stable public new-constructor adapters already delegate body generation and impl wrapping to shared helpers",
        },
        ReviewedDuplicateGroup {
            locations: "../server_admin/src/auth.rs::from_request\n../server_admin/src/auth.rs::from_request",
            reason: "Axum requires distinct FromRequest implementations for separate authenticated body extractors",
        },
    ]
}

#[derive(optml::Optml, Debug, Default)]
struct FunctionBodyComplexity {
    expression_count: usize,
}

#[derive(optml::Optml)]
struct FunctionBodyVisitor<'visitor_lt> {
    bodies: super::types::StdFunctionBodyLocationsMapMutRef<'visitor_lt>,
    identifier_pattern: super::types::RegexRegexRef<'visitor_lt>,
    path: super::types::StdPathRef<'visitor_lt>,
}

#[derive(optml::Optml)]
struct ReviewedDuplicateGroup {
    locations: &'static str,
    reason: &'static str,
}

impl<'ast> syn::visit::Visit<'ast> for FunctionBodyComplexity {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        self.expression_count = self.expression_count.saturating_add(1usize);
        syn::visit::visit_expr(self, i);
    }
}

impl FunctionBodyVisitor<'_> {
    fn record(&mut self, name: &syn::Ident, block: &syn::Block) {
        if function_body_is_substantial(block) {
            self.bodies
                .entry(function_body_hash(block, self.identifier_pattern))
                .or_default()
                .push(format!("{}::{name}", self.path.as_ref().display()));
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for FunctionBodyVisitor<'_> {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        self.record(&i.sig.ident, &i.block);
        syn::visit::visit_impl_item_fn(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if !super::item_fn_is_unit_test(super::types::SynItemFnRef::from(i)).get() {
            self.record(&i.sig.ident, &i.block);
        }
        syn::visit::visit_item_fn(self, i);
    }
}

fn function_body_is_substantial(block: &syn::Block) -> bool {
    let mut complexity = FunctionBodyComplexity::default();
    syn::visit::Visit::visit_block(&mut complexity, block);
    complexity.expression_count >= 9usize
}

fn function_body_hash(
    block: &syn::Block,
    identifier_pattern: super::types::RegexRegexRef<'_>,
) -> super::types::FunctionBodyHash {
    let body = format!("{block:?}");
    let normalized_body =
        identifier_pattern.replace_all(&body, str_constants::NORMALIZED_IDENTIFIER);
    let mut hasher = std::hash::DefaultHasher::new();
    std::hash::Hash::hash(&normalized_body, &mut hasher);
    super::types::FunctionBodyHash::from(std::hash::Hasher::finish(&hasher))
}

#[test]
fn substantial_function_bodies_have_one_source_of_truth() {
    let mut bodies = super::types::StdFunctionBodyLocationsMap::default();
    let identifier_pattern =
        regex::Regex::new(r"Ident \{ sym: [^,]+, span: [^}]+ \}").expect("d4a8c2f1");
    super::snapshot::with_codebase_snapshot(|snapshot| {
        snapshot.rs_files().iter().for_each(|file| {
            let mut visitor = FunctionBodyVisitor {
                bodies: super::types::StdFunctionBodyLocationsMapMutRef::from(&mut bodies),
                identifier_pattern: super::types::RegexRegexRef::from(&identifier_pattern),
                path: super::types::StdPathRef::from(file.path().as_ref()),
            };
            syn::visit::Visit::visit_file(&mut visitor, file.ast().as_ref());
        });
    });
    let mut reviewed = reviewed_duplicate_groups().into_iter().fold(
        std::collections::BTreeMap::<&str, &str>::new(),
        |mut reviewed_groups, group| {
            assert!(
                !group.reason.trim().is_empty(),
                "reviewed duplicate group must explain why extraction is inappropriate: {}",
                group.locations
            );
            assert!(
                reviewed_groups
                    .insert(group.locations, group.reason)
                    .is_none(),
                "reviewed duplicate group is declared more than once: {}",
                group.locations
            );
            reviewed_groups
        },
    );
    let duplicates = std::collections::BTreeMap::<
        super::types::FunctionBodyHash,
        super::types::SourceTextList,
    >::from(bodies)
    .into_values()
    .filter(|locations| locations.len() > 1usize)
    .filter_map(|mut locations| {
        locations.sort_unstable();
        let location_signature = locations.join("\n");
        reviewed
            .remove(location_signature.as_str())
            .is_none()
            .then_some(location_signature)
    })
    .collect::<Vec<String>>();
    assert!(
        duplicates.is_empty(),
        "substantial duplicate function bodies found; extract one source of truth:\n{}",
        duplicates.join("\n\n")
    );
    assert!(
        reviewed.is_empty(),
        "reviewed duplicate groups no longer match the source; remove or update them:\n{}",
        reviewed.keys().copied().collect::<Vec<_>>().join("\n\n")
    );
}

#[test]
fn function_body_similarity_ignores_identifier_names() {
    let first = syn::parse_str::<syn::ItemFn>("fn first(input: u32) { let value = input + 1; }")
        .expect("ca632fad");
    let second =
        syn::parse_str::<syn::ItemFn>("fn second(source: u32) { let result = source + 1; }")
            .expect("b608f7e1");
    let identifier_pattern =
        regex::Regex::new(r"Ident \{ sym: [^,]+, span: [^}]+ \}").expect("9658f225");
    let identifier_pattern_ref = super::types::RegexRegexRef::from(&identifier_pattern);
    assert_eq!(
        function_body_hash(&first.block, identifier_pattern_ref),
        function_body_hash(&second.block, identifier_pattern_ref)
    );
}

#[test]
fn function_body_similarity_preserves_behavioral_structure() {
    let addition = syn::parse_str::<syn::ItemFn>("fn value(input: u32) { let value = input + 1; }")
        .expect("cb1d077f");
    let subtraction =
        syn::parse_str::<syn::ItemFn>("fn value(input: u32) { let value = input - 1; }")
            .expect("ae9313cb");
    let identifier_pattern =
        regex::Regex::new(r"Ident \{ sym: [^,]+, span: [^}]+ \}").expect("fdf7075b");
    let identifier_pattern_ref = super::types::RegexRegexRef::from(&identifier_pattern);
    assert_ne!(
        function_body_hash(&addition.block, identifier_pattern_ref),
        function_body_hash(&subtraction.block, identifier_pattern_ref)
    );
}

#[test]
fn short_mechanical_adapters_are_not_substantial() {
    let adapter = syn::parse_str::<syn::ItemFn>(
        "fn value(input: Option<u32>) -> u32 { input.map(|value| value + 1).unwrap_or_default() }",
    )
    .expect("9dc062d1");
    assert!(!function_body_is_substantial(&adapter.block));
}
