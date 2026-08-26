#[test]
fn string_wrappers_do_not_use_from_string() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::E2A6B9C4),
        super::types::SourceTextRef::from(constants_str::STRING_WRAPPERS_MUST_VALIDATE_LENGTH_USE_TRYFROM_STRING_WITH_A_LENGTH_CHECK),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::PathRef::from(path))
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
                super::domain_analysis::StringWrapperFromVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    len_checked_function_names: &len_checked_function_names,
                    string_wrapper_names: &string_wrapper_names,
                    try_from_string_names: super::types::SourceTextBTreeSet::default(),
                    try_from_string_len_checked_names: super::types::SourceTextBTreeSet::default(),
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
    let ast = syn::parse_file(constants_str::NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_IMPL_FROM_STRING_FOR_SOURCETEXT_NEWLINE).expect("f7c0e2a9 from_string_impl_visitor_rejects_non_string_wrappers_too invariant must hold");
    let string_wrapper_names = super::types::SourceTextBTreeSet::default();
    let len_checked_function_names =
        super::len_checked_function_names(super::types::SynFileRef::from(&ast));
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::SourceTextBTreeSet::default(),
            try_from_string_len_checked_names: super::types::SourceTextBTreeSet::default(),
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
        constants_str::NEWLINE_CONST_SOURCE_TEXT_MAX_LEN_USIZE_1024_NEWLINE_DERIVE_NEWTYPE_PATH,
    )
    .expect("90df57a8 bounded_string_derive_satisfies_string_wrapper_policy invariant must hold");
    let string_wrapper_names = super::string_wrapper_names(super::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        super::len_checked_function_names(super::types::SynFileRef::from(&ast));
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::SourceTextBTreeSet::default(),
            try_from_string_len_checked_names: super::types::SourceTextBTreeSet::default(),
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
        super::domain_analysis::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::SourceTextBTreeSet::default(),
            try_from_string_len_checked_names: super::types::SourceTextBTreeSet::default(),
        },
    );
    assert!(visitor.try_from_string_names.contains("Value"), "4d8a4c7e");
    assert!(
        visitor.try_from_string_len_checked_names.contains("Value"),
        "7c2e4b50"
    );
}
#[test]
fn newtype_try_from_explicit_error_satisfies_string_wrapper_policy() {
    let ast: syn::File = syn::parse_quote! {
        #[derive(newtype::TryFrom)]
        #[try_from(
            error = SharedValueError,
            validator = Value::validate
        )]
        struct Value(String);
        impl Value {
            fn validate(value: &str) -> Result<(), SharedValueError> {
                if value.len() > 8usize {
                    Err(SharedValueError)
                } else {
                    Ok(())
                }
            }
        }
    };
    let string_wrapper_names = super::string_wrapper_names(super::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        super::len_checked_function_names(super::types::SynFileRef::from(&ast));
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            len_checked_function_names: &len_checked_function_names,
            string_wrapper_names: &string_wrapper_names,
            try_from_string_names: super::types::SourceTextBTreeSet::default(),
            try_from_string_len_checked_names: super::types::SourceTextBTreeSet::default(),
        },
    );
    assert!(visitor.try_from_string_names.contains("Value"), "89c632cd");
    assert!(
        visitor.try_from_string_len_checked_names.contains("Value"),
        "b49bc6d0"
    );
}
#[test]
fn tuple_wrappers_do_not_expose_inner_field() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::B7C84E2A),
        super::types::SourceTextRef::from(constants_str::PUBLIC_TUPLE_WRAPPERS_MUST_NOT_EXPOSE_INNER_FIELDS_INITIALIZE_THEM_THROUGH_FROM),
        |path, ast, ers| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::PublicTupleWrapperFieldVisitor {
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
fn tuple_wrapper_deserialization_uses_from_or_try_from() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::B7C84E2A),
        super::types::SourceTextRef::from(constants_str::VALUE_532F14A8),
        |path, ast, ers| {
            let collector = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::TupleWrapperConversionCollector {
                    converted_names: super::types::SourceTextBTreeSet::default(),
                    inner_types: std::collections::BTreeMap::default(),
                    names: super::types::SourceTextBTreeSet::default(),
                    from_names: super::types::SourceTextBTreeSet::default(),
                    from_inner_names: super::types::SourceTextBTreeSet::default(),
                    try_from_names: super::types::SourceTextBTreeSet::default(),
                    try_from_inner_names: super::types::SourceTextBTreeSet::default(),
                },
            );
            let derive_visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::DirectDeserializeTupleWrapperVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                },
            );
            ers.extend(
                derive_visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
            let manual_visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::ManualDeserializeTupleWrapperVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    names: &collector.names,
                },
            );
            ers.extend(
                manual_visitor
                    .ers
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn tuple_wrapper_deserialization_policy_rejects_direct_derive() {
    let ast: syn::File = syn::parse_quote! {
        #[derive(Deserialize)]
        struct Rejected(u64);

        #[derive(Deserialize)]
        #[serde(from = "u64")]
        struct AllowedFrom(u64);

        #[derive(Deserialize)]
        #[serde(try_from = "String")]
        struct AllowedTryFrom(String);

        #[derive(Deserialize)]
        struct Named { value: u64 }

        struct ManualRejected(u64);

        impl<'de> Deserialize<'de> for ManualRejected {
            fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> {
                loop {}
            }
        }

        struct ManualAllowed(u64);

        impl<'de> Deserialize<'de> for ManualAllowed {
            fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> {
                Ok(Self::from(1))
            }
        }
    };
    let collector = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::TupleWrapperConversionCollector {
            converted_names: super::types::SourceTextBTreeSet::default(),
            inner_types: std::collections::BTreeMap::default(),
            names: super::types::SourceTextBTreeSet::default(),
            from_names: super::types::SourceTextBTreeSet::default(),
            from_inner_names: super::types::SourceTextBTreeSet::default(),
            try_from_names: super::types::SourceTextBTreeSet::default(),
            try_from_inner_names: super::types::SourceTextBTreeSet::default(),
        },
    );
    let derive_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::DirectDeserializeTupleWrapperVisitor {
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(derive_visitor.ers.len(), 1, "b0406560");
    assert!(
        derive_visitor.ers.first().is_some_and(|error| {
            error.contains("can bypass validation or other construction invariants")
                && error.contains("#[serde(try_from = \"RawType\")]")
        }),
        "f103b2f0"
    );
    let manual_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::ManualDeserializeTupleWrapperVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            names: &collector.names,
        },
    );
    assert_eq!(manual_visitor.ers.len(), 1, "fbe61e18");
    assert!(
        manual_visitor.ers.first().is_some_and(|error| {
            error.contains("without an explicit From/TryFrom call")
                && error.contains("Self::try_from(raw)")
        }),
        "10fa222c"
    );
}
#[test]
fn tuple_wrappers_initialize_only_through_from_or_try_from() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::B7C84E2A),
        super::types::SourceTextRef::from(constants_str::VALUE_15F71E67),
        |path, ast, ers| {
            let collector = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::TupleWrapperConversionCollector {
                    converted_names: super::types::SourceTextBTreeSet::default(),
                    inner_types: std::collections::BTreeMap::default(),
                    names: super::types::SourceTextBTreeSet::default(),
                    from_names: super::types::SourceTextBTreeSet::default(),
                    from_inner_names: super::types::SourceTextBTreeSet::default(),
                    try_from_names: super::types::SourceTextBTreeSet::default(),
                    try_from_inner_names: super::types::SourceTextBTreeSet::default(),
                },
            );
            ers.extend(
                collector
                    .names
                    .difference(&collector.converted_names)
                    .map(|name| {
                        format!(
                            "{}: tuple wrapper `{name}` has no From/TryFrom implementation",
                            path.display()
                        )
                    }),
            );
            ers.extend(
                collector
                    .from_inner_names
                    .intersection(&collector.try_from_inner_names)
                    .map(|name| {
                        format!(
                            "{}: tuple wrapper `{name}` implements both From and TryFrom for its inner type",
                            path.display()
                        )
                    }),
            );
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::DirectTupleWrapperConstructorVisitor {
                    names: &collector.names,
                    inside_conversion_impl: super::types::AnalyzerBool::default(),
                    current_wrapper_name: None,
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
fn tuple_wrapper_rejects_from_and_try_from_for_same_inner_type() {
    let ast: syn::File = syn::parse_quote! {
        struct Conflict(u64);

        impl From<u64> for Conflict {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl TryFrom<u64> for Conflict {
            type Error = ();

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Ok(Self(value))
            }
        }

        struct DifferentFrom(u64);

        impl From<isize> for DifferentFrom {
            fn from(value: isize) -> Self {
                Self(value as u64)
            }
        }

        impl TryFrom<u64> for DifferentFrom {
            type Error = ();

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Ok(Self(value))
            }
        }
    };

    let collector = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::TupleWrapperConversionCollector {
            converted_names: super::types::SourceTextBTreeSet::default(),
            inner_types: std::collections::BTreeMap::default(),
            names: super::types::SourceTextBTreeSet::default(),
            from_names: super::types::SourceTextBTreeSet::default(),
            from_inner_names: super::types::SourceTextBTreeSet::default(),
            try_from_names: super::types::SourceTextBTreeSet::default(),
            try_from_inner_names: super::types::SourceTextBTreeSet::default(),
        },
    );
    let conflicts: std::collections::BTreeSet<String> = collector
        .from_inner_names
        .intersection(&collector.try_from_inner_names)
        .cloned()
        .collect();
    assert_eq!(
        conflicts,
        std::collections::BTreeSet::from([String::from("Conflict")]),
        "4c8f5a3d {conflicts:#?}"
    );
}

