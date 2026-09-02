#[test]
fn test_string_wrappers_do_not_use_from_string() {
    let len_checked_function_names =
        super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
            crate::types::SourceTextBTreeSet::from(
                snapshot
                    .rs_files()
                    .iter()
                    .flat_map(|source_file| {
                        crate::code_style::len_checked_function_names(
                            crate::types::SynFileRef::from(source_file.ast().as_ref()),
                        )
                    })
                    .collect::<std::collections::BTreeSet<String>>(),
            )
        });
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::E2A6B9C4),
        crate::types::SourceTextRef::from(constants_str::STRING_WRAPPERS_MUST_VALIDATE_LENGTH_USE_TRYFROM_STRING_WITH_A_LENGTH_CHECK),
        |path, ast, ers| {
            if !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let string_wrapper_names =
                crate::code_style::string_wrapper_names(crate::types::SynFileRef::from(ast));
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::StringWrapperFromVisitor::new(crate::types::DiagnosticMsgs::default(), &len_checked_function_names, &string_wrapper_names, crate::types::SourceTextBTreeSet::default(), crate::types::SourceTextBTreeSet::default()),
            );
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.get_try_from_string_names().contains(name) {
                            None
                        } else {
                            Some(format!(
                                "{}: string wrapper `{name}` must implement `TryFrom<String>` with a length check",
                                path.display()
                            ))
                        }
                    }));
            ers.extend(string_wrapper_names.iter().filter_map(|name| {
                        if visitor.get_try_from_string_len_checked_names().contains(name) {
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
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_from_string_impl_visitor_rejects_non_string_wrappers_too() {
    let ast = syn::parse_file(constants_str::NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_IMPL_FROM_STRING_FOR_SOURCETEXT_NEWLINE).expect(constants_str::DIAGNOSTIC_F7C0E2A9);
    let string_wrapper_names = crate::types::SourceTextBTreeSet::default();
    let len_checked_function_names =
        crate::code_style::len_checked_function_names(crate::types::SynFileRef::from(&ast));
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            &len_checked_function_names,
            &string_wrapper_names,
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    let ers = visitor
        .get_ers()
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    assert_eq!(ers.len(), 1, "a06d3c4f {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_1CCF1FF0)),
        "b19e40c8 {ers:#?}"
    );
}
#[test]
fn test_bounded_string_derive_satisfies_string_wrapper_policy() {
    let ast = syn::parse_file(
        constants_str::NEWLINE_CONST_SOURCE_TEXT_MAX_LEN_USIZE_1024_NEWLINE_DERIVE_NEWTYPE_PATH,
    )
    .expect(constants_str::DIAGNOSTIC_90DF57A8);
    let string_wrapper_names =
        crate::code_style::string_wrapper_names(crate::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        crate::code_style::len_checked_function_names(crate::types::SynFileRef::from(&ast));
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            &len_checked_function_names,
            &string_wrapper_names,
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    assert!(
        visitor
            .get_try_from_string_names()
            .contains(constants_str::SOURCETEXT),
        "e4b9120c"
    );
    assert!(
        visitor
            .get_try_from_string_len_checked_names()
            .contains(constants_str::SOURCETEXT),
        "69f280b3"
    );
    assert!(
        visitor.get_ers().is_empty(),
        "fbd8c479 {:#?}",
        visitor.get_ers()
    );
}
#[test]
fn test_bounded_string_wrappers_store_bounded_string() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::E2A6B9C4),
        crate::types::SourceTextRef::from(constants_str::BOUNDEDSTRING),
        |path, ast, ers| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::BoundedStringStorageVisitor::new(
                    crate::types::DiagnosticMsgs::default(),
                ),
            );
            ers.extend(
                visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_bounded_string_storage_visitor_rejects_raw_string_and_old_derive() {
    let ast: syn::File = syn::parse_quote! {
        #[derive(proc_macro_newtype::BoundedString)]
        #[bounded_string(max = 8usize)]
        struct Value(String);
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::BoundedStringStorageVisitor::new(
            crate::types::DiagnosticMsgs::default(),
        ),
    );
    assert_eq!(
        visitor.get_ers().len(),
        2,
        "9301b84f {:#?}",
        visitor.get_ers()
    );
}
#[test]
fn test_newtype_try_from_validator_satisfies_string_wrapper_policy() {
    let ast: syn::File = syn::parse_quote! {
            #[derive(proc_macro_newtype::TryFrom)]
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
    let string_wrapper_names =
        crate::code_style::string_wrapper_names(crate::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        crate::code_style::len_checked_function_names(crate::types::SynFileRef::from(&ast));
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            &len_checked_function_names,
            &string_wrapper_names,
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    assert!(
        visitor
            .get_try_from_string_names()
            .contains(constants_str::CODE_STYLE_VALUE),
        "4d8a4c7e"
    );
    assert!(
        visitor
            .get_try_from_string_len_checked_names()
            .contains(constants_str::CODE_STYLE_VALUE),
        "7c2e4b50"
    );
}
#[test]
fn test_newtype_try_from_explicit_error_satisfies_string_wrapper_policy() {
    let ast: syn::File = syn::parse_quote! {
        #[derive(proc_macro_newtype::TryFrom)]
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
    let string_wrapper_names =
        crate::code_style::string_wrapper_names(crate::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        crate::code_style::len_checked_function_names(crate::types::SynFileRef::from(&ast));
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            &len_checked_function_names,
            &string_wrapper_names,
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    assert!(
        visitor
            .get_try_from_string_names()
            .contains(constants_str::CODE_STYLE_VALUE),
        "89c632cd"
    );
    assert!(
        visitor
            .get_try_from_string_len_checked_names()
            .contains(constants_str::CODE_STYLE_VALUE),
        "b49bc6d0"
    );
}
#[test]
fn test_manual_try_from_delegated_validator_satisfies_string_wrapper_policy() {
    let ast: syn::File = syn::parse_quote! {
        struct Value(String);
        fn validate_value(value: &str) -> Result<(), ValueError> {
            if value.len() > 8usize {
                Err(ValueError)
            } else {
                Ok(())
            }
        }
        impl TryFrom<String> for Value {
            type Error = ValueError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                validate_value(value.as_str())?;
                Ok(Self(value))
            }
        }
    };
    let string_wrapper_names =
        crate::code_style::string_wrapper_names(crate::types::SynFileRef::from(&ast));
    let len_checked_function_names =
        crate::code_style::len_checked_function_names(crate::types::SynFileRef::from(&ast));
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::StringWrapperFromVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            &len_checked_function_names,
            &string_wrapper_names,
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    assert!(
        visitor
            .get_try_from_string_names()
            .contains(constants_str::CODE_STYLE_VALUE),
        "49c5a28f"
    );
    assert!(
        visitor
            .get_try_from_string_len_checked_names()
            .contains(constants_str::CODE_STYLE_VALUE),
        "21d03c6b"
    );
}
#[test]
fn test_tuple_wrappers_do_not_expose_inner_field() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::B7C84E2A),
        crate::types::SourceTextRef::from(constants_str::PUBLIC_TUPLE_WRAPPERS_MUST_NOT_EXPOSE_INNER_FIELDS_INITIALIZE_THEM_THROUGH_FROM),
        |path, ast, ers| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::PublicTupleWrapperFieldVisitor::new(crate::types::DiagnosticMsgs::default()),
            );
            ers.extend(
                visitor
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_wrapper_deserialization_uses_from_or_try_from() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::B7C84E2A),
        crate::types::SourceTextRef::from(constants_str::VALUE_532F14A8),
        |path, ast, ers| {
            let collector = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::TupleWrapperConversionCollector::new(
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    std::collections::BTreeMap::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                ),
            );
            let derive_visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::DirectDeserializeTupleWrapperVisitor::new(
                    crate::types::DiagnosticMsgs::default(),
                ),
            );
            ers.extend(
                derive_visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
            let manual_visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::ManualDeserializeTupleWrapperVisitor::new(
                    crate::types::DiagnosticMsgs::default(),
                    collector.get_names(),
                ),
            );
            ers.extend(
                manual_visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_tuple_wrapper_deserialization_policy_rejects_direct_derive() {
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
    let collector = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::TupleWrapperConversionCollector::new(
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            std::collections::BTreeMap::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    let derive_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::DirectDeserializeTupleWrapperVisitor::new(
            crate::types::DiagnosticMsgs::default(),
        ),
    );
    assert_eq!(derive_visitor.get_ers().len(), 1, "b0406560");
    assert!(
        derive_visitor.get_ers().first().is_some_and(|error| {
            error.contains(constants_str::VALUE_55F7B06B)
                && error.contains(constants_str::VALUE_43E042B0)
        }),
        "f103b2f0"
    );
    let manual_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::ManualDeserializeTupleWrapperVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            collector.get_names(),
        ),
    );
    assert_eq!(manual_visitor.get_ers().len(), 1, "fbe61e18");
    assert!(
        manual_visitor.get_ers().first().is_some_and(|error| {
            error.contains(constants_str::VALUE_F8243212)
                && error.contains(constants_str::VALUE_98BCCBF1)
        }),
        "10fa222c"
    );
}
#[test]
fn test_tuple_wrappers_initialize_only_through_from_or_try_from() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::B7C84E2A),
        crate::types::SourceTextRef::from(constants_str::VALUE_15F71E67),
        |path, ast, ers| {
            let collector = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::TupleWrapperConversionCollector::new(
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    std::collections::BTreeMap::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                    crate::types::SourceTextBTreeSet::default(),
                ),
            );
            ers.extend(
                collector
                    .get_names()
                    .difference(collector.get_converted_names())
                    .map(|name| {
                        format!(
                            "{}: tuple wrapper `{name}` has no From/TryFrom implementation",
                            path.display()
                        )
                    }),
            );
            ers.extend(
                collector
                    .get_from_inner_names()
                    .intersection(collector.get_try_from_inner_names())
                    .map(|name| {
                        format!(
                            "{}: tuple wrapper `{name}` implements both From and TryFrom for its inner type",
                            path.display()
                        )
                    }),
            );
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::DirectTupleWrapperConstructorVisitor::new(
                    None,
                    crate::types::DiagnosticMsgs::default(),
                    crate::types::AnalyzerBool::default(),
                    collector.get_names(),
                ),
            );
            ers.extend(
                visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}

#[test]
fn test_tuple_wrapper_rejects_from_and_try_from_for_same_inner_type() {
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

    let collector = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::TupleWrapperConversionCollector::new(
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            std::collections::BTreeMap::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    let conflicts: std::collections::BTreeSet<String> = collector
        .get_from_inner_names()
        .intersection(collector.get_try_from_inner_names())
        .cloned()
        .collect();
    assert_eq!(
        conflicts,
        std::collections::BTreeSet::from([String::from(constants_str::VALUE_014659AB)]),
        "4c8f5a3d {conflicts:#?}"
    );
}

#[test]
fn test_tuple_wrapper_initialization_policy_rejects_direct_constructors() {
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
    let collector = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::TupleWrapperConversionCollector::new(
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            std::collections::BTreeMap::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
            crate::types::SourceTextBTreeSet::default(),
        ),
    );
    assert_eq!(collector.get_names().len(), 2, "b058f76c");
    assert_eq!(collector.get_converted_names().len(), 1, "70552188");
    assert_eq!(
        collector
            .get_names()
            .difference(collector.get_converted_names())
            .count(),
        1,
        "5cbdfec2"
    );

    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::DirectTupleWrapperConstructorVisitor::new(
            None,
            crate::types::DiagnosticMsgs::default(),
            crate::types::AnalyzerBool::default(),
            collector.get_names(),
        ),
    );
    assert_eq!(visitor.get_ers().len(), 2, "dd79f331");
}
#[test]
fn test_domain_boundaries_use_repository_declared_types() {
    let repo_crates = crate::code_style::workspace_crate_names();
    let mut names = std::collections::BTreeSet::new();
    crate::code_style::for_each_rs_file(|file| {
        let visitor = crate::code_style::visit_syn_file(
            crate::types::SynFileRef::from(file.ast().as_ref()),
            super::domain_analysis::DeclaredDomainTypeVisitor::new(
                crate::types::SourceTextBTreeSet::default(),
            ),
        );
        names.extend(visitor.get_names().iter().cloned());
    });
    let repo_types = crate::types::SourceTextBTreeSet::from(names);
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::A7F9C3E1),
        crate::types::SourceTextRef::from(constants_str::RAW_EXTERNAL_OR_PRIMITIVE_TYPES_FOUND_IN_DOMAIN_BOUNDARIES_USE_REPOSITORY_DOMAIN),
        |path, ast, ers| {
            if !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(path))
                .get()
                || crate::code_style::is_test_crate_source_path(crate::types::PathRef::from(path)).get()
                || crate::code_style::is_code_style_meta_harness_source_path(crate::types::PathRef::from(
                    path,
                ))
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::DomainTypePolicyVisitor::new(crate::types::AnalyzerBool::default(), crate::types::AnalyzerCount::default(), crate::types::DiagnosticMsgs::default(), Vec::new(), crate::types::SourceTextBTreeSetRef::from(repo_crates.as_ref()), crate::types::SourceTextBTreeSetRef::from(repo_types.as_ref())),
            );
            ers.extend(
                visitor
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
            let local_visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::RawTextLocalVisitor::new(
                    crate::types::DiagnosticMsgs::default(),
                ),
            );
            ers.extend(
                local_visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_environment_initializer_is_in_domain_boundary_policy_scope() {
    assert!(
        crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_4638547C)
        ))
        .get(),
        "c4a791e2"
    );
}
#[test]
fn test_workspace_test_runner_uses_test_crate_domain_boundary() {
    assert!(
        !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_SRC)
        ))
        .get(),
        "446a3bb7"
    );
}
#[test]
fn test_workspace_scaffold_is_in_domain_boundary_policy_scope() {
    assert!(
        crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::WORKSPACE_SCAFFOLD_SRC)
        ))
        .get(),
        "c1a7e4d9"
    );
}
#[test]
fn test_server_admin_frontend_is_in_domain_boundary_policy_scope() {
    assert!(
        crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::SERVER_ADMIN_FRONTEND_SRC_APP_RS)
        ))
        .get(),
        "73e9c20f"
    );
}

