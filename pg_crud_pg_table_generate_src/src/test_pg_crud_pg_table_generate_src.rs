#[test]
fn test_table_revision_conflicts_roll_back_and_release_only_enabled_idempotency() {
    assert!([false, true].into_iter().all(|revision_enabled| {
        [false, true].into_iter().all(|idempotency_enabled| {
            let revision_setting = if revision_enabled {
                quote::quote! { "revision" }
            } else {
                quote::quote! { null }
            };
            let input = quote::quote! {
                #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                    "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False",
                    "optimistic_revision_field": #revision_setting, "idempotent_mutations": #idempotency_enabled
                })]
                struct Table {
                    #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                    name: StringAsNonNullText,
                    revision: I64AsNonNullInt8
                }
            };
            let generated = crate::generate_pg_table::generate_pg_table(
                macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
            );
            let output = generated.to_string();
            assert!(!output.contains(stringify!(compile_error)));
            assert!(syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated))
                .is_ok_and(|file| !file.items.is_empty()));
            let mismatch = quote::quote! { if v.len() != update_for_query_vec.len() }.to_string();
            let conflict_return = quote::quote! {
                return axum::response::IntoResponse::into_response(http::StatusCode::PRECONDITION_FAILED);
            }.to_string();
            let conflict_branch = output.split_once(&mismatch);
            assert_eq!(conflict_branch.is_some(), revision_enabled);
            conflict_branch.is_none_or(|(_, branch_tail)| {
                branch_tail.split_once(&conflict_return).is_some_and(|(before_return, _)| {
                    let rollback = before_return.find(&quote::quote! { executor.rollback().await }.to_string());
                    let release = before_return.find(&quote::quote! {
                        pg_table::release_pg_table_idempotency::release_pg_table_idempotency
                    }.to_string());
                    assert!(rollback.is_some());
                    assert_eq!(release.is_some(), idempotency_enabled);
                    assert!(!before_return.contains(&quote::quote! { executor.commit().await }.to_string()));
                    release.is_none_or(|release_position| rollback.is_some_and(|rollback_position| rollback_position < release_position))
                })
            })
        })
    }));
}

#[test]
fn test_validation_rejects_non_struct_input_without_emitting_source() {
    let input = quote::quote! { enum NotATable { Value } };
    let parsed = crate::parse_generate_pg_table::parse_generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_5D4F86A1);
    assert!(matches!(
        crate::build_generate_pg_table::build_generate_pg_table(parsed),
        Err(crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Build(_error))
    ));
}

#[test]
fn test_build_stage_exposes_typed_model_without_emitting_source() {
    let input = quote::quote! { struct Table { id: i64, name: String } };
    let parsed = crate::parse_generate_pg_table::parse_generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_0F8B43D2);
    let built = crate::build_generate_pg_table::build_generate_pg_table(parsed)
        .expect(constants_str::DIAGNOSTIC_A715E9C4);
    assert_eq!(usize::from(built.get().field_count()), 2usize);
}

#[test]
fn test_validation_rejects_empty_table_model_without_emitting_source() {
    let input = quote::quote! { struct EmptyTable; };
    let parsed = crate::parse_generate_pg_table::parse_generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    )
    .expect(constants_str::DIAGNOSTIC_67D029AB);
    let built = crate::build_generate_pg_table::build_generate_pg_table(parsed)
        .expect(constants_str::DIAGNOSTIC_C15B8F34);
    assert!(matches!(
        crate::validate_generate_pg_table::validate_generate_pg_table(built),
        Err(
            crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Validate(_error)
        )
    ));
}

