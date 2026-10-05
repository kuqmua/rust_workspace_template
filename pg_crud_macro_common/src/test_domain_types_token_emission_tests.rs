#[test]
fn test_query_part_write_error_emitter_preserves_import_and_location() {
    assert!([crate::import::Import::Crate, crate::import::Import::PgCrudCommon]
        .into_iter().all(|import| {
            let generated = crate::generate_query_part_error_write_into_buffer_token_stream::generate_query_part_error_write_into_buffer_token_stream(import);
            let expected: syn::Expr = syn::parse_quote! {
                #import::query_part_error::QueryPartError::WriteIntoBuffer {
                    location: proc_macro_location_bang::location!()
                }
            };
            syn::parse2::<syn::Expr>(proc_macro2::TokenStream::from(generated))
                .is_ok_and(|expression| expression == expected)
        }));
}

#[test]
fn test_postgresql_derive_presets_preserve_visibility_traits_and_declarations() {
    assert!([
        (
            crate::common_derive_token_stream_builder::common_derive_token_stream_builder(),
            quote::quote! { Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize },
        ),
        (
            crate::error_enum_derive_token_stream_builder::error_enum_derive_token_stream_builder(),
            quote::quote! { Debug, thiserror::Error, proc_macro_location_derive_location::Location },
        ),
        (
            crate::serde_error_enum_derive_token_stream_builder::serde_error_enum_derive_token_stream_builder(),
            quote::quote! { Debug, serde::Serialize, serde::Deserialize, thiserror::Error, proc_macro_location_derive_location::Location },
        ),
    ].into_iter().all(|(builder, traits)| {
        let generated = builder.build_enum(
            &quote::quote! { #[repr(C)] },
            &quote::quote! { TestPreset },
            &quote::quote! { <T> },
            &quote::quote! { { First(T), Second } },
        );
        let expected: syn::ItemEnum = syn::parse_quote! {
            #[derive(#traits)]
            #[repr(C)]
            pub enum TestPreset<T> { First(T), Second }
        };
        syn::parse2::<syn::ItemEnum>(generated).is_ok_and(|item_enum| item_enum == expected)
    }));
}

#[test]
fn test_unique_vector_match_emitter_preserves_success_empty_and_invariant_failure_branches() {
    assert!([crate::import::Import::Crate, crate::import::Import::PgCrudCommon]
        .into_iter().all(|import| {
            let source_expression = quote::quote! { validated_values };
            let success_binding = quote::quote! { unique_values };
            let generated = crate::generate_match_not_empty_unique_vec_try_new_some_or_none_token_stream::generate_match_not_empty_unique_vec_try_new_some_or_none_token_stream(
                &import, &source_expression, &success_binding,
                crate::panic_uuid_ref::PanicUuidRef::from(constants_str::TEST_ACCESS_SESSION_ID),
            );
            let Ok(expression) = syn::parse2::<syn::ExprMatch>(proc_macro2::TokenStream::from(generated)) else { return false; };
            let Some(success) = expression.arms.first() else { return false; };
            let Some(failure) = expression.arms.last() else { return false; };
            let syn::Expr::Match(error_match) = failure.body.as_ref() else { return false; };
            let Some(empty) = error_match.arms.first() else { return false; };
            let Some(non_unique) = error_match.arms.last() else { return false; };
            let syn::Expr::Block(invariant) = non_unique.body.as_ref() else { return false; };
            let panic_macro = match invariant.block.stmts.first() {
                Some(syn::Stmt::Macro(statement)) => &statement.mac,
                Some(syn::Stmt::Expr(syn::Expr::Macro(expression_macro), _)) => &expression_macro.mac,
                _ => return false,
            };
            let Ok(panic_arguments) = syn::parse::Parser::parse2(
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
                panic_macro.tokens.clone(),
            ) else { return false; };
            let Some(syn::Expr::Lit(diagnostic)) = panic_arguments.first() else { return false; };
            let syn::Lit::Str(message) = &diagnostic.lit else { return false; };
            let Some(syn::Expr::Lit(context)) = panic_arguments.last() else { return false; };
            let syn::Lit::Str(context_literal) = &context.lit else { return false; };
            let source = &expression.expr;
            let success_pattern = &success.pat;
            let success_body = &success.body;
            let failure_pattern = &failure.pat;
            let error_expression = &error_match.expr;
            let empty_pattern = &empty.pat;
            let empty_body = &empty.body;
            let non_unique_pattern = &non_unique.pat;
            expression.arms.len() == 2usize
                && quote::quote! { #source }.to_string() == source_expression.to_string()
                && quote::quote! { #success_pattern }.to_string() == quote::quote! { Ok(unique_values) }.to_string()
                && quote::quote! { #success_body }.to_string() == quote::quote! { Some(unique_values) }.to_string()
                && quote::quote! { #failure_pattern }.to_string() == quote::quote! { Err(error) }.to_string()
                && quote::quote! { #error_expression }.to_string() == stringify!(error)
                && error_match.arms.len() == 2usize
                && quote::quote! { #empty_pattern }.to_string() == quote::quote! { #import::not_empty_unique_vec::NotEmptyUniqueVecTryNewError::IsEmpty {..} }.to_string()
                && quote::quote! { #empty_body }.to_string() == stringify!(None)
                && quote::quote! { #non_unique_pattern }.to_string() == quote::quote! { #import::not_empty_unique_vec::NotEmptyUniqueVecTryNewError::NotUnique {..} }.to_string()
                && invariant.block.stmts.len() == 1usize
                && panic_macro.path.is_ident(stringify!(panic))
                && panic_arguments.len() == 2usize
                && context_literal.value() == constants_str::TEST_ACCESS_SESSION_ID
                && message.value().split_once(' ').is_some_and(|(prefix, _)| {
                    prefix.len() == 8usize && prefix.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                })
        }));
}

#[test]
fn test_result_emitters_preserve_expression_binding_assignment_and_early_error_return() {
    assert!([
        (quote::quote! { prepare() }, quote::quote! { query }, quote::quote! { prepared }),
        (quote::quote! { resource.prepare(argument)? }, quote::quote! { state.query }, quote::quote! { next_query }),
    ].into_iter().all(|(expression, assignment, binding)| {
        let assigned = crate::generate_match_ok_assign_or_return_err_token_stream::generate_match_ok_assign_or_return_err_token_stream(
            &expression, &assignment, &binding,
        );
        let returned = crate::generate_match_ok_or_return_err_token_stream::generate_match_ok_or_return_err_token_stream(
            &expression, &binding,
        );
        let expected_assignment: syn::Expr = syn::parse_quote! {
            match #expression {
                Ok(#binding) => { #assignment = #binding; }
                Err(error) => { return Err(error); }
            }
        };
        let expected_return: syn::Expr = syn::parse_quote! {
            match #expression {
                Ok(#binding) => #binding,
                Err(error) => { return Err(error); }
            }
        };
        syn::parse2::<syn::Expr>(proc_macro2::TokenStream::from(assigned))
            .is_ok_and(|assigned_expression| assigned_expression == expected_assignment)
            && syn::parse2::<syn::Expr>(proc_macro2::TokenStream::from(returned))
                .is_ok_and(|returned_expression| returned_expression == expected_return)
    }));
}

#[test]
fn test_optional_query_emitter_preserves_borrowed_condition_and_final_query_result() {
    let expression = quote::quote! { filter.prepare(query) };
    let binding = quote::quote! { filter };
    let prepared_binding = quote::quote! { prepared_query };
    let generated = crate::generate_if_let_some_match_ok_assign_query_or_return_err_token_stream::generate_if_let_some_match_ok_assign_query_or_return_err_token_stream(
        &expression, &binding, &prepared_binding,
    );
    let expected: syn::Block = syn::parse_quote! {
        {
            if let Some(filter) = &v.0 {
                match filter.prepare(query) {
                    Ok(prepared_query) => { query = prepared_query; }
                    Err(error) => { return Err(error); }
                }
            }
            Ok(query)
        }
    };
    assert!(
        syn::parse2::<syn::Block>(quote::quote! { { #generated } })
            .is_ok_and(|block| block == expected)
    );
}

#[test]
fn test_dimension_indices_preserve_zero_based_order() {
    assert!(
        [
            (
                crate::dimension::Dimension::One,
                crate::dimension_index_number::DimensionIndexNumber::Zero
            ),
            (
                crate::dimension::Dimension::Two,
                crate::dimension_index_number::DimensionIndexNumber::One
            ),
            (
                crate::dimension::Dimension::Three,
                crate::dimension_index_number::DimensionIndexNumber::Two
            ),
            (
                crate::dimension::Dimension::Four,
                crate::dimension_index_number::DimensionIndexNumber::Three
            ),
        ]
        .into_iter()
        .all(|(dimension, expected_index)| {
            matches!(
                (
                    crate::dimension_index_number::DimensionIndexNumber::from(&dimension),
                    expected_index
                ),
                (
                    crate::dimension_index_number::DimensionIndexNumber::Zero,
                    crate::dimension_index_number::DimensionIndexNumber::Zero
                ) | (
                    crate::dimension_index_number::DimensionIndexNumber::One,
                    crate::dimension_index_number::DimensionIndexNumber::One
                ) | (
                    crate::dimension_index_number::DimensionIndexNumber::Two,
                    crate::dimension_index_number::DimensionIndexNumber::Two
                ) | (
                    crate::dimension_index_number::DimensionIndexNumber::Three,
                    crate::dimension_index_number::DimensionIndexNumber::Three
                )
            )
        })
    );
}

#[test]
fn test_dimension_pagination_identifier_preserves_numeric_boundaries() {
    assert!([
        (0usize, quote::quote! { dimension0_pagination }),
        (1usize, quote::quote! { dimension1_pagination }),
        (4usize, quote::quote! { dimension4_pagination }),
        (10usize, quote::quote! { dimension10_pagination }),
    ].into_iter().all(|(number, expected_tokens)| {
        let generated = crate::generate_dimension_number_pagination_token_stream::generate_dimension_number_pagination_token_stream(
            crate::dimension_number::DimensionNumber::from(number),
        );
        syn::parse2::<syn::Ident>(proc_macro2::TokenStream::from(generated))
            .is_ok_and(|identifier| identifier == expected_tokens.to_string())
    }));
}

#[test]
fn test_filter_variants_preserve_generic_tokens_names_and_where_prefixes() {
    let identifier = quote::quote! { domain::Value<Inner> };
    assert!([
        (crate::pg_type_filter::PgTypeFilter::Eq{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { Eq }, quote::quote! { PgTypeWhereEq }),
        (crate::pg_type_filter::PgTypeFilter::GreaterThan{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { GreaterThan }, quote::quote! { PgTypeWhereGreaterThan }),
        (crate::pg_type_filter::PgTypeFilter::Between{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { Between }, quote::quote! { PgTypeWhereBetween }),
        (crate::pg_type_filter::PgTypeFilter::In{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { In }, quote::quote! { PgTypeWhereIn }),
        (crate::pg_type_filter::PgTypeFilter::Before{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { Before }, quote::quote! { PgTypeWhereBefore }),
        (crate::pg_type_filter::PgTypeFilter::FindRangesWithinGivenRange{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { FindRangesWithinGivenRange }, quote::quote! { PgTypeWhereFindRangesWithinGivenRange }),
        (crate::pg_type_filter::PgTypeFilter::FindRangesThatFullyContainTheGivenRange{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { FindRangesThatFullyContainTheGivenRange }, quote::quote! { PgTypeWhereFindRangesThatFullyContainTheGivenRange }),
        (crate::pg_type_filter::PgTypeFilter::StrictlyToLeftOfRange{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { StrictlyToLeftOfRange }, quote::quote! { PgTypeWhereStrictlyToLeftOfRange }),
        (crate::pg_type_filter::PgTypeFilter::StrictlyToRightOfRange{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { StrictlyToRightOfRange }, quote::quote! { PgTypeWhereStrictlyToRightOfRange }),
        (crate::pg_type_filter::PgTypeFilter::IncludedLowerBound{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { IncludedLowerBound }, quote::quote! { PgTypeWhereIncludedLowerBound }),
        (crate::pg_type_filter::PgTypeFilter::ExcludedUpperBound{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { ExcludedUpperBound }, quote::quote! { PgTypeWhereExcludedUpperBound }),
        (crate::pg_type_filter::PgTypeFilter::GreaterThanIncludedLowerBound{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { GreaterThanIncludedLowerBound }, quote::quote! { PgTypeWhereGreaterThanIncludedLowerBound }),
        (crate::pg_type_filter::PgTypeFilter::GreaterThanExcludedUpperBound{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { GreaterThanExcludedUpperBound }, quote::quote! { PgTypeWhereGreaterThanExcludedUpperBound }),
        (crate::pg_type_filter::PgTypeFilter::OverlapWithRange{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { OverlapWithRange }, quote::quote! { PgTypeWhereOverlapWithRange }),
        (crate::pg_type_filter::PgTypeFilter::AdjacentWithRange{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { AdjacentWithRange }, quote::quote! { PgTypeWhereAdjacentWithRange }),
        (crate::pg_type_filter::PgTypeFilter::RangeLen{ identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(identifier.clone()) }, true, quote::quote! { RangeLen }, quote::quote! { PgTypeWhereRangeLen }),
        (crate::pg_type_filter::PgTypeFilter::Regex, false, quote::quote! { Regex }, quote::quote! { PgTypeWhereRegex }),
        (crate::pg_type_filter::PgTypeFilter::CurrentDate, false, quote::quote! { CurrentDate }, quote::quote! { PgTypeWhereCurrentDate }),
        (crate::pg_type_filter::PgTypeFilter::GreaterThanCurrentDate, false, quote::quote! { GreaterThanCurrentDate }, quote::quote! { PgTypeWhereGreaterThanCurrentDate }),
        (crate::pg_type_filter::PgTypeFilter::CurrentTimestamp, false, quote::quote! { CurrentTimestamp }, quote::quote! { PgTypeWhereCurrentTimestamp }),
        (crate::pg_type_filter::PgTypeFilter::GreaterThanCurrentTimestamp, false, quote::quote! { GreaterThanCurrentTimestamp }, quote::quote! { PgTypeWhereGreaterThanCurrentTimestamp }),
        (crate::pg_type_filter::PgTypeFilter::CurrentTime, false, quote::quote! { CurrentTime }, quote::quote! { PgTypeWhereCurrentTime }),
        (crate::pg_type_filter::PgTypeFilter::GreaterThanCurrentTime, false, quote::quote! { GreaterThanCurrentTime }, quote::quote! { PgTypeWhereGreaterThanCurrentTime }),
        (crate::pg_type_filter::PgTypeFilter::EqToEncodedStringRepresentation, false, quote::quote! { EqToEncodedStringRepresentation }, quote::quote! { PgTypeWhereEqToEncodedStringRepresentation }),
    ].into_iter().all(|(filter, has_generic, expected_name, expected_prefix)| {
        let generic = crate::pg_filter::PgFilter::maybe_generic(&filter);
        let name = crate::pg_filter::PgFilter::ucc(&filter);
        let prefix = crate::pg_filter::PgFilter::prefix_where_self_upper_camel_case(&filter);
        generic.as_ref().map_or_else(|| !has_generic, |generic_tokens| {
            has_generic && generic_tokens.to_string() == identifier.to_string()
        }) && name.to_string() == expected_name.to_string()
            && quote::quote! { #name }.to_string() == expected_name.to_string()
            && prefix.to_string() == expected_prefix.to_string()
    }));
}

#[test]
fn test_equality_operator_paths_preserve_variants_and_import_roots() {
    assert!(
        [
            crate::import::Import::Crate,
            crate::import::Import::PgCrudCommon
        ]
        .into_iter()
        .all(|import| {
            [
                (
                    crate::eq_operator_variant::EqOperatorVariant::Eq,
                    quote::quote! { #import::eq_operator::EqOperator::Eq },
                ),
                (
                    crate::eq_operator_variant::EqOperatorVariant::IsNull,
                    quote::quote! { #import::eq_operator::EqOperator::IsNull },
                ),
            ]
            .into_iter()
            .all(|(variant, expected_tokens)| {
                let generated = variant.to_tokens_path(&import);
                syn::parse2::<syn::Path>(proc_macro2::TokenStream::from(generated)).is_ok_and(
                    |path| quote::quote! { #path }.to_string() == expected_tokens.to_string(),
                )
            })
        })
    );
}

#[test]
fn test_read_and_update_names_preserve_display_and_tokens() {
    assert!(
        [
            (
                crate::read_or_update::ReadOrUpdate::Read,
                quote::quote! { Read }
            ),
            (
                crate::read_or_update::ReadOrUpdate::Update,
                quote::quote! { Update }
            ),
        ]
        .into_iter()
        .all(|(operation, expected_tokens)| {
            let name = operation.ucc();
            name.to_string() == expected_tokens.to_string()
                && quote::quote! { #name }.to_string() == expected_tokens.to_string()
        })
    );
}

#[test]
fn test_scope_wrapping_preserves_tokens_and_introduces_one_parenthesis_group() {
    assert!(
        [
            quote::quote! {},
            quote::quote! { value.method() },
            quote::quote! { first, second },
            quote::quote! { (first, second), [third], { fourth } },
        ]
        .into_iter()
        .all(|tokens| {
            let unchanged =
                crate::maybe_wrap_into_braces_token_stream::maybe_wrap_into_braces_token_stream(
                    &tokens,
                    crate::wrap_into_braces::WrapIntoBraces::from(false),
                );
            let wrapped =
                crate::maybe_wrap_into_braces_token_stream::maybe_wrap_into_braces_token_stream(
                    &tokens,
                    crate::wrap_into_braces::WrapIntoBraces::from(true),
                );
            let direct =
                crate::wrap_into_scopes_token_stream::wrap_into_scopes_token_stream(&tokens);
            let mut token_trees = proc_macro2::TokenStream::from(wrapped.clone()).into_iter();
            let Some(proc_macro2::TokenTree::Group(group)) = token_trees.next() else {
                return false;
            };
            unchanged.to_string() == tokens.to_string()
                && wrapped.to_string() == direct.to_string()
                && token_trees.next().is_none()
                && group.delimiter() == proc_macro2::Delimiter::Parenthesis
                && group.stream().to_string() == tokens.to_string()
        })
    );
}

#[test]
fn test_sqlx_json_type_emitter_preserves_nested_types() {
    assert!([
        quote::quote! { domain::Value },
        quote::quote! { Option<Vec<domain::Value>> },
        quote::quote! { (domain::First, domain::Second) },
    ].into_iter().all(|type_tokens| {
        let generated = crate::generate_sqlx_types_json_type_declaration_token_stream::generate_sqlx_types_json_type_declaration_token_stream(&type_tokens);
        let expected: syn::Type = syn::parse_quote! { sqlx::types::Json<#type_tokens> };
        syn::parse2::<syn::Type>(proc_macro2::TokenStream::from(generated))
            .is_ok_and(|generated_type| generated_type == expected)
    }));
}

#[test]
fn test_query_error_emitters_preserve_type_path_checked_add_variant_and_location() {
    let generated_type = crate::pg_crud_common_query_part_error_token_stream::pg_crud_common_query_part_error_token_stream();
    let generated_error = crate::pg_crud_common_query_part_error_checked_add_initialization_token_stream::pg_crud_common_query_part_error_checked_add_initialization_token_stream();
    let expected_type: syn::Type =
        syn::parse_quote! { pg_crud_common::query_part_error::QueryPartError };
    let expected_error: syn::Expr = syn::parse_quote! {
        pg_crud_common::query_part_error::QueryPartError::CheckedAdd {
            location: proc_macro_location_bang::location!()
        }
    };
    assert!(
        syn::parse2::<syn::Type>(proc_macro2::TokenStream::from(generated_type))
            .is_ok_and(|type_expression| type_expression == expected_type)
    );
    assert!(
        syn::parse2::<syn::Expr>(proc_macro2::TokenStream::from(generated_error))
            .is_ok_and(|error_expression| error_expression == expected_error)
    );
}

#[test]
fn test_deserializer_validation_match_preserves_constructor_arguments_and_debug_error_source() {
    let identifier = quote::quote! { domain::Validated };
    let initialization = quote::quote! { raw.first, raw.second };
    let generated = crate::generate_match_try_new_in_deserialize_token_stream::generate_match_try_new_in_deserialize_token_stream(
        &identifier, &initialization,
    );
    let diagnostic_format = syn::LitStr::new(
        &format!("{{{}:?}}", stringify!(error)),
        proc_macro2::Span::call_site(),
    );
    let expected: syn::Expr = syn::parse_quote! {
        match domain::Validated::try_new(raw.first, raw.second) {
            Ok(v) => Ok(v),
            Err(error) => Err(serde::de::Error::custom(format!(#diagnostic_format)))
        }
    };
    assert!(
        syn::parse2::<syn::Expr>(proc_macro2::TokenStream::from(generated))
            .is_ok_and(|expression| expression == expected)
    );
}

#[test]
fn test_default_variant_adapters_preserve_trait_paths_methods_and_supplied_bodies() {
    let identifier = quote::quote! { TestVariants };
    let values = quote::quote! { [Self::First, Self::Second] };
    let body = quote::quote! { Self::First };
    assert!([
        (
            crate::generate_impl_pg_crud_all_variants_default_some_one_element_token_stream::generate_impl_pg_crud_all_variants_default_some_one_element_token_stream(&identifier, &values),
            quote::quote! {
                impl pg_crud_common::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement for TestVariants {
                    fn all_variants_default_some_one_element() -> pg_crud_common::all_enum_variants::AllEnumVariants<Self> { ([Self::First, Self::Second]).into() }
                }
            },
        ),
        (
            crate::generate_impl_pg_crud_all_variants_default_some_one_element_max_page_size_token_stream::generate_impl_pg_crud_all_variants_default_some_one_element_max_page_size_token_stream(&identifier, &values),
            quote::quote! {
                impl pg_crud_common::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize for TestVariants {
                    fn all_variants_default_some_one_element_max_page_size() -> pg_crud_common::all_enum_variants::AllEnumVariants<Self> { ([Self::First, Self::Second]).into() }
                }
            },
        ),
        (
            crate::generate_impl_all_variants_default_some_one_element_max_page_size_token_stream::generate_impl_all_variants_default_some_one_element_max_page_size_token_stream(&crate::import::Import::Crate, &identifier, &values),
            quote::quote! {
                impl crate::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize for TestVariants {
                    fn all_variants_default_some_one_element_max_page_size() -> crate::AllEnumVariants<Self> { ([Self::First, Self::Second]).into() }
                }
            },
        ),
        (
            crate::generate_impl_pg_crud_default_some_one_element_max_page_size_token_stream::generate_impl_pg_crud_default_some_one_element_max_page_size_token_stream(&identifier, &quote::quote! { <'static> }, &body),
            quote::quote! {
                impl pg_crud_common::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize for TestVariants<'static> {
                    fn default_some_one_element_max_page_size() -> Self { Self::First }
                }
            },
        ),
    ].into_iter().all(|(generated, expected_tokens)| {
        let expected = syn::parse2::<syn::ItemImpl>(expected_tokens);
        syn::parse2::<syn::ItemImpl>(proc_macro2::TokenStream::from(generated))
            .is_ok_and(|item_impl| expected.is_ok_and(|expected_impl| item_impl == expected_impl))
    }));
}

#[test]
fn test_debug_display_adapter_preserves_both_formats_and_validated_error_text_conversion() {
    let generated = crate::generate_impl_display_and_to_err_string_debug_token_stream::generate_impl_display_and_to_err_string_debug_token_stream(&quote::quote! { TestDiagnostic });
    let display_format = syn::LitStr::new(
        &format!("{{{}:?}}", stringify!(self)),
        proc_macro2::Span::call_site(),
    );
    let error_format = syn::LitStr::new(
        &format!("{{{}:#?}}", stringify!(self)),
        proc_macro2::Span::call_site(),
    );
    let expected: syn::File = syn::parse_quote! {
        impl std::fmt::Display for TestDiagnostic {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #display_format)
            }
        }
        impl to_err_string::to_err_string::ToErrString for TestDiagnostic {
            fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
                to_err_string::error_text::ErrorText::try_from(format!(#error_format)).unwrap_or_else(to_err_string::error_text::ErrorText::from)
            }
        }
    };
    assert!(
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated))
            .is_ok_and(|file| file == expected)
    );
}

#[test]
fn test_import_trait_selections_preserve_root_and_snake_case_text() {
    assert!([
        (crate::import::Import::Crate, constants_str::CRATE,
            quote::quote! { crate::AllEnumVariantsArrayDefaultSomeOneElement },
            quote::quote! { crate::DefaultSomeOneElement },
            quote::quote! { crate::DefaultSomeOneElementMaxPageSize }),
        (crate::import::Import::PgCrudCommon, constants_str::PG_CRUD_COMMON_DOMAIN_TYPES,
            quote::quote! { pg_crud_common::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement },
            quote::quote! { pg_crud_common::default_some_one_element::DefaultSomeOneElement },
            quote::quote! { pg_crud_common::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize }),
    ].into_iter().all(|(import, expected_text, all_variants_path, default_path, maximum_path)| {
        let all_variants = import.all_variants_default_some_one_element();
        let default = import.default_some_one_element();
        let maximum = import.default_some_one_element_max_page_size();
        import.sc_str().to_string() == expected_text
            && quote::quote! { #all_variants }.to_string() == all_variants_path.to_string()
            && quote::quote! { #default }.to_string() == default_path.to_string()
            && quote::quote! { #maximum }.to_string() == maximum_path.to_string()
    }));
}

#[test]
fn test_where_emitter_schema_selection_preserves_other_items() {
    let variants = [
        crate::pg_type_filter::PgTypeFilter::Eq {
            identifier: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(quote::quote! { domain::Value }),
        },
        crate::pg_type_filter::PgTypeFilter::Regex,
    ];
    assert!([
        crate::emission_types::ShouldDSchemarsJsonSchema::False,
        crate::emission_types::ShouldDSchemarsJsonSchema::True,
    ].into_iter().all(|schemars| {
        [crate::emission_types::IsQueryBindMut::False, crate::emission_types::IsQueryBindMut::True]
            .into_iter().all(|query_mutability| {
                let emit = |schema_selection: crate::emission_types::ShouldDeriveUtoipaToSchema| {
                    syn::parse2::<syn::File>(proc_macro2::TokenStream::from(
                        crate::generate_pg_type_where_token_stream::generate_pg_type_where_token_stream(
                            &quote::quote! { #[cfg(any())] }, &variants, &quote::quote! { TestFilter },
                            &schema_selection, &schemars, &query_mutability,
                        ),
                    ))
                };
                let Ok(without_schema) = emit(crate::emission_types::ShouldDeriveUtoipaToSchema::False) else { return false; };
                let Ok(mut with_schema) = emit(crate::emission_types::ShouldDeriveUtoipaToSchema::True) else { return false; };
                let is_schema_impl = |item: &syn::Item| {
                    matches!(item, syn::Item::Impl(item_impl) if item_impl.trait_.as_ref().is_some_and(|(path, _)|
                        path.segments.first().is_some_and(|segment| segment.ident == stringify!(utoipa))
                        && path.segments.last().is_some_and(|segment|
                            segment.ident == stringify!(PartialSchema) || segment.ident == stringify!(ToSchema))))
                };
                let with_count = with_schema.items.iter().filter(|item| is_schema_impl(item)).count();
                let without_count = without_schema.items.iter().filter(|item| is_schema_impl(item)).count();
                with_schema.items.retain(|item| !is_schema_impl(item));
                with_count == 2usize && without_count == 0usize && with_schema == without_schema
            })
    }));
}