#[test]
fn test_server_admin_frontend_ui_is_an_explicit_framework_adapter_boundary() {
    assert!(
        !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_BAF40B19)
        ))
        .get(),
        "e33b8472"
    );
    assert!(
        crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::SERVER_ADMIN_FRONTEND_SRC_APP_NAVIGATION_RS)
        ))
        .get(),
        "29bc703d"
    );
}
#[test]
fn test_domain_fixture_and_benchmark_directory_boundaries_are_exact() {
    assert!(
        !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_68737D5D)
        ))
        .get(),
        "4ab6e2d1"
    );
    assert!(
        crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_C80A8856)
        ))
        .get(),
        "d8c3175f"
    );
    assert!(
        !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_B9B617A7)
        ))
        .get(),
        "09e5a6bc"
    );
    assert!(
        !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(
            std::path::Path::new(constants_str::VALUE_0890A317)
        ))
        .get(),
        "e5f21c07"
    );
}
#[test]
fn test_domain_type_policy_reports_raw_browser_external_types_natively() {
    let ast =
        syn::parse_file(constants_str::VALUE_B7CF0D16).expect(constants_str::DIAGNOSTIC_D031EA92);
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::new();
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor::new(
            crate::types::AnalyzerBool::from(true),
            crate::types::AnalyzerCount::default(),
            crate::types::DiagnosticMsgs::default(),
            Vec::new(),
            crate::types::SourceTextBTreeSetRef::from(&repo_crates),
            crate::types::SourceTextBTreeSetRef::from(&repo_types),
        ),
    );
    assert_eq!(
        visitor.get_ers().len(),
        constants_usize::ONE,
        "79ce162a {:#?}",
        visitor.get_ers()
    );
    assert!(
        visitor
            .get_ers()
            .first()
            .is_some_and(|error| error.contains(constants_str::VALUE_A7FCF9B8)),
        "bd624f03"
    );
}
#[test]
fn test_proc_macro_helpers_are_checked_while_compiler_entrypoints_are_exempt() {
    let ast =
        syn::parse_file(constants_str::VALUE_1FB67C5A).expect(constants_str::DIAGNOSTIC_5A1D8C34);
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::new();
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor::new(
            crate::types::AnalyzerBool::from(true),
            crate::types::AnalyzerCount::default(),
            crate::types::DiagnosticMsgs::default(),
            Vec::new(),
            crate::types::SourceTextBTreeSetRef::from(&repo_crates),
            crate::types::SourceTextBTreeSetRef::from(&repo_types),
        ),
    );
    assert_eq!(
        visitor.get_ers().len(),
        2usize,
        "c82fb6d1 {:#?}",
        visitor.get_ers()
    );
    assert!(
        visitor
            .get_ers()
            .iter()
            .all(|error| error.contains(constants_str::VALUE_E81D3B0E)),
        "109eb4a7"
    );
}
#[test]
fn test_domain_type_policy_checks_explicit_closure_parameter_types() {
    let ast = syn::parse_file(
        constants_str::NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_FN_DEMO_NEWLINE_LET_PATH_CB,
    )
    .expect(constants_str::DIAGNOSTIC_C81A6F20);
    let repo_crates = std::collections::BTreeSet::new();
    let repo_types = std::collections::BTreeSet::from([String::from(constants_str::SOURCETEXT)]);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor::new(
            crate::types::AnalyzerBool::from(true),
            crate::types::AnalyzerCount::default(),
            crate::types::DiagnosticMsgs::default(),
            Vec::new(),
            crate::types::SourceTextBTreeSetRef::from(&repo_crates),
            crate::types::SourceTextBTreeSetRef::from(&repo_types),
        ),
    );
    let ers = visitor
        .get_ers()
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    assert_eq!(ers.len(), 2, "0f6d3a91 {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_8212AC3A)),
        "d4b2f8a0 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_27005C7D)),
        "60b8ae2d {ers:#?}"
    );
}