#[test]
fn test_table_config_rejects_zero_bulk_limits_and_invalid_database_names() {
    let non_ascii_name = proc_macro2::Literal::string(&'\u{00e9}'.to_string());
    let cases = [
        (
            quote::quote! { "create_many_max_items": 0 },
            constants_str::COMPILE_ERROR_CE_013,
        ),
        (
            quote::quote! { "update_many_max_items": 0 },
            constants_str::COMPILE_ERROR_CE_013,
        ),
        (
            quote::quote! { "rule_prefix": "" },
            constants_str::COMPILE_ERROR_CE_051,
        ),
        (
            quote::quote! { "rule_prefix": "Upper" },
            constants_str::COMPILE_ERROR_CE_051,
        ),
        (
            quote::quote! { "rule_prefix": "with-dash" },
            constants_str::COMPILE_ERROR_CE_051,
        ),
        (
            quote::quote! { "rule_prefix": #non_ascii_name },
            constants_str::COMPILE_ERROR_CE_051,
        ),
        (
            quote::quote! { "db_table_name": "" },
            constants_str::COMPILE_ERROR_CE_083,
        ),
        (
            quote::quote! { "db_table_name": "Upper" },
            constants_str::COMPILE_ERROR_CE_083,
        ),
        (
            quote::quote! { "db_table_name": "with-dash" },
            constants_str::COMPILE_ERROR_CE_083,
        ),
        (
            quote::quote! { "db_table_name": #non_ascii_name },
            constants_str::COMPILE_ERROR_CE_083,
        ),
    ];
    assert!(cases.into_iter().all(|(configuration, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False",
                #configuration
            })]
            struct Table { id: i64 }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote! { compile_error!(#diagnostic); }.to_string()
        );
        true
    }));
}

#[test]
fn test_table_error_variant_attributes_reject_invalid_enum_shapes_and_field_annotations() {
    let cases = [
        (
            quote::quote! { enum WrongName { Failure { source: String } } },
            constants_str::COMPILE_ERROR_CE_022,
        ),
        (
            quote::quote! { enum CreateManyErrorVariants { Failure } },
            constants_str::COMPILE_ERROR_CE_004,
        ),
        (
            quote::quote! { enum CreateManyErrorVariants { Failure(String) } },
            constants_str::COMPILE_ERROR_CE_004,
        ),
        (
            quote::quote! { enum CreateManyErrorVariants { Failure { source: String } } },
            constants_str::COMPILE_ERROR_CE_023,
        ),
        (
            quote::quote! { enum CreateManyErrorVariants { Failure {
                #[error_field_to_err_string]
                #[error_field_to_err_string]
                source: String
            } } },
            constants_str::COMPILE_ERROR_CE_029,
        ),
        (
            quote::quote! { enum CreateManyErrorVariants { Failure {
                #[other::error_field_to_err_string] source: String
            } } },
            constants_str::COMPILE_ERROR_CE_023,
        ),
        (
            quote::quote! { enum CreateManyErrorVariants { Failure {
                #[unrelated] source: String
            } } },
            constants_str::COMPILE_ERROR_CE_023,
        ),
    ];
    assert!(cases.into_iter().all(|(error_variants, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            })]
            #[proc_macro_generate_pg_table::create_many_error_variants(#error_variants)]
            struct Table { id: i64 }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote! { compile_error!(#diagnostic); }.to_string()
        );
        true
    }));
}

