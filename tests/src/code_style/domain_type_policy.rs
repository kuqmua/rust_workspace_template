#[test]
fn string_wrappers_do_not_use_from_string() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::E2A6B9C4),
        super::types::SourceTextRef::from(str_constants::STRING_WRAPPERS_MUST_VALIDATE_LENGTH_USE_TRYFROM_STRING_WITH_A_LENGTH_CHECK),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let string_wrapper_names =
                super::string_wrapper_names(super::types::SynFileRef::from(ast));
            let len_checked_function_names =
                super::len_checked_function_names(super::types::SynFileRef::from(ast));
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::StringWrapperFromVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    len_checked_function_names: &len_checked_function_names,
                    string_wrapper_names: &string_wrapper_names,
                    try_from_string_names: super::types::StdSourceTextSet::default(),
                    try_from_string_len_checked_names: super::types::StdSourceTextSet::default(),
                },
            );
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.try_from_string_names.contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` must implement `TryFrom<String>` with a length check",
                                path.display()
                            ))
                        }
                    }));
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.try_from_string_len_checked_names.contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` implements `TryFrom<String>` without a `.len()` check",
                                path.display()
                            ))
                        }
                    }));
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn from_string_impl_visitor_rejects_non_string_wrappers_too() {
    let ast = syn::parse_file(str_constants::NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_IMPL_FROM_STRING_FOR_SOURCETEXT_NEWLINE).expect("f7c0e2a9");
    let string_wrapper_names = super::types::StdSourceTextSet::default();
    let len_checked_function_names =
        super::len_checked_function_names(super::types::SynFileRef::from(&ast));
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::StdSourceTextSet::default(),
            try_from_string_len_checked_names: super::types::StdSourceTextSet::default(),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 1, "a06d3c4f {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains("`SourceText` implements `From<String>`")),
        "b19e40c8 {ers:#?}"
    );
}
#[test]
fn bounded_string_derive_satisfies_string_wrapper_policy() {
    let ast = syn::parse_file(
        str_constants::NEWLINE_CONST_SOURCE_TEXT_MAX_LEN_USIZE_1024_NEWLINE_DERIVE_NEWTYPE_PATH,
    )
    .expect("90df57a8");
    let string_wrapper_names = super::string_wrapper_names(super::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        super::len_checked_function_names(super::types::SynFileRef::from(&ast));
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::StdSourceTextSet::default(),
            try_from_string_len_checked_names: super::types::StdSourceTextSet::default(),
        },
    );
    assert!(
        visitor.try_from_string_names.contains("SourceText"),
        "e4b9120c"
    );
    assert!(
        visitor
            .try_from_string_len_checked_names
            .contains("SourceText"),
        "69f280b3"
    );
    assert!(visitor.ers.is_empty(), "fbd8c479 {:#?}", visitor.ers);
}
#[test]
fn newtype_try_from_validator_satisfies_string_wrapper_policy() {
    let ast: syn::File = syn::parse_quote! {
            #[derive(newtype::TryFrom)]
    #[try_from(
                validator = validate_value
            )]
            struct Value(String);
            fn validate_value(value: &str) -> Result<(), ValueError> {
                if value.len() > 8usize {
                    Err(ValueError)
                } else {
                    Ok(())
                }
            }
        };
    let string_wrapper_names = super::string_wrapper_names(super::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        super::len_checked_function_names(super::types::SynFileRef::from(&ast));
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::StdSourceTextSet::default(),
            try_from_string_len_checked_names: super::types::StdSourceTextSet::default(),
        },
    );
    assert!(visitor.try_from_string_names.contains("Value"), "4d8a4c7e");
    assert!(
        visitor.try_from_string_len_checked_names.contains("Value"),
        "7c2e4b50"
    );
}
#[test]
fn public_tuple_wrappers_do_not_expose_inner_field() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::B7C84E2A),
        super::types::SourceTextRef::from(str_constants::PUBLIC_TUPLE_WRAPPERS_MUST_NOT_EXPOSE_INNER_FIELDS_INITIALIZE_THEM_THROUGH_FROM),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::PublicTupleWrapperFieldVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn domain_boundaries_use_repository_declared_types() {
    let repo_crates = super::workspace_crate_names();
    let repo_types = super::declared_domain_type_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::A7F9C3E1),
        super::types::SourceTextRef::from(str_constants::RAW_EXTERNAL_OR_PRIMITIVE_TYPES_FOUND_IN_DOMAIN_BOUNDARIES_USE_REPOSITORY_DOMAIN),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::DomainTypePolicyVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    closure_body_scan_depth: super::types::AnalyzerCount::default(),
                    generic_scopes: Vec::new(),
                    repo_crates: super::types::StdStdSourceTextSetRef::from(repo_crates.as_ref()),
                    repo_types: super::types::StdStdSourceTextSetRef::from(repo_types.as_ref()),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn domain_type_policy_checks_explicit_closure_parameter_types() {
    let ast = syn::parse_file(
        str_constants::NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_FN_DEMO_NEWLINE_LET_PATH_CB,
    )
    .expect("c81a6f20");
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::from([String::from(str_constants::SOURCETEXT)]);
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::DomainTypePolicyVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            closure_body_scan_depth: super::types::AnalyzerCount::default(),
            generic_scopes: Vec::new(),
            repo_crates: super::types::StdStdSourceTextSetRef::from(&repo_crates),
            repo_types: super::types::StdStdSourceTextSetRef::from(&repo_types),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 2, "0f6d3a91 {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains("closure parameter uses `std::path::PathBuf`")),
        "d4b2f8a0 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains("closure parameter uses `syn::Type`")),
        "60b8ae2d {ers:#?}"
    );
}
#[test]
fn analyzer_state_struct_fields_use_repository_declared_wrappers() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::F2C7A91B),
        super::types::SourceTextRef::from(str_constants::RAW_TEXT_CONTAINERS_FOUND_IN_HELPER_STRUCT_FIELDS_USE_REPOSITORY_WRAPPER_TYPES),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::AnalyzerStateRawContainerFieldVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn analyzer_state_raw_container_field_visitor_reports_helper_fields() {
    let ast = syn::parse_file(str_constants::NEWLINE_STRUCT_HELPERSTATE_NEWLINE_NAMES_VEC_STRING_NEWLINE_SEEN_STD_PATH_COLLECTIONS).expect("9f4d2a7c");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::AnalyzerStateRawContainerFieldVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 3, "2c0b7e91 {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains("field `names` uses `Vec<String>`")),
        "74e18b2d {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains("field `seen` uses `BTreeSet<String>`")),
        "4a0df351 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains("field `refs` uses `HashSet<&str>`")),
        "81c6a2ef {ers:#?}"
    );
}
#[test]
fn helper_return_types_use_repository_declared_text_wrappers() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::VALUE_6D41C8E2),
        super::types::SourceTextRef::from(str_constants::RAW_TEXT_RETURN_TYPES_FOUND_IN_HELPER_FUNCTIONS_USE_REPOSITORY_WRAPPER_TYPES),
        |path, ast, ers| {
            if !super::is_code_style_meta_harness_source_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::HelperRawTextReturnVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn helper_raw_text_return_visitor_reports_free_and_inherent_helpers() {
    let ast = syn::parse_file(
        str_constants::NEWLINE_FN_DIRECT_ARROW_STRING_NEWLINE_STRING_PATH_NEW_NEWLINE_NEWLINE_FN,
    )
    .expect("3a9d7e2c");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::HelperRawTextReturnVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 4, "7b2e41a0 {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains("function `direct` return type uses `String`")),
        "08d4b6ea {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains("function `list` return type uses `Vec<String>`")),
        "ae71c3f4 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains("function `optional` return type uses `Option<&str>`")),
        "59f0bca8 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains("method `nested` return type uses `String`")),
        "c46d8e10 {ers:#?}"
    );
}
#[test]
fn external_leaf_tuple_wrappers_include_crate_name() {
    let repo_crates = super::workspace_crate_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr(str_constants::B93D2A8C),
        super::types::SourceTextRef::from(
            str_constants::TUPLE_WRAPPERS_OVER_EXTERNAL_TYPES_MUST_INCLUDE_THE_EXTERNAL_CRATE_NAME,
        ),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::StdPathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::ExternalLeafWrapperNameVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    repo_crates: super::types::StdStdSourceTextSetRef::from(repo_crates.as_ref()),
                },
            );
            ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