#[test]
fn test_domain_type_policy_checks_explicit_local_types() {
    let ast: syn::File = syn::parse_quote! {
        fn example() {
            let raw: String = String::new();
            let wrapped: SourceText = SourceText::try_from(String::new());
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::RawTextLocalVisitor::new(crate::types::DiagnosticMsgs::default()),
    );
    assert_eq!(visitor.get_ers().len(), 1, "90d1487c");
    assert!(
        visitor
            .get_ers()
            .first()
            .is_some_and(|error| error.contains(constants_str::VALUE_CBBF3E32)),
        "0db73fec {:#?}",
        visitor.get_ers()
    );
}

#[test]
fn test_domain_type_policy_allows_only_option_and_result_containers() {
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
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::DomainTypePolicyVisitor::new(
            crate::types::AnalyzerBool::from(true),
            crate::types::AnalyzerCount::default(),
            crate::types::DiagnosticMsgs::default(),
            Vec::new(),
            crate::types::SourceTextBTreeSetRef::from(&repo_crates),
            crate::types::SourceTextBTreeSetRef::from(&repo_types),
        ),
    );
    let ers = visitor
        .get_ers()
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    assert_eq!(ers.len(), 1, "c47a91e2 {ers:#?}");
    assert!(
        ers.first()
            .is_some_and(|error| error.contains(constants_str::VALUE_14A14329)),
        "d8b305f6 {ers:#?}"
    );
}
#[test]
fn test_analyzer_state_struct_fields_use_repository_declared_wrappers() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::F2C7A91B),
        crate::types::SourceTextRef::from(constants_str::RAW_TEXT_CONTAINERS_FOUND_IN_HELPER_STRUCT_FIELDS_USE_REPOSITORY_WRAPPER_TYPES),
        |path, ast, ers| {
            if !crate::code_style::domain_type_policy_should_check_path(crate::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::AnalyzerStateRawContainerFieldVisitor::new(crate::types::DiagnosticMsgs::default()),
            );
            ers.extend(
                visitor
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_analyzer_state_raw_container_field_visitor_reports_helper_fields() {
    let ast = syn::parse_file(constants_str::NEWLINE_STRUCT_HELPERSTATE_NEWLINE_NAMES_VEC_STRING_NEWLINE_SEEN_STD_PATH_COLLECTIONS).expect(constants_str::DIAGNOSTIC_9F4D2A7C);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::AnalyzerStateRawContainerFieldVisitor::new(
            crate::types::DiagnosticMsgs::default(),
        ),
    );
    let ers = visitor
        .get_ers()
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    assert_eq!(ers.len(), 3, "2c0b7e91 {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_7C64B919)),
        "74e18b2d {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_F6C0ACAB)),
        "4a0df351 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_6FCF10B8)),
        "81c6a2ef {ers:#?}"
    );
}
#[test]
fn test_helper_return_types_use_repository_declared_text_wrappers() {
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::VALUE_6D41C8E2),
        crate::types::SourceTextRef::from(constants_str::RAW_TEXT_RETURN_TYPES_FOUND_IN_HELPER_FUNCTIONS_USE_REPOSITORY_WRAPPER_TYPES),
        |path, ast, ers| {
            if !crate::code_style::is_code_style_meta_harness_source_path(crate::types::PathRef::from(path))
                .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::HelperRawTextReturnVisitor::new(crate::types::DiagnosticMsgs::default()),
            );
            ers.extend(
                visitor
                    .get_ers().clone().into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_helper_raw_text_return_visitor_reports_free_and_inherent_helpers() {
    let ast = syn::parse_file(
        constants_str::NEWLINE_FN_DIRECT_ARROW_STRING_NEWLINE_STRING_PATH_NEW_NEWLINE_NEWLINE_FN,
    )
    .expect(constants_str::DIAGNOSTIC_3A9D7E2C);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::HelperRawTextReturnVisitor::new(
            crate::types::DiagnosticMsgs::default(),
        ),
    );
    let ers = visitor
        .get_ers()
        .clone()
        .into_iter()
        .collect::<Vec<String>>();
    assert_eq!(ers.len(), 4, "7b2e41a0 {ers:#?}");
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_801070C4)),
        "08d4b6ea {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_EB19F83F)),
        "ae71c3f4 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_8AB55626)),
        "59f0bca8 {ers:#?}"
    );
    assert!(
        ers.iter()
            .any(|error| error.contains(constants_str::VALUE_F834D834)),
        "c46d8e10 {ers:#?}"
    );
}
#[test]
fn test_external_leaf_tuple_wrappers_include_source_name() {
    let repo_crates = crate::code_style::workspace_crate_names();
    crate::code_style::assert_rs_ast_ers_empty_with_ctx(
        crate::types::StaticStr::from(constants_str::B93D2A8C),
        crate::types::SourceTextRef::from(
            constants_str::TUPLE_WRAPPERS_OVER_EXTERNAL_TYPES_MUST_INCLUDE_THE_SOURCE_NAME,
        ),
        |path, ast, ers| {
            if !crate::code_style::domain_type_policy_should_check_path(
                crate::types::PathRef::from(path),
            )
            .get()
            {
                return;
            }
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(ast),
                super::domain_analysis::ExternalLeafWrapperNameVisitor::new(
                    crate::types::DiagnosticMsgs::default(),
                    crate::types::SourceTextBTreeSetRef::from(repo_crates.as_ref()),
                ),
            );
            ers.extend(
                visitor
                    .get_ers()
                    .clone()
                    .into_iter()
                    .map(|error| format!("{}: {error}", path.display())),
            );
        },
    );
}
#[test]
fn test_external_leaf_wrapper_type_rule_has_no_name_exceptions() {
    let ast: syn::File = syn::parse_quote! {
        struct GeneratedTokens(proc_macro2::TokenStream);
        struct ProcMacro2GeneratedTokens(proc_macro2::TokenStream);
    };
    let repo_crates = std::collections::BTreeSet::new();
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        super::domain_analysis::ExternalLeafWrapperNameVisitor::new(
            crate::types::DiagnosticMsgs::default(),
            crate::types::SourceTextBTreeSetRef::from(&repo_crates),
        ),
    );
    assert_eq!(
        visitor.get_ers().len(),
        constants_usize::ONE,
        "9db6310a {:#?}",
        visitor.get_ers()
    );
    assert!(
        visitor
            .get_ers()
            .first()
            .is_some_and(|error| error.contains(constants_str::VALUE_9CE15201)),
        "e7340ba2"
    );
}