#[test]
fn test_table_fields_validate_primary_key_count_and_column_name_length() {
    let maximum_column_name = proc_macro2::Ident::new(
        &constants_str::X.repeat(63usize),
        proc_macro2::Span::call_site(),
    );
    let oversized_column_name = proc_macro2::Ident::new(
        &constants_str::X.repeat(64usize),
        proc_macro2::Span::call_site(),
    );
    let cases = [
        (
            quote::quote! { struct Table { id: i64 } },
            constants_str::COMPILE_ERROR_CE_015,
        ),
        (
            quote::quote! { struct Table { #maximum_column_name: i64 } },
            constants_str::COMPILE_ERROR_CE_015,
        ),
        (
            quote::quote! { struct Table { #oversized_column_name: i64 } },
            constants_str::COMPILE_ERROR_CE_002,
        ),
        (
            quote::quote! { struct Table {
                #[generate_pg_table_primary_key] first: i64,
                #[generate_pg_table_primary_key] second: i64
            } },
            constants_str::COMPILE_ERROR_CE_003,
        ),
        (
            quote::quote! { struct Table(i64); },
            constants_str::COMPILE_ERROR_CE_018,
        ),
    ];
    assert!(cases.into_iter().all(|(table, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            })]
            #table
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote! { compile_error!(#diagnostic); }.to_string()
        );
        true
    }));
}

#[test]
fn test_table_exclusion_lists_reject_primary_keys_unknown_fields_and_duplicate_reads() {
    let cases = [
        (
            quote::quote! { "create_exclude_fields": ["id"] },
            constants_str::COMPILE_ERROR_CE_017,
        ),
        (
            quote::quote! { "create_exclude_fields": ["missing"] },
            constants_str::COMPILE_ERROR_CE_017,
        ),
        (
            quote::quote! { "create_exclude_fields": ["name", "missing"] },
            constants_str::COMPILE_ERROR_CE_017,
        ),
        (
            quote::quote! { "read_exclude_fields": ["id"] },
            constants_str::COMPILE_ERROR_CE_027,
        ),
        (
            quote::quote! { "read_exclude_fields": ["missing"] },
            constants_str::COMPILE_ERROR_CE_027,
        ),
        (
            quote::quote! { "read_exclude_fields": ["name", "missing"] },
            constants_str::COMPILE_ERROR_CE_027,
        ),
        (
            quote::quote! { "read_exclude_fields": ["name", "name"] },
            constants_str::COMPILE_ERROR_CE_027,
        ),
    ];
    assert!(cases.into_iter().all(|(configuration, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False",
                #configuration
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: i64,
                name: String
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote! { compile_error!(#diagnostic); }.to_string()
        );
        true
    }));
}

#[test]
fn test_table_optimistic_revision_rejects_missing_primary_and_incompatible_fields() {
    let cases = [
        (
            quote::quote! { "missing" },
            quote::quote! { String },
            constants_str::COMPILE_ERROR_CE_055,
        ),
        (
            quote::quote! { "id" },
            quote::quote! { I64AsNonNullInt8 },
            constants_str::COMPILE_ERROR_CE_012,
        ),
        (
            quote::quote! { "revision" },
            quote::quote! { String },
            constants_str::COMPILE_ERROR_CE_012,
        ),
        (
            quote::quote! { "revision" },
            quote::quote! { I64AsNullableInt8 },
            constants_str::COMPILE_ERROR_CE_012,
        ),
        (
            quote::quote! { "revision" },
            quote::quote! { &I64AsNonNullInt8 },
            constants_str::COMPILE_ERROR_CE_012,
        ),
        (
            quote::quote! { "revision" },
            quote::quote! { (I64AsNonNullInt8,) },
            constants_str::COMPILE_ERROR_CE_012,
        ),
    ];
    assert!(cases.into_iter().all(|(revision_name, revision_type, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False",
                "optimistic_revision_field": #revision_name
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                revision: #revision_type
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(generated.to_string(), quote::quote! { compile_error!(#diagnostic); }.to_string());
        true
    }));
}

#[test]
fn test_table_read_page_rejects_unreadable_search_columns_and_invalid_rust_paths() {
    let search_error = quote::quote! { compile_error!("read page search columns must be readable table columns"); };
    let path_error =
        quote::quote! { compile_error!("read page extension paths must be valid Rust paths"); };
    let context_error = quote::quote! { compile_error!("read page context field must contain a valid Rust type and identifier"); };
    let cases = [
        (
            quote::quote! { "search_columns": ["missing"], "response": "Response", "enrich": "enrich", "error": "Error" },
            &search_error,
        ),
        (
            quote::quote! { "search_columns": ["name"], "response": "Response", "enrich": "enrich", "error": "Error" },
            &search_error,
        ),
        (
            quote::quote! { "search_columns": [], "response": "-", "enrich": "enrich", "error": "Error" },
            &path_error,
        ),
        (
            quote::quote! { "search_columns": [], "response": "Response", "enrich": "-", "error": "Error" },
            &path_error,
        ),
        (
            quote::quote! { "search_columns": [], "response": "Response", "enrich": "enrich", "error": "-" },
            &path_error,
        ),
        (
            quote::quote! { "search_columns": [], "response": "Response", "enrich": "enrich", "error": "Error", "context_field": { "rust_type": "-", "name": "context" } },
            &context_error,
        ),
        (
            quote::quote! { "search_columns": [], "response": "Response", "enrich": "enrich", "error": "Error", "context_field": { "rust_type": "Context", "name": "with-dash" } },
            &context_error,
        ),
    ];
    assert!(cases.into_iter().all(|(read_page, expected)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False",
                "read_exclude_fields": ["name"],
                "read_page": { #read_page }
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                name: String
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(generated.to_string(), expected.to_string());
        true
    }));
}

#[test]
fn test_table_frontend_order_rejects_collisions_with_explicit_default_and_hidden_columns() {
    let cases = [
        quote::quote! {
            #[generate_pg_table_primary_key]
            #[generate_pg_table_frontend(order = 3)] id: I64AsNonNullInt8,
            #[generate_pg_table_frontend(order = 3)] name: String
        },
        quote::quote! {
            #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
            #[generate_pg_table_frontend(order = 0)] name: String
        },
        quote::quote! {
            #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
            #[generate_pg_table_frontend(hidden, order = 0)] name: String
        },
    ];
    let diagnostic = constants_str::COMPILE_ERROR_CE_011;
    assert!(cases.into_iter().all(|fields| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            })]
            struct Table { #fields }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(
            generated.to_string(),
            quote::quote! { compile_error!(#diagnostic); }.to_string()
        );
        true
    }));
}

#[test]
fn test_table_frontend_options_reject_duplicate_annotations_and_invalid_labels() {
    let cases = [
        (
            quote::quote! { #[generate_pg_table_frontend(hidden)] #[generate_pg_table_frontend(sortable)] },
            constants_str::DUPLICATE_GENERATE_PG_TABLE_FRONTEND_ATTRIBUTE,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(filterable, filterable)] },
            constants_str::DUPLICATE_FILTERABLE_OPTION,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(hidden, hidden)] },
            constants_str::DUPLICATE_HIDDEN_OPTION,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(sortable, sortable)] },
            constants_str::DUPLICATE_SORTABLE_OPTION,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(label = "first", label = "second")] },
            constants_str::DUPLICATE_LABEL_OPTION,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(order = 1, order = 2)] },
            constants_str::DUPLICATE_ORDER_OPTION,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(placeholder = "first", placeholder = "second")] },
            constants_str::DUPLICATE_PLACEHOLDER_OPTION,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(label = "")] },
            constants_str::FRONTEND_LABEL_MUST_NOT_BE_EMPTY,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(label = "   ")] },
            constants_str::FRONTEND_LABEL_MUST_NOT_BE_EMPTY,
        ),
        (
            quote::quote! { #[generate_pg_table_frontend(unknown)] },
            constants_str::UNSUPPORTED_GENERATE_PG_TABLE_FRONTEND_OPTION,
        ),
    ];
    assert!(cases.into_iter().all(|(attributes, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                #attributes name: String
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
            item.mac
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == stringify!(compile_error))
                && syn::parse2::<syn::LitStr>(item.mac.tokens)
                    .is_ok_and(|message| message.value() == diagnostic)
        })
    }));
}