#[test]
fn tuple_wrapper_initialization_policy_rejects_direct_constructors() {
    let ast: syn::File = syn::parse_quote! {
        struct Allowed(u64);

        impl From<u64> for Allowed {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl Allowed {
            fn direct_self() -> Self {
                Self(1)
            }
        }

        fn direct() -> Allowed {
            Allowed(1)
        }

        struct Missing(u64);
        struct Named { value: u64 }
        struct Pair(u64, u64);
    };
    let collector = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::TupleWrapperConversionCollector {
            converted_names: super::types::SourceTextBTreeSet::default(),
            inner_types: std::collections::BTreeMap::default(),
            names: super::types::SourceTextBTreeSet::default(),
            from_names: super::types::SourceTextBTreeSet::default(),
            from_inner_names: super::types::SourceTextBTreeSet::default(),
            try_from_names: super::types::SourceTextBTreeSet::default(),
            try_from_inner_names: super::types::SourceTextBTreeSet::default(),
        },
    );
    assert_eq!(collector.names.len(), 2, "b058f76c");
    assert_eq!(collector.converted_names.len(), 1, "70552188");
    assert_eq!(
        collector
            .names
            .difference(&collector.converted_names)
            .count(),
        1,
        "5cbdfec2"
    );

    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::DirectTupleWrapperConstructorVisitor {
            names: &collector.names,
            inside_conversion_impl: super::types::AnalyzerBool::default(),
            current_wrapper_name: None,
            ers: super::types::DiagnosticMsgs::default(),
        },
    );
    assert_eq!(visitor.ers.len(), 2, "dd79f331");
}
#[test]
fn domain_boundaries_use_repository_declared_types() {
    let repo_crates = super::workspace_crate_names();
    let mut names = std::collections::BTreeSet::new();
    super::for_each_rs_file(|file| {
        let visitor = super::visit_syn_file(
            super::types::SynFileRef::from(file.ast().as_ref()),
            super::domain_analysis::DeclaredDomainTypeVisitor {
                names: super::types::SourceTextBTreeSet::default(),
            },
        );
        names.extend(visitor.names);
    });
    let repo_types = super::types::SourceTextBTreeSet::from(names);
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::A7F9C3E1),
        super::types::SourceTextRef::from(constants_str::RAW_EXTERNAL_OR_PRIMITIVE_TYPES_FOUND_IN_DOMAIN_BOUNDARIES_USE_REPOSITORY_DOMAIN),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::PathRef::from(path))
                .get()
                || super::is_test_crate_source_path(super::types::PathRef::from(path)).get()
                || super::is_code_style_meta_harness_source_path(super::types::PathRef::from(
                    path,
                ))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::DomainTypePolicyVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    closure_body_scan_depth: super::types::AnalyzerCount::default(),
                    generic_scopes: Vec::new(),
                    repo_crates: super::types::SourceTextBTreeSetRef::from(repo_crates.as_ref()),
                    repo_types: super::types::SourceTextBTreeSetRef::from(repo_types.as_ref()),
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
fn environment_initializer_is_in_domain_boundary_policy_scope() {
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("init_env_files/src/domain_types.rs")
        ))
        .get(),
        "c4a791e2"
    );
}
#[test]
fn workspace_test_runner_is_in_domain_boundary_policy_scope() {
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_SRC)
        ))
        .get(),
        "446a3bb7"
    );
}
#[test]
fn workspace_scaffold_is_in_domain_boundary_policy_scope() {
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new(constants_str::WORKSPACE_SCAFFOLD_SRC)
        ))
        .get(),
        "c1a7e4d9"
    );
}
#[test]
fn server_admin_frontend_is_in_domain_boundary_policy_scope() {
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new(constants_str::SERVER_ADMIN_FRONTEND_SRC_APP_RS)
        ))
        .get(),
        "73e9c20f"
    );
}