#[test]
fn test_table_schema_emits_each_foreign_key_delete_action_and_jsonb_override() {
    let cases = [
        (stringify!(Cascade), quote::quote! { Cascade }),
        (stringify!(NoAction), quote::quote! { NoAction }),
        (stringify!(Restrict), quote::quote! { Restrict }),
        (stringify!(SetDefault), quote::quote! { SetDefault }),
        (stringify!(SetNull), quote::quote! { SetNull }),
    ];
    assert!(cases.into_iter().all(|(action_name, action)| {
        let action_literal = proc_macro2::Literal::string(action_name);
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False",
                "db_column_type_overrides": [{"column": "name", "data_type": "jsonb"}],
                "db_unique_keys": [["name", "id"]],
                "db_foreign_keys": [{"columns": ["name", "id"], "referenced_columns": ["parent_name", "parent_id"], "referenced_table": "parents", "on_delete": #action_literal}]
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                name: StringAsNonNullText
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        assert!(output.contains(&quote::quote! { pg_crud_common::db_static_schema_text::DbStaticSchemaText::from(stringify!(jsonb)) }.to_string()));
        let expected_unique = quote::quote! {
            pg_crud_common::db_key_spec::DbKeySpec::Unique(vec![
                pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("name"),
                pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("id")
            ].into())
        };
        let expected_foreign = quote::quote! {
            pg_crud_common::db_key_spec::DbKeySpec::ForeignKey {
                columns: vec![
                    pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("name"),
                    pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("id")
                ].into(),
                on_delete: pg_crud_common::db_foreign_key_delete_action::DbForeignKeyDeleteAction::#action,
                referenced_columns: vec![
                    pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("parent_name"),
                    pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("parent_id")
                ].into(),
                referenced_table: pg_crud_common::db_static_schema_text::DbStaticSchemaText::from("parents"),
            }
        };
        assert!(output.contains(&expected_unique.to_string()));
        assert!(output.contains(&expected_foreign.to_string()));
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| !file.items.is_empty())
    }));
}

#[test]
fn test_table_columns_preserve_type_metadata_and_explicit_default_flags() {
    let cases = [
        (quote::quote! {}, false),
        (quote::quote! { #[generate_pg_table_db_default] }, true),
    ];
    assert!(cases.into_iter().all(|(default_attribute, explicit_default)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False",
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullBigSerialInitializationByPg,
                #default_attribute name: StringAsNonNullText,
                optional: StringAsNullableText
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        let columns = [
            (stringify!(id), quote::quote! { I64AsNonNullBigSerialInitializationByPg }, false),
            (stringify!(name), quote::quote! { StringAsNonNullText }, explicit_default),
            (stringify!(optional), quote::quote! { StringAsNullableText }, false),
        ];
        columns.into_iter().all(|(column_name, field_type, has_explicit_default)| {
            let column_literal = proc_macro2::Literal::string(column_name);
            let expected = quote::quote! {
                pg_crud_common::db_column_spec::DbColumnSpec::new(
                    pg_crud_common::db_static_schema_text::DbStaticSchemaText::from(#column_literal),
                    <#field_type as pg_crud_common::pg_column_schema::PgColumnSchema>::data_type(),
                    pg_crud_common::db_column_nullable::DbColumnNullable::from(<#field_type as pg_crud_common::pg_column_schema::PgColumnSchema>::NULLABLE),
                    pg_crud_common::db_column_has_server_default::DbColumnHasServerDefault::from(<#field_type as pg_crud_common::pg_column_schema::PgColumnSchema>::HAS_SERVER_DEFAULT || #has_explicit_default),
                )
            };
            output.contains(&expected.to_string())
        })
    }));
}

#[test]
fn test_table_api_modes_emit_exact_enabled_route_catalogs() {
    let cases = [
        (
            stringify!(Crud),
            vec![
                stringify!(CreateMany),
                stringify!(Read),
                stringify!(Update),
                stringify!(DeleteMany),
            ],
        ),
        (
            stringify!(AppendOnly),
            vec![stringify!(CreateMany), stringify!(Read)],
        ),
        (
            stringify!(CreateReadDelete),
            vec![
                stringify!(CreateMany),
                stringify!(Read),
                stringify!(DeleteMany),
            ],
        ),
        (stringify!(ReadOnly), vec![stringify!(Read)]),
        (
            stringify!(ReadUpdate),
            vec![stringify!(Read), stringify!(Update)],
        ),
    ];
    assert!(cases.into_iter().all(|(mode, expected)| {
        let mode_literal = proc_macro2::Literal::string(mode);
        [false, true].into_iter().all(|idempotency_enabled| {
        [false, true].into_iter().all(|capabilities_enabled| {
        let revision_setting = if capabilities_enabled { quote::quote! { "revision" } } else { quote::quote! { null } };
        let rule_setting = if capabilities_enabled { quote::quote! { "resources" } } else { quote::quote! { null } };
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False", "api_mode": #mode_literal, "idempotent_mutations": #idempotency_enabled, "optimistic_revision_field": #revision_setting, "rule_prefix": #rule_setting
            })]
            struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText, revision: I64AsNonNullInt8 }
        };
        let generated = crate::generate_pg_table::generate_pg_table(macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input));
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| {
            file.items.iter().any(|item| {
                let syn::Item::Impl(implementation) = item else { return false; };
                let syn::Type::Path(self_type) = implementation.self_ty.as_ref() else { return false; };
                if !self_type.path.is_ident(stringify!(TableRouteContract)) { return false; }
                implementation.items.iter().any(|implementation_item| {
                    let syn::ImplItem::Const(catalog) = implementation_item else { return false; };
                    if catalog.ident != stringify!(ALL) { return false; }
                    let syn::Expr::Array(array) = &catalog.expr else { return false; };
                    array.elems.len() == expected.len() && array.elems.iter().zip(&expected).all(|(entry, operation_name)| {
                        let syn::Expr::Call(call) = entry else { return false; };
                        let Some(syn::Expr::Path(operation)) = call.args.iter().nth(3usize) else { return false; };
                        let Some(syn::Expr::Lit(idempotency)) = call.args.iter().nth(2usize) else { return false; };
                        let syn::Lit::Bool(idempotency_flag) = &idempotency.lit else { return false; };
                        let Some(syn::Expr::Lit(optimistic)) = call.args.iter().nth(4usize) else { return false; };
                        let syn::Lit::Bool(optimistic_flag) = &optimistic.lit else { return false; };
                        let open_api_name = match *operation_name {
                            stringify!(CreateMany) => stringify!(__generated_path_table_create_many_open_api),
                            stringify!(Read) => stringify!(__generated_path_table_read_open_api),
                            stringify!(Update) => stringify!(__generated_path_table_update_open_api),
                            stringify!(DeleteMany) => stringify!(__generated_path_table_delete_many_open_api),
                            _ => return false,
                        };
                        let Some(open_api_implementation) = file.items.iter().find_map(|open_api_item| {
                            let syn::Item::Impl(open_api_implementation) = open_api_item else { return None; };
                            let syn::Type::Path(open_api_self_type) = open_api_implementation.self_ty.as_ref() else { return None; };
                            open_api_self_type.path.is_ident(open_api_name).then_some(open_api_implementation)
                        }) else { return false; };
                        let open_api_output = quote::quote! { #open_api_implementation }.to_string();
                        let idempotency_header = proc_macro2::Literal::string(constants_str::IDEMPOTENCY_KEY);
                        let revision_header = proc_macro2::Literal::string(constants_str::IF_MATCH);
                        assert_eq!(open_api_output.contains(&quote::quote! { .name(#idempotency_header) }.to_string()), idempotency_flag.value);
                        assert_eq!(open_api_output.contains(&quote::quote! { .name(#revision_header) }.to_string()), optimistic_flag.value);
                        let (method_name, status_name, rule_action) = match *operation_name {
                            stringify!(CreateMany) => (stringify!(Post), stringify!(Code201), constants_str::PG_CRUD_CREATE_RULE_ACTION),
                            stringify!(Read) => (stringify!(Post), stringify!(Code200), constants_str::PG_CRUD_READ_RULE_ACTION),
                            stringify!(Update) => (stringify!(Patch), stringify!(Code200), constants_str::PG_CRUD_UPDATE_RULE_ACTION),
                            stringify!(DeleteMany) => (stringify!(Delete), stringify!(Code200), constants_str::PG_CRUD_DELETE_RULE_ACTION),
                            _ => return false,
                        };
                        let Some(syn::Expr::Path(method)) = call.args.iter().nth(1usize) else { return false; };
                        let Some(syn::Expr::Path(status)) = call.args.iter().nth(5usize) else { return false; };
                        let Some(authentication) = call.args.iter().next() else { return false; };
                        let expected_authentication = if capabilities_enabled {
                            let rule_literal = proc_macro2::Literal::string(&format!("{}{}{}", stringify!(resources), ':', rule_action));
                            quote::quote! { TableAuthenticationRequirement::Rule(#rule_literal) }
                        } else { quote::quote! { TableAuthenticationRequirement::Public } };
                        quote::quote! { #authentication }.to_string() == expected_authentication.to_string()
                            && method.path.segments.last().is_some_and(|segment| segment.ident == method_name)
                            && status.path.segments.last().is_some_and(|segment| segment.ident == status_name)
                            && operation.path.segments.last().is_some_and(|segment| segment.ident == *operation_name)
                            && idempotency_flag.value == (idempotency_enabled && *operation_name != stringify!(Read))
                            && optimistic_flag.value == (capabilities_enabled && *operation_name == stringify!(Update))
                    })
                })
            })
        })
        })
        })
    }));
}

#[test]
fn test_table_config_attribute_failures_emit_diagnostics_instead_of_source() {
    let cases = [
        quote::quote! { struct Table { id: i64 } },
        quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config]
            struct Table { id: i64 }
        },
        quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({})]
            struct Table { id: i64 }
        },
        quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": 0,
                "common_write_into_file": "False",
                "whole_write_into_file": "False"
            })]
            struct Table { id: i64 }
        },
    ];
    assert!(cases.into_iter().all(|input| {
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
            item.mac.path.is_ident(stringify!(compile_error))
                && syn::parse2::<syn::LitStr>(item.mac.tokens).is_ok_and(|message| {
                    message.value().contains(stringify!(GeneratePgTableConfig))
                })
        })
    }));
}

#[test]
fn test_table_primary_keys_reject_nullable_types_and_missing_update_fields() {
    let cases = [
        (
            quote::quote! { #[generate_pg_table_primary_key] id: I64AsNullableInt8, name: StringAsNonNullText },
            constants_str::PRIMARY_KEY_TYPE_MUST_BE_NON_NULLABLE,
        ),
        (
            quote::quote! { #[generate_pg_table_primary_key] id: pg_types_numeric::generate_pg_types_mod::I64AsNullableInt8, name: StringAsNonNullText },
            constants_str::PRIMARY_KEY_TYPE_MUST_BE_NON_NULLABLE,
        ),
        (
            quote::quote! { #[generate_pg_table_primary_key] id: OptionalI64AsNonNullInt8, name: StringAsNonNullText },
            constants_str::PRIMARY_KEY_TYPE_MUST_BE_NON_NULLABLE,
        ),
        (
            quote::quote! { #[generate_pg_table_primary_key] id: I64AsNonNullInt8 },
            constants_str::UPDATE_OPERATIONS_REQUIRE_AT_LEAST_ONE_NON_PRIMARY_KEY_FIELD,
        ),
    ];
    assert!(cases.into_iter().all(|(fields, diagnostic)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
            })]
            struct Table { #fields }
        };
        let generated = crate::generate_pg_table::generate_pg_table(macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input));
        syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
            item.mac.path.segments.last().is_some_and(|segment| segment.ident == stringify!(compile_error))
                && syn::parse2::<syn::LitStr>(item.mac.tokens).is_ok_and(|message| message.value() == diagnostic)
        })
    }));
}