#[test]
fn server_admin_frontend_ui_is_an_explicit_framework_adapter_boundary() {
    assert!(
        !super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("../server_admin_frontend/src/domain_types_with_owner_button.rs")
        ))
        .get(),
        "e33b8472"
    );
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("server_admin_frontend/src/admin_settings_view.rs")
        ))
        .get(),
        "29bc703d"
    );
}
#[test]
fn domain_fixture_directory_exclusions_are_owner_exact() {
    assert!(
        !super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("../location_lib_location_test/src/lib.rs")
        ))
        .get(),
        "4ab6e2d1"
    );
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("../location_lib_location/src/location_test.rs")
        ))
        .get(),
        "d8c3175f"
    );
    assert!(
        !super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("../pg_crud_common/benches/query.rs")
        ))
        .get(),
        "09e5a6bc"
    );
    assert!(
        super::domain_type_policy_should_check_path(super::types::PathRef::from(
            std::path::Path::new("../server/benches/query.rs")
        ))
        .get(),
        "e5f21c07"
    );
}
#[test]
fn domain_type_policy_reports_raw_browser_external_types_natively() {
    let ast = syn::parse_file(constants_str::VALUE_B7CF0D16)
        .expect("d031ea92 browser invariant must hold");
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::new();
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            closure_body_scan_depth: super::types::AnalyzerCount::default(),
            generic_scopes: Vec::new(),
            repo_crates: super::types::SourceTextBTreeSetRef::from(&repo_crates),
            repo_types: super::types::SourceTextBTreeSetRef::from(&repo_types),
        },
    );
    assert_eq!(
        visitor.ers.len(),
        constants_usize::ONE,
        "79ce162a {:#?}",
        visitor.ers
    );
    assert!(
        visitor
            .ers
            .first()
            .is_some_and(|error| error.contains("web_sys")),
        "bd624f03"
    );
}
#[test]
fn proc_macro_helpers_are_checked_while_compiler_entrypoints_are_exempt() {
    let ast =
        syn::parse_file(constants_str::VALUE_1FB67C5A).expect("5a1d8c34 entry invariant must hold");
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::new();
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            closure_body_scan_depth: super::types::AnalyzerCount::default(),
            generic_scopes: Vec::new(),
            repo_crates: super::types::SourceTextBTreeSetRef::from(&repo_crates),
            repo_types: super::types::SourceTextBTreeSetRef::from(&repo_types),
        },
    );
    assert_eq!(visitor.ers.len(), 2usize, "c82fb6d1 {:#?}", visitor.ers);
    assert!(
        visitor.ers.iter().all(|error| error.contains("helper")),
        "109eb4a7"
    );
}
#[test]
fn domain_type_policy_checks_explicit_closure_parameter_types() {
    let ast = syn::parse_file(
        constants_str::NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_FN_DEMO_NEWLINE_LET_PATH_CB,
    )
    .expect(
        "c81a6f20 domain_type_policy_checks_explicit_closure_parameter_types invariant must hold",
    );
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::from([String::from(constants_str::SOURCETEXT)]);
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            closure_body_scan_depth: super::types::AnalyzerCount::default(),
            generic_scopes: Vec::new(),
            repo_crates: super::types::SourceTextBTreeSetRef::from(&repo_crates),
            repo_types: super::types::SourceTextBTreeSetRef::from(&repo_types),
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
fn domain_type_policy_allows_only_option_and_result_containers() {
    let ast: syn::File = syn::parse_quote! {
        struct AdminRole;
        struct AdminError;
        struct Allowed {
            optional: Option<AdminRole>,
            fallible: Result<AdminRole, AdminError>,
        }
        struct Rejected {
            roles: Vec<AdminRole>,
        }
    };
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::from([
        String::from(constants_str::VALUE_ACE3D828),
        String::from(constants_str::VALUE_81BBD51A),
        String::from(constants_str::VALUE_1BB201D1),
        String::from(constants_str::VALUE_AEA4A04A),
    ]);
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            closure_body_scan_depth: super::types::AnalyzerCount::default(),
            generic_scopes: Vec::new(),
            repo_crates: super::types::SourceTextBTreeSetRef::from(&repo_crates),
            repo_types: super::types::SourceTextBTreeSetRef::from(&repo_types),
        },
    );
    let ers = visitor.ers.into_iter().collect::<Vec<String>>();
    assert_eq!(ers.len(), 1, "c47a91e2 {ers:#?}");
    assert!(
        ers.first()
            .is_some_and(|error| error.contains("uses `Vec`")),
        "d8b305f6 {ers:#?}"
    );
}
#[test]
fn analyzer_state_struct_fields_use_repository_declared_wrappers() {
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::F2C7A91B),
        super::types::SourceTextRef::from(constants_str::RAW_TEXT_CONTAINERS_FOUND_IN_HELPER_STRUCT_FIELDS_USE_REPOSITORY_WRAPPER_TYPES),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::AnalyzerStateRawContainerFieldVisitor {
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
    let ast = syn::parse_file(constants_str::NEWLINE_STRUCT_HELPERSTATE_NEWLINE_NAMES_VEC_STRING_NEWLINE_SEEN_STD_PATH_COLLECTIONS).expect("9f4d2a7c analyzer_state_raw_container_field_visitor_reports_helper_fields invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::AnalyzerStateRawContainerFieldVisitor {
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
        super::types::StaticStr::from(constants_str::VALUE_6D41C8E2),
        super::types::SourceTextRef::from(constants_str::RAW_TEXT_RETURN_TYPES_FOUND_IN_HELPER_FUNCTIONS_USE_REPOSITORY_WRAPPER_TYPES),
        |path, ast, ers| {
            if !super::is_code_style_meta_harness_source_path(super::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::HelperRawTextReturnVisitor {
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
        constants_str::NEWLINE_FN_DIRECT_ARROW_STRING_NEWLINE_STRING_PATH_NEW_NEWLINE_NEWLINE_FN,
    )
    .expect("3a9d7e2c helper_raw_text_return_visitor_reports_free_and_inherent_helpers invariant must hold");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::HelperRawTextReturnVisitor {
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
fn external_leaf_tuple_wrappers_include_source_name() {
    let repo_crates = super::workspace_crate_names();
    super::assert_rs_ast_ers_empty_with_ctx(
        super::types::StaticStr::from(constants_str::B93D2A8C),
        super::types::SourceTextRef::from(
            constants_str::TUPLE_WRAPPERS_OVER_EXTERNAL_TYPES_MUST_INCLUDE_THE_SOURCE_NAME,
        ),
        |path, ast, ers| {
            if !super::domain_type_policy_should_check_path(super::types::PathRef::from(path)).get()
            {
                return;
            }
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(ast),
                super::domain_analysis::ExternalLeafWrapperNameVisitor {
                    ers: super::types::DiagnosticMsgs::default(),
                    repo_crates: super::types::SourceTextBTreeSetRef::from(repo_crates.as_ref()),
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
fn external_leaf_wrapper_type_rule_has_no_name_exceptions() {
    let ast: syn::File = syn::parse_quote! {
        struct GeneratedTokens(proc_macro2::TokenStream);
        struct ProcMacro2GeneratedTokens(proc_macro2::TokenStream);
    };
    let repo_crates = std::collections::BTreeSet::new();
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        super::domain_analysis::ExternalLeafWrapperNameVisitor {
            ers: super::types::DiagnosticMsgs::default(),
            repo_crates: super::types::SourceTextBTreeSetRef::from(&repo_crates),
        },
    );
    assert_eq!(
        visitor.ers.len(),
        constants_usize::ONE,
        "9db6310a {:#?}",
        visitor.ers
    );
    assert!(
        visitor
            .ers
            .first()
            .is_some_and(|error| error.contains("GeneratedTokens")),
        "e7340ba2"
    );
}