#[test]
fn test_table_database_and_route_names_follow_independent_override_precedence() {
    let cases = [
        (
            quote::quote! { "db_table_name": null, "route_resource_name": null },
            quote::quote! { "table" },
            quote::quote! { "/table/create_many" },
        ),
        (
            quote::quote! { "db_table_name": "records", "route_resource_name": null },
            quote::quote! { "records" },
            quote::quote! { "/records/create_many" },
        ),
        (
            quote::quote! { "db_table_name": "records", "route_resource_name": "resources" },
            quote::quote! { "records" },
            quote::quote! { "/resources/create_many" },
        ),
        (
            quote::quote! { "db_table_name": null, "route_resource_name": "resources" },
            quote::quote! { "table" },
            quote::quote! { "/resources/create_many" },
        ),
    ];
    assert!(cases.into_iter().all(|(configuration, database_name, route_path)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False", #configuration
            })]
            struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
        };
        let generated = crate::generate_pg_table::generate_pg_table(macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input));
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        assert!(output.contains(&quote::quote! { const fn db_table_name() -> &'static str { #database_name } }.to_string()));
        assert!(output.contains(&quote::quote! { TableOperation::CreateMany => #route_path }.to_string()));
        assert!(output.contains(&quote::quote! { pub const fn table_name() -> &'static str { "table" } }.to_string()));
        true
    }));
}

#[test]
fn test_table_frontend_metadata_preserves_overrides_exclusions_and_default_labels() {
    let input = quote::quote! {
        #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
            "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False",
            "api_mode": "ReadOnly", "create_exclude_fields": ["display_name"], "read_exclude_fields": ["display_name"]
        })]
        struct Table {
            #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
            #[generate_pg_table_frontend(hidden, filterable, sortable, label = "Friendly", placeholder = "Enter Name", order = 5)]
            display_name: StringAsNonNullText,
            created_at: StringAsNonNullText
        }
    };
    let generated = crate::generate_pg_table::generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    );
    let output = generated.to_string();
    assert!(!output.contains(stringify!(compile_error)));
    let expected = quote::quote! {
        frontend_contract::field_contract::FieldContract::new(
            frontend_contract::field_name::FieldName::from(frontend_contract::contract_str::ContractStr::from("display_name")),
            frontend_contract::field_label::FieldLabel::from(frontend_contract::contract_str::ContractStr::from("Friendly")),
            <StringAsNonNullText as frontend_contract::has_type_contract::HasTypeContract>::type_contract(),
        )
        .with_primary_key(frontend_contract::primary_key_kind::PrimaryKeyKind::NonPrimary)
        .with_creatable(frontend_contract::field_capability::FieldCapability::Disabled)
        .with_filterable(frontend_contract::field_capability::FieldCapability::Enabled)
        .with_filters(<StringAsNonNullText as frontend_contract::has_filter_contracts::HasFilterContracts>::filter_contracts())
        .with_order(frontend_contract::field_order::FieldOrder::from(5usize))
        .with_placeholder(frontend_contract::field_placeholder::FieldPlaceholder::Value(frontend_contract::contract_str::ContractStr::from("Enter Name")))
        .with_readable(frontend_contract::field_capability::FieldCapability::Disabled)
        .with_sortable(frontend_contract::field_capability::FieldCapability::Enabled)
        .with_updatable(frontend_contract::field_capability::FieldCapability::Disabled)
        .with_visibility(frontend_contract::field_visibility::FieldVisibility::Hidden)
    };
    assert!(output.contains(&expected.to_string()));
    let default_label = quote::quote! { frontend_contract::field_label::FieldLabel::from(frontend_contract::contract_str::ContractStr::from("Created At")) };
    assert!(
        output
            .find(&default_label.to_string())
            .zip(output.find(&expected.to_string()))
            .is_some_and(
                |(default_position, override_position)| default_position < override_position
            )
    );
}

#[test]
fn test_table_read_page_preserves_qualified_paths_and_optional_context_and_search() {
    assert!([false, true].into_iter().all(|extensions_enabled| {
        let context_setting = if extensions_enabled {
            quote::quote! { { "rust_type": "crate::context::ReadContext", "name": "context" } }
        } else { quote::quote! { null } };
        let search_setting = if extensions_enabled { quote::quote! { ["name"] } } else { quote::quote! { [] } };
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False",
                "api_mode": "ReadOnly",
                "read_page": {
                    "search_columns": #search_setting,
                    "response": "crate::response::ReadResponse",
                    "enrich": "crate::enrich::enrich_read_page",
                    "error": "crate::error::ReadError",
                    "context_field": #context_setting
                }
            })]
            struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
        };
        let generated = crate::generate_pg_table::generate_pg_table(macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input));
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        assert!(output.contains(&quote::quote! { crate::response::ReadResponse }.to_string()));
        assert!(output.contains(&quote::quote! { crate::enrich::enrich_read_page }.to_string()));
        assert!(output.contains(&quote::quote! { crate::error::ReadError }.to_string()));
        assert_eq!(output.contains(&quote::quote! { #[serde(default)] context: Option<crate::context::ReadContext> }.to_string()), extensions_enabled);
        assert_eq!(output.contains(&quote::quote! { parameters.payload.context.as_ref() }.to_string()), extensions_enabled);
        assert_eq!(output.contains(&quote::quote! { #[serde(default)] search: Option<pg_crud_common::read_search::ReadSearch> }.to_string()), extensions_enabled);
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| !file.items.is_empty())
    }));
}

#[test]
fn test_table_custom_error_variants_accept_annotated_sources_and_reserved_location_fields() {
    let input = quote::quote! {
        #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
            "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
        })]
        #[proc_macro_generate_pg_table::common_error_variants(
            enum CommonErrorVariants {
                CustomFailure {
                    #[error_field_to_err_string] source: std::io::Error,
                    location: location_lib::location::Location
                }
            }
        )]
        struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
    };
    let generated = crate::generate_pg_table::generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    );
    let output = generated.to_string();
    assert!(!output.contains(stringify!(compile_error)));
    assert!(output.contains(stringify!(CustomFailure)));
    assert!(output.contains(&quote::quote! { source: std::io::Error }.to_string()));
    assert!(
        output.contains(&quote::quote! { location: location_lib::location::Location }.to_string())
    );
    assert!(
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| {
            [
                stringify!(TableCreateManyError),
                stringify!(TableReadError),
                stringify!(TableUpdateError),
                stringify!(TableDeleteManyError),
            ]
            .into_iter()
            .all(|error_name| {
                file.items.iter().any(|item| {
                    let syn::Item::Enum(error_enum) = item else {
                        return false;
                    };
                    if error_enum.ident != error_name {
                        return false;
                    }
                    error_enum.variants.iter().any(|variant| {
                        if variant.ident != stringify!(CustomFailure) {
                            return false;
                        }
                        let syn::Fields::Named(fields) = &variant.fields else {
                            return false;
                        };
                        let mut fields_iter = fields.named.iter();
                        let Some(source_field) = fields_iter.next() else {
                            return false;
                        };
                        let Some(location_field) = fields_iter.next() else {
                            return false;
                        };
                        fields_iter.next().is_none()
                            && source_field
                                .ident
                                .as_ref()
                                .is_some_and(|identifier| identifier == stringify!(source))
                            && location_field
                                .ident
                                .as_ref()
                                .is_some_and(|identifier| identifier == stringify!(location))
                    })
                })
            })
        })
    );
}
