#[must_use]
pub fn emit_generate_where_filters(
    validated: crate::validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig,
) -> crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream
{
    #[derive(Clone, optimal_memory_layout::OptimalMemoryLayout)]
    enum Generic {
        False,
        True {
            maybe_extra_traits_token_stream: Option<proc_macro2::TokenStream>,
        },
    }
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, optimal_memory_layout::OptimalMemoryLayout)]
    enum PgTypePtrn {
        Standard,
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum PgTypeKind {
        Standard,
    }
    impl PgTypeKind {
        const fn format_argument(&self) -> &'static str {
            match &self {
                Self::Standard => constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            }
        }
    }
    panic_location::panic_location();
    let generate_where_filters_config =
        crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig::from(
            validated,
        );
    let (pg_types_write_into_file, whole_write_into_file) =
        generate_where_filters_config.into_parts();
    let column_snake_case = naming::domain_types::ColumnSnakeCase;
    let error_snake_case = naming::domain_types::ErrorSnakeCase;
    let increment_snake_case = naming::domain_types::IncrementSnakeCase;
    let query_snake_case = naming::domain_types::QuerySnakeCase;
    let self_snake_case = naming::domain_types::SelfSnakeCase;
    let values_snake_case = naming::domain_types::ValuesSnakeCase;
    let pg_crud_common_default_some_one_element = token_patterns::PgCrudCommonDefaultSomeOneElement;
    let pg_crud_common_default_some_one_element_call =
        token_patterns::PgCrudCommonDefaultSomeOneElementCall;
    let import = pg_crud_macro_common::import::Import::PgCrudCommon;
    let t_token_stream = quote::quote! {T};
    let t_ann_generic_token_stream = quote::quote! {<#t_token_stream>};
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    let pub_v_t_token_stream = quote::quote! {#[schema(inline)] #values_snake_case: T};
    let v_default_some_one_element_token_stream = quote::quote! {
        #values_snake_case: #pg_crud_common_default_some_one_element_call
    };
    let generate_struct_token_stream =
        |filter_initialization_with_try_new_result_is_ok,
         generic: &Generic,
         identifier: &dyn quote::ToTokens,
         struct_extra_fields_token_stream: &dyn quote::ToTokens| {
            macro_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize_if(if filter_initialization_with_try_new_result_is_ok {
                    macro_helpers::derive_token_stream_builder::DSerdeDeserialize::False
                } else {
                    macro_helpers::derive_token_stream_builder::DSerdeDeserialize::True
                })
                .d_schemars_json_schema()
                .d_utoipa_to_schema()
                .build_struct(
                    &quote::quote! {#[derive(generate_accessor::Getters, generate_constructor::New)]},
                    &identifier,
                    &match &generic {
                        Generic::False => proc_macro2::TokenStream::new(),
                        Generic::True {
                            maybe_extra_traits_token_stream,
                        } => maybe_extra_traits_token_stream.as_ref().map_or_else(
                            || quote::quote! {<#t_token_stream>},
                            |v| quote::quote! {<#t_token_stream: #v>},
                        ),
                    },
                    &quote::quote! {{
                        operator: #import::operator::Operator,
                        #struct_extra_fields_token_stream
                    }},
                )
        };
    let generate_impl_default_some_one_element_token_stream =
        |generic: &Generic, identifier: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            pg_crud_macro_common::generate_impl_default_some_one_element_token_stream::generate_impl_default_some_one_element_token_stream(
                &match &generic {
                    Generic::False => proc_macro2::TokenStream::new(),
                    Generic::True {
                        maybe_extra_traits_token_stream,
                    } => maybe_extra_traits_token_stream.as_ref().map_or_else(
                        || quote::quote! {<T: #pg_crud_common_default_some_one_element>},
                        |v| quote::quote! {<T: #v + #pg_crud_common_default_some_one_element>},
                    ),
                },
                &pg_crud_macro_common::import::Import::PgCrudCommon,
                &identifier,
                match &generic {
                    Generic::False => &proc_macro2_token_stream_new,
                    Generic::True { .. } => &t_ann_generic_token_stream,
                },
                &quote::quote! {
                    Self {
                        operator: #pg_crud_common_default_some_one_element_call,
                        #ts
                    }
                },
            )
        };
    let generate_impl_pg_type_where_filter_token_stream =
        |generic: &Generic,
         identifier: &dyn quote::ToTokens,
         increment_parameter_undrscr: &pg_crud_macro_common::emission_types::IncrementParameterUndrscr,
         add_operator_undrscr: &pg_crud_macro_common::emission_types::AddOperatorUndrscr,
         query_part_token_stream: &dyn quote::ToTokens,
         is_query_bind_mut: &pg_crud_macro_common::emission_types::IsQueryBindMut,
         query_bind_token_stream: &dyn quote::ToTokens| {
            pg_crud_macro_common::impl_pg_type_where_filter_for_identifier_token_stream::impl_pg_type_where_filter_for_identifier_token_stream(
                &{
                    let maybe_t_extra_traits_for_pg_type_where_filter_token_stream: &dyn quote::ToTokens =
                        match &generic {
                            Generic::False => &proc_macro2_token_stream_new,
                            Generic::True { maybe_extra_traits_token_stream } => {
                                let send_and_lt_token_stream = quote::quote! {Send + 'lt};
                                let ts = maybe_extra_traits_token_stream.as_ref().map_or_else(
                                    || quote::quote! {Send + 'lt},
                                    |v| quote::quote! {#v + #send_and_lt_token_stream},
                                );
                                &quote::quote! {, T: #ts}
                            }
                        };
                    quote::quote! {<'lt #maybe_t_extra_traits_for_pg_type_where_filter_token_stream>}
                },
                &identifier,
                &match &generic {
                    Generic::False => &proc_macro2_token_stream_new,
                    Generic::True { .. } => &t_ann_generic_token_stream,
                },
                increment_parameter_undrscr,
                &pg_crud_macro_common::emission_types::ColumnParameterUndrscr::False,
                add_operator_undrscr,
                &query_part_token_stream,
                is_query_bind_mut,
                &query_bind_token_stream,
                &pg_crud_macro_common::import::Import::PgCrudCommon,
            )
        };
    let add_regex_case_and_v_declaration_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            regex_case: crate::regex_case::RegexCase,
            #values_snake_case: crate::regex_regex::RegexRegex
        }
    };
    let add_regex_case_and_v_default_initialization_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            regex_case: #pg_crud_common_default_some_one_element_call,
            #v_default_some_one_element_token_stream
        }
    };
    let generate_match_increment_checked_add_one_initialization_token_stream =
        |ts: &dyn quote::ToTokens| {
            let match_token_stream =
                pg_crud_macro_common::generate_match_ok_or_return_err_token_stream::generate_match_ok_or_return_err_token_stream(
                    &quote::quote! {#import::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(#increment_snake_case)},
                    &quote::quote! {v_25d59e01},
                );
            quote::quote! {
                let #ts = #match_token_stream;
            }
        };
    let v_match_increment_checked_add_one_initialization_token_stream =
        generate_match_increment_checked_add_one_initialization_token_stream(&values_snake_case);
    let self_operator_to_query_part_token_stream =
        quote::quote! {&#self_snake_case.operator.to_query_part(add_operator),};
    let generate_regex_query_part_format_token_stream =
        |v: &dyn std::fmt::Display,
         maybe_dimensions_ies_initialization_token_stream: &dyn quote::ToTokens,
         maybe_extra_parameters_token_stream: &dyn quote::ToTokens| {
            let format_token_stream = generate_quotes::dq_token_stream::dq_token_stream(&v);
            quote::quote! {
                #maybe_dimensions_ies_initialization_token_stream
                #v_match_increment_checked_add_one_initialization_token_stream
                let mut query_part_28bc96ee = String::with_capacity(32);
                if std::fmt::Write::write_fmt(
                    &mut query_part_28bc96ee,
                    format_args!(
                        #format_token_stream,
                        #self_operator_to_query_part_token_stream
                        #column_snake_case,
                        #maybe_extra_parameters_token_stream
                        #self_snake_case.regex_case.postgreql_syntax(),
                        #values_snake_case
                    ),
                )
                .is_err()
                {
                    return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_28bc96ee)?)
            }
        };
    let if_let_err_query_try_bind_self_v_to_string_token_stream = quote::quote! {
        if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(#self_snake_case.#values_snake_case.to_string()) {
            return Err(#import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(#error_snake_case));
        }
        Ok(#query_snake_case)
    };
    let if_let_err_query_try_bind_self_v_token_stream = quote::quote! {
        if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(#self_snake_case.#values_snake_case) {
            return Err(#import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(#error_snake_case));
        }
    };
    let query_bind_one_v_token_stream = quote::quote! {
        #if_let_err_query_try_bind_self_v_token_stream
        Ok(#query_snake_case)
    };
    let generate_generic_true_debug_partial_eq_partial_ord_clone_type_encode = || Generic::True {
        maybe_extra_traits_token_stream: Some(quote::quote! {
            std::fmt::Debug
            + PartialEq
            + PartialOrd
            + Clone
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>
            + utoipa::ToSchema
        }),
    };
    let pub_v_between_t_token_stream =
        quote::quote! {#[schema(inline)] #values_snake_case: crate::between::Between<T>};
    let generate_match_query_bind_token_stream = |field_token_stream: &dyn quote::ToTokens| {
        pg_crud_macro_common::generate_match_ok_assign_or_return_err_token_stream::generate_match_ok_assign_or_return_err_token_stream(
            &quote::quote! {#field_token_stream.query_bind(#query_snake_case)},
            &query_snake_case,
            &quote::quote! {v_f6d31bdd},
        )
    };
    let query_self_v_query_bind_token_stream = {
        let ts = generate_match_query_bind_token_stream(
            &quote::quote! {#self_snake_case.#values_snake_case},
        );
        quote::quote! {
            #ts
            Ok(#query_snake_case)
        }
    };
    let pg_type_ptrn_standard = PgTypePtrn::Standard;
    let generate_identifier_match_field_fn_ok_v_return_err_token_stream =
        |identifier_token_stream: &dyn quote::ToTokens,
         field_token_stream: &dyn quote::ToTokens,
         fn_token_stream: &dyn quote::ToTokens| {
            let match_token_stream =
                pg_crud_macro_common::generate_match_ok_or_return_err_token_stream::generate_match_ok_or_return_err_token_stream(
                    &quote::quote! {self.#field_token_stream.#fn_token_stream(#increment_snake_case, #column_snake_case, add_operator)},
                    &quote::quote! {v_0a22ee9a},
                );
            quote::quote! {
                let #identifier_token_stream = #match_token_stream;
            }
        };
    let v_match_self_v_query_part_initialization_token_stream =
        generate_identifier_match_field_fn_ok_v_return_err_token_stream(
            &values_snake_case,
            &values_snake_case,
            &quote::quote! {query_part},
        );
    let generate_maybe_dimensions_declaration_pub_v_t_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            #pub_v_t_token_stream
        }
    };
    let generate_maybe_dimensions_default_initialization_v_default_token_stream =
        |ts: &dyn quote::ToTokens| {
            quote::quote! {
                #ts
                #v_default_some_one_element_token_stream
            }
        };
    let generate_two_token_stream =
        |maybe_dimensions_query_bind_token_stream: &dyn quote::ToTokens,
         trailing_token_stream: &dyn quote::ToTokens| {
            quote::quote! {
                #maybe_dimensions_query_bind_token_stream
                #trailing_token_stream
            }
        };
    let is_query_bind_mut_true = pg_crud_macro_common::emission_types::IsQueryBindMut::True;
    let is_query_bind_mut_false = pg_crud_macro_common::emission_types::IsQueryBindMut::False;
    let generate_query_part_format_with_v_token_stream =
        |maybe_dimensions_ies_initialization_token_stream: &dyn quote::ToTokens,
         format_token_stream: &dyn quote::ToTokens,
         maybe_extra_parameters_token_stream: &dyn quote::ToTokens| {
            quote::quote! {
                #maybe_dimensions_ies_initialization_token_stream
                #v_match_increment_checked_add_one_initialization_token_stream
                let mut query_part_1c95685d = String::with_capacity(32);
                if std::fmt::Write::write_fmt(
                    &mut query_part_1c95685d,
                    format_args!(
                        #format_token_stream,
                        #self_operator_to_query_part_token_stream
                        #column_snake_case,
                        #maybe_extra_parameters_token_stream
                        #values_snake_case
                    ),
                )
                .is_err()
                {
                    return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_1c95685d)?)
            }
        };
    let generate_pg_type_dimensions_helpers = |pg_type_ptrn: &PgTypePtrn| match pg_type_ptrn {
        PgTypePtrn::Standard => (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
            PgTypeKind::Standard,
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        ),
    };
    let pg_type_token_stream = {
        let generate_filters_token_stream =
            |filter: &pg_crud_macro_common::pg_type_filter::PgTypeFilter| {
                let identifier =
                    naming::parameter::PgTypeWhereSelfUpperCamelCase::from_display(&filter);
                let (
                    generic,
                    struct_extra_fields_token_stream,
                    impl_default_some_one_element_extra_fields_token_stream,
                    increment_parameter_undrscr,
                    query_part_token_stream,
                    is_query_bind_mut,
                    query_bind_token_stream,
                ) = {
                    let generate_sqlx_type_pg_encode_token_stream = || quote::quote! {sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + utoipa::ToSchema};
                    let generate_generic_true_type_encode = || Generic::True {
                        maybe_extra_traits_token_stream: Some(
                            generate_sqlx_type_pg_encode_token_stream(),
                        ),
                    };
                    let generate_pg_type_dimensions_helpers_pg_type =
                        |pg_type_ptrn: &PgTypePtrn| {
                            generate_pg_type_dimensions_helpers(pg_type_ptrn)
                        };
                    let generate_cmp_filter_token_stream =
                        |pg_type_ptrn: &PgTypePtrn,
                         format_value: &dyn Fn(&PgTypeKind) -> String| {
                            let (
                                maybe_dimensions_declaration_token_stream,
                                maybe_dimensions_default_initialization_token_stream,
                                maybe_dimensions_ies_initialization_token_stream,
                                pg_type_kind,
                                maybe_extra_parameters_token_stream,
                                maybe_dimensions_query_bind_token_stream,
                            ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                            (
                            generate_generic_true_type_encode(),
                            generate_maybe_dimensions_declaration_pub_v_t_token_stream(
                                &maybe_dimensions_declaration_token_stream,
                            ),
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                            generate_query_part_format_with_v_token_stream(
                                &maybe_dimensions_ies_initialization_token_stream,
                                &generate_quotes::dq_token_stream::dq_token_stream(&format_value(
                                    &pg_type_kind,
                                )),
                                &maybe_extra_parameters_token_stream,
                            ),
                            is_query_bind_mut_true,
                            generate_two_token_stream(
                                &maybe_dimensions_query_bind_token_stream,
                                &query_bind_one_v_token_stream,
                            ),
                        )
                        };
                    let generate_operator_cmp_filter_token_stream =
                        |pg_type_ptrn: &PgTypePtrn, operator: &dyn std::fmt::Display| {
                            generate_cmp_filter_token_stream(
                                pg_type_ptrn,
                                &|pg_type_kind: &PgTypeKind| {
                                    format!(
                                        "{{}}({{}}{} {operator} ${{}})",
                                        pg_type_kind.format_argument()
                                    )
                                },
                            )
                        };
                    let generate_greater_than_token_stream = |pg_type_ptrn: &PgTypePtrn| {
                        generate_operator_cmp_filter_token_stream(
                            pg_type_ptrn,
                            &constants_str::TEXT_ALT_11,
                        )
                    };
                    let generate_between_token_stream = |pg_type_ptrn: &PgTypePtrn| {
                        let (
                            maybe_dimensions_declaration_token_stream,
                            maybe_dimensions_default_initialization_token_stream,
                            maybe_dimensions_ies_initialization_token_stream,
                            pg_type_kind,
                            maybe_extra_parameters_token_stream,
                            maybe_dimensions_query_bind_token_stream,
                        ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                        (
                            generate_generic_true_debug_partial_eq_partial_ord_clone_type_encode(),
                            quote::quote! {
                                #maybe_dimensions_declaration_token_stream
                                #pub_v_between_t_token_stream
                            },
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                            {
                                let format_token_stream =
                                    generate_quotes::dq_token_stream::dq_token_stream(&format!(
                                        "{{}}({{}}{} {{}})",
                                        pg_type_kind.format_argument()
                                    ));
                                quote::quote! {
                                    #maybe_dimensions_ies_initialization_token_stream
                                    #v_match_self_v_query_part_initialization_token_stream
                                    let mut query_part_8d4535a3 = String::with_capacity(32);
                                    if std::fmt::Write::write_fmt(
                                        &mut query_part_8d4535a3,
                                        format_args!(
                                            #format_token_stream,
                                            #self_operator_to_query_part_token_stream
                                            #column_snake_case,
                                            #maybe_extra_parameters_token_stream
                                            #values_snake_case
                                        ),
                                    )
                                    .is_err()
                                    {
                                        return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                    }
                                    Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_8d4535a3)?)
                                }
                            },
                            is_query_bind_mut_true,
                            quote::quote! {
                                #maybe_dimensions_query_bind_token_stream
                                #query_self_v_query_bind_token_stream
                            },
                        )
                    };
                    let generate_in_token_stream = |pg_type_ptrn: &PgTypePtrn| {
                        let (
                            maybe_dimensions_declaration_token_stream,
                            maybe_dimensions_default_initialization_token_stream,
                            maybe_dimensions_ies_initialization_token_stream,
                            pg_type_kind,
                            maybe_extra_parameters_token_stream,
                            maybe_dimensions_query_bind_token_stream,
                        ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                        (
                            Generic::True {
                                maybe_extra_traits_token_stream: Some({
                                    let sqlx_type_pg_encode_token_stream =
                                        generate_sqlx_type_pg_encode_token_stream();
                                    quote::quote! {std::fmt::Debug + PartialEq + Clone + #sqlx_type_pg_encode_token_stream}
                                }),
                            },
                            quote::quote! {
                                #maybe_dimensions_declaration_token_stream
                                #values_snake_case: crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec<T>
                            },
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                            {
                                let format_token_stream =
                                    generate_quotes::dq_token_stream::dq_token_stream(&format!(
                                        "{{}}({{}}{} in (",
                                        pg_type_kind.format_argument()
                                    ));
                                let if_write_is_err_token_stream =
                                macro_helpers::generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream(
                                    &quote::quote! {query_part_bce8c9ae, "${v_daedba9c},"},
                                    &quote::quote! {return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });},
                                );
                                quote::quote! {
                                    #maybe_dimensions_ies_initialization_token_stream
                                    let values = #self_snake_case.#values_snake_case.as_slice();
                                    let mut query_part_bce8c9ae = String::with_capacity(
                                        32usize.saturating_add(values.len().saturating_mul(8))
                                    );
                                    if std::fmt::Write::write_fmt(
                                        &mut query_part_bce8c9ae,
                                        format_args!(
                                            #format_token_stream,
                                            #self_operator_to_query_part_token_stream
                                            #column_snake_case,
                                            #maybe_extra_parameters_token_stream
                                        ),
                                    )
                                    .is_err()
                                    {
                                        return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                    }
                                    values.iter().try_for_each(|_| {
                                        let v_daedba9c = #import::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(#increment_snake_case)?;
                                        #if_write_is_err_token_stream
                                        Ok::<(), #import::query_part_error::QueryPartError>(())
                                    })?;
                                    let _: Option<char> = query_part_bce8c9ae.pop();
                                    query_part_bce8c9ae.push_str("))");
                                    Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_bce8c9ae)?)
                                }
                            },
                            is_query_bind_mut_true,
                            quote::quote! {
                                #maybe_dimensions_query_bind_token_stream
                                for element in Vec::from(#self_snake_case.#values_snake_case) {
                                    if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(element) {
                                        return Err(#import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(#error_snake_case));
                                    }
                                }
                                Ok(#query_snake_case)
                            },
                        )
                    };
                    let generate_regex_token_stream = |pg_type_ptrn: &PgTypePtrn| {
                        let (
                            maybe_dimensions_declaration_token_stream,
                            maybe_dimensions_default_initialization_token_stream,
                            maybe_dimensions_ies_initialization_token_stream,
                            pg_type_kind,
                            maybe_extra_parameters_token_stream,
                            maybe_dimensions_query_bind_token_stream,
                        ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                        (
                            Generic::False,
                            add_regex_case_and_v_declaration_token_stream(
                                &maybe_dimensions_declaration_token_stream,
                            ),
                            add_regex_case_and_v_default_initialization_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                            generate_regex_query_part_format_token_stream(
                                &format!("{{}}({{}}{} {{}} ${{}})", pg_type_kind.format_argument()),
                                &maybe_dimensions_ies_initialization_token_stream,
                                &maybe_extra_parameters_token_stream,
                            ),
                            is_query_bind_mut_true,
                            generate_two_token_stream(
                                &maybe_dimensions_query_bind_token_stream,
                                &if_let_err_query_try_bind_self_v_to_string_token_stream,
                            ),
                        )
                    };
                    let generate_pg_syntax_filter_token_stream =
                        |pg_type_ptrn: &PgTypePtrn, pg_syntax: &dyn std::fmt::Display| {
                            let (
                                maybe_dimensions_declaration_token_stream,
                                maybe_dimensions_default_initialization_token_stream,
                                maybe_dimensions_ies_initialization_token_stream,
                                pg_type_kind,
                                maybe_extra_parameters_token_stream,
                                maybe_dimensions_query_bind_token_stream,
                            ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                            (
                                Generic::False,
                                maybe_dimensions_declaration_token_stream,
                                maybe_dimensions_default_initialization_token_stream,
                                pg_crud_macro_common::emission_types::IncrementParameterUndrscr::True,
                                {
                                    let format_token_stream =
                                        generate_quotes::dq_token_stream::dq_token_stream(
                                            &format!(
                                                "{{}}({{}}{} {pg_syntax})",
                                                pg_type_kind.format_argument()
                                            ),
                                        );
                                    quote::quote! {
                                        #maybe_dimensions_ies_initialization_token_stream
                                        let mut query_part_1a7fed15 = String::with_capacity(32);
                                        if std::fmt::Write::write_fmt(
                                            &mut query_part_1a7fed15,
                                            format_args!(
                                                #format_token_stream,
                                                #self_operator_to_query_part_token_stream
                                                #column_snake_case,
                                                #maybe_extra_parameters_token_stream
                                            ),
                                        )
                                        .is_err()
                                        {
                                            return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                        }
                                        Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_1a7fed15)?)
                                    }
                                },
                                is_query_bind_mut_false,
                                quote::quote! {
                                    #maybe_dimensions_query_bind_token_stream
                                    Ok(#query_snake_case)
                                },
                            )
                        };
                    let generate_eq_to_encoded_string_representation_token_stream =
                        |pg_type_ptrn: &PgTypePtrn| {
                            let (
                                maybe_dimensions_declaration_token_stream,
                                maybe_dimensions_default_initialization_token_stream,
                                maybe_dimensions_ies_initialization_token_stream,
                                pg_type_kind,
                                maybe_extra_parameters_token_stream,
                                maybe_dimensions_query_bind_token_stream,
                            ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                            (
                                Generic::False,
                                quote::quote! {
                                    #maybe_dimensions_declaration_token_stream
                                    encode_format: crate::encode_format::EncodeFormat,
                                    encoded_string_representation: String,
                                },
                                quote::quote! {
                                    #maybe_dimensions_default_initialization_token_stream
                                    encode_format: #pg_crud_common_default_some_one_element_call,
                                    encoded_string_representation: String::default()
                                },
                                pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                                {
                                    let format_token_stream =
                                        generate_quotes::dq_token_stream::dq_token_stream(&format!(
                                            "{{}}(encode({{}}{}, '{{}}') = ${{}})",
                                            pg_type_kind.format_argument()
                                        ));
                                    quote::quote! {
                                        #maybe_dimensions_ies_initialization_token_stream
                                        #v_match_increment_checked_add_one_initialization_token_stream
                                        let mut query_part_7a76888d = String::with_capacity(32);
                                        if std::fmt::Write::write_fmt(
                                            &mut query_part_7a76888d,
                                            format_args!(
                                                #format_token_stream,
                                                #self_operator_to_query_part_token_stream
                                                #column_snake_case,
                                                #maybe_extra_parameters_token_stream
                                                &#self_snake_case.encode_format,
                                                #values_snake_case
                                            ),
                                        )
                                        .is_err()
                                        {
                                            return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                        }
                                        Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_7a76888d)?)
                                    }
                                },
                                is_query_bind_mut_true,
                                quote::quote! {
                                    #maybe_dimensions_query_bind_token_stream
                                    if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(self.encoded_string_representation) {
                                        return Err(#import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(#error_snake_case));
                                    }
                                    Ok(#query_snake_case)
                                },
                            )
                        };
                    let generate_range_bound_cmp_filter_token_stream =
                        |pg_type_ptrn: &PgTypePtrn, bound_fn, operator| {
                            generate_cmp_filter_token_stream(
                                pg_type_ptrn,
                                &|pg_type_kind: &PgTypeKind| {
                                    format!(
                                        "{{}}({bound_fn}({{}}{}) {operator} ${{}})",
                                        pg_type_kind.format_argument()
                                    )
                                },
                            )
                        };
                    let generate_range_len_token_stream = |pg_type_ptrn: &PgTypePtrn| {
                        let (
                            maybe_dimensions_declaration_token_stream,
                            maybe_dimensions_default_initialization_token_stream,
                            maybe_dimensions_ies_initialization_token_stream,
                            pg_type_kind,
                            maybe_extra_parameters_token_stream,
                            maybe_dimensions_query_bind_token_stream,
                        ) = generate_pg_type_dimensions_helpers_pg_type(pg_type_ptrn);
                        (
                            Generic::False,
                            quote::quote! {
                                #maybe_dimensions_declaration_token_stream
                                #values_snake_case: #import::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32
                            },
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                            generate_query_part_format_with_v_token_stream(
                                &maybe_dimensions_ies_initialization_token_stream,
                                &generate_quotes::dq_token_stream::dq_token_stream(&format!(
                                    "{{}}(upper({{}}{}) - lower({{}}{}) = ${{}})",
                                    pg_type_kind.format_argument(),
                                    pg_type_kind.format_argument(),
                                )),
                                &quote::quote! {
                                    #maybe_extra_parameters_token_stream
                                    #column_snake_case,
                                },
                            ),
                            is_query_bind_mut_true,
                            quote::quote! {
                                #maybe_dimensions_query_bind_token_stream
                                #query_bind_one_v_token_stream
                            },
                        )
                    };
                    let equality_sql_operator =
                        crate::filter_sql_operator_value::filter_sql_operator_value(
                            crate::filter_spec::FilterSpec::equality(),
                        );
                    let generate_eq_operator_query_part_token_stream =
                        |maybe_dimensions_ies_initialization_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                #maybe_dimensions_ies_initialization_token_stream
                                let operator = <T as #import::pg_type_eq_operator::PgTypeEqOperator>::operator(&#self_snake_case.#values_snake_case);
                                let mut query_part_6e4b019d = String::with_capacity(48);
                                let write_result_6e4b019d = match operator {
                                    #import::eq_operator::EqOperator::Eq => {
                                        #v_match_increment_checked_add_one_initialization_token_stream
                                        std::fmt::Write::write_fmt(
                                            &mut query_part_6e4b019d,
                                            format_args!("{}({} {} ${values})", #self_operator_to_query_part_token_stream #column_snake_case, #equality_sql_operator),
                                        )
                                    },
                                    #import::eq_operator::EqOperator::IsNull => std::fmt::Write::write_fmt(
                                        &mut query_part_6e4b019d,
                                        format_args!("{}({} is null)", #self_operator_to_query_part_token_stream #column_snake_case),
                                    ),
                                };
                                if write_result_6e4b019d.is_err() {
                                    return Err(#import::query_part_error::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                }
                                Ok(#import::query_part_fragment::QueryPartFragment::try_from(query_part_6e4b019d)?)
                            }
                        };
                    let generate_eq_operator_query_bind_token_stream =
                        |ts: &dyn quote::ToTokens| {
                            quote::quote! {
                                #ts
                                if matches!(&<T as #import::pg_type_eq_operator::PgTypeEqOperator>::operator(&#self_snake_case.#values_snake_case), #import::eq_operator::EqOperator::Eq)
                                    && let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(#self_snake_case.#values_snake_case)
                                {
                                    return Err(#import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(#error_snake_case));
                                }
                                Ok(#query_snake_case)
                            }
                        };
                    match &filter {
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::Eq { .. } => {
                        let (
                            maybe_dimensions_declaration_token_stream,
                            maybe_dimensions_default_initialization_token_stream,
                            maybe_dimensions_ies_initialization_token_stream,
                            _,
                            _,
                            maybe_dimensions_query_bind_token_stream,
                        ) = generate_pg_type_dimensions_helpers_pg_type(&pg_type_ptrn_standard);
                        (
                            Generic::True {
                                maybe_extra_traits_token_stream: Some({
                                    let sqlx_type_pg_encode_token_stream = generate_sqlx_type_pg_encode_token_stream();
                                    quote::quote! {#sqlx_type_pg_encode_token_stream + #import::pg_type_eq_operator::PgTypeEqOperator}
                                }),
                            },
                            generate_maybe_dimensions_declaration_pub_v_t_token_stream(&maybe_dimensions_declaration_token_stream),
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                            pg_crud_macro_common::emission_types::IncrementParameterUndrscr::False,
                            generate_eq_operator_query_part_token_stream(&maybe_dimensions_ies_initialization_token_stream),
                            is_query_bind_mut_true,
                            generate_eq_operator_query_bind_token_stream(&maybe_dimensions_query_bind_token_stream),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::GreaterThan { .. } => {
                        generate_greater_than_token_stream(&pg_type_ptrn_standard)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::Between { .. } => generate_between_token_stream(&pg_type_ptrn_standard),
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::In { .. } => generate_in_token_stream(&pg_type_ptrn_standard),
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::Regex => generate_regex_token_stream(&pg_type_ptrn_standard),
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::Before { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::filter_spec::FilterSpec::before().sql_operator(),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::CurrentDate => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &constants_str::CURRENT_DATE)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::GreaterThanCurrentDate => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &constants_str::CURRENT_DATE_ALT)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::CurrentTimestamp => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &constants_str::CURRENT_TIMESTAMP)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::GreaterThanCurrentTimestamp => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &constants_str::CURRENT_TIMESTAMP_ALT)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::CurrentTime => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &constants_str::CURRENT_TIME)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::GreaterThanCurrentTime => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &constants_str::CURRENT_TIME_ALT)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::EqToEncodedStringRepresentation => {
                        generate_eq_to_encoded_string_representation_token_stream(&pg_type_ptrn_standard)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::FindRangesWithinGivenRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::filter_spec::FilterSpec::within().sql_operator(),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::FindRangesThatFullyContainTheGivenRange {
                        ..
                    } => generate_operator_cmp_filter_token_stream(
                        &pg_type_ptrn_standard,
                        &crate::filter_spec::FilterSpec::contains().sql_operator(),
                    ),
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::StrictlyToLeftOfRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::filter_spec::FilterSpec::left_of().sql_operator(),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::StrictlyToRightOfRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::filter_spec::FilterSpec::right_of().sql_operator(),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::IncludedLowerBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, constants_str::LOWER, constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::ExcludedUpperBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, constants_str::UPPER, constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::GreaterThanIncludedLowerBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, constants_str::LOWER, constants_str::TEXT_ALT_11)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::GreaterThanExcludedUpperBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, constants_str::UPPER, constants_str::TEXT_ALT_11)
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::OverlapWithRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::filter_spec::FilterSpec::overlaps().sql_operator(),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::AdjacentWithRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::filter_spec::FilterSpec::adjacent().sql_operator(),
                        )
                    }
                    pg_crud_macro_common::pg_type_filter::PgTypeFilter::RangeLen => {
                        generate_range_len_token_stream(&pg_type_ptrn_standard)
                    }
                }
                };
                let struct_token_stream = generate_struct_token_stream(
                    false,
                    &generic,
                    &identifier,
                    &struct_extra_fields_token_stream,
                );
                let impl_default_some_one_element_token_stream =
                    generate_impl_default_some_one_element_token_stream(
                        &generic,
                        &identifier,
                        &impl_default_some_one_element_extra_fields_token_stream,
                    );
                let impl_pg_type_where_filter_token_stream =
                    generate_impl_pg_type_where_filter_token_stream(
                        &generic,
                        &identifier,
                        &increment_parameter_undrscr,
                        &pg_crud_macro_common::emission_types::AddOperatorUndrscr::False,
                        &query_part_token_stream,
                        &is_query_bind_mut,
                        &query_bind_token_stream,
                    );
                let gend = quote::quote! {
                    #struct_token_stream
                    #impl_default_some_one_element_token_stream
                    #impl_pg_type_where_filter_token_stream
                };
                gend
            };
        let filter_array_token_stream =
            <pg_crud_macro_common::pg_type_filter::PgTypeFilter as strum::IntoEnumIterator>::iter()
                .map(|element| generate_filters_token_stream(&element));
        let gend = quote::quote! {#(#filter_array_token_stream)*};
        if let Err(error) =
            macro_helpers::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
                pg_types_write_into_file,
                constants_str::GENERATE_WHERE_FILTERS_PG_TYPES,
                macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&gend),
                &macro_helpers::format_with_cargofmt::FormatWithCargofmt::True,
            )
        {
            let message = format!("failed to write generated where-filter PG types: {error}");
            return crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            );
        }
        gend
    };
    let text_search_spec = crate::filter_spec::FilterSpec::text_search();
    #[allow(
        clippy::if_not_else,
        reason = "schema validation failure is the exceptional branch"
    )]
    let text_search_schema_token_stream = if !crate::schema_uses_text_value::schema_uses_text_value(
        text_search_spec,
    )
    .get()
    {
        quote::quote! {compile_error!("text search schema requires text value shape");}
    } else {
        quote::quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::IntoInnerFrom)]
            pub struct TextSearchMaximumInputBytes(usize);
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct TextSearchPolicy {
                maximum_input_bytes: TextSearchMaximumInputBytes,
            }
            impl TextSearchPolicy {
                pub const DEFAULT: Self = Self {
                    maximum_input_bytes: TextSearchMaximumInputBytes(1_024usize),
                };
                pub const fn maximum_input_bytes(self) -> TextSearchMaximumInputBytes {
                    self.maximum_input_bytes
                }
            }
            #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema, utoipa::ToSchema)]
            #[serde(rename_all = "snake_case")]
            pub enum TextSearchMode {
                Contains,
                EndsWith,
                StartsWith,
            }
            #[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefStr, newtype::IntoInnerFrom)]
            pub struct TextSearchPattern(String);
            #[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
            pub enum TextSearchValueError {
                #[error("text search value must not be empty")]
                Empty,
                #[error("text search value exceeds {maximum_bytes} bytes: got {actual_bytes}")]
                TooLong { actual_bytes: usize, maximum_bytes: usize },
            }
            pub fn build_text_search_pattern(value: &str, mode: TextSearchMode) -> Result<TextSearchPattern, TextSearchValueError> {
                if value.is_empty() {
                    return Err(TextSearchValueError::Empty);
                }
                let maximum_input_bytes = usize::from(TextSearchPolicy::DEFAULT.maximum_input_bytes());
                if value.len() > maximum_input_bytes {
                    return Err(TextSearchValueError::TooLong {
                        actual_bytes: value.len(),
                        maximum_bytes: maximum_input_bytes,
                    });
                }
                let wildcard_count = match mode {
                    TextSearchMode::Contains => 2usize,
                    TextSearchMode::EndsWith | TextSearchMode::StartsWith => constants_usize::ONE,
                };
                let escaped_symbol_count = value.as_bytes().iter().copied().filter(|byte| matches!(byte, b'\\' | b'%' | b'_')).count();
                let mut pattern = String::with_capacity(value.len().saturating_add(escaped_symbol_count).saturating_add(wildcard_count));
                if matches!(mode, TextSearchMode::Contains | TextSearchMode::EndsWith) {
                    pattern.push('%');
                }
                value.chars().for_each(|character| {
                    if matches!(character, '\\' | '%' | '_') {
                        pattern.push('\\');
                    }
                    pattern.push(character);
                });
                if matches!(mode, TextSearchMode::Contains | TextSearchMode::StartsWith) {
                    pattern.push('%');
                }
                Ok(TextSearchPattern(pattern))
            }
            #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema, utoipa::ToSchema)]
            #[serde(deny_unknown_fields)]
            pub struct PgTypeWhereTextSearch {
                value: String,
                mode: TextSearchMode,
                operator: pg_crud_common::operator::Operator,
            }
        }
    };
    #[allow(
        clippy::if_not_else,
        reason = "client validation failure is the exceptional branch"
    )]
    let text_search_client_token_stream = if !crate::client_uses_text_value::client_uses_text_value(
        text_search_spec,
    )
    .get()
    {
        quote::quote! {compile_error!("text search client requires text value shape");}
    } else {
        quote::quote! {
            impl PgTypeWhereTextSearch {
                pub fn try_new(operator: pg_crud_common::operator::Operator, mode: TextSearchMode, value: String) -> Result<Self, TextSearchValueError> {
                    let _validated_pattern = build_text_search_pattern(value.as_str(), mode)?;
                    Ok(Self { value, mode, operator })
                }
                pub fn pattern(&self) -> Result<TextSearchPattern, TextSearchValueError> {
                    build_text_search_pattern(self.value.as_str(), self.mode)
                }
            }
        }
    };
    #[allow(
        clippy::if_not_else,
        reason = "bind-count validation failure is the exceptional branch"
    )]
    let text_search_bind_token_stream = if !crate::bind_count_matches::bind_count_matches(
        text_search_spec,
        crate::filter_placeholder_count::FilterPlaceholderCount::one(),
    )
    .get()
    {
        quote::quote! {compile_error!("text search bind count must match one placeholder");}
    } else {
        let sql_operator =
            crate::filter_sql_operator_value::filter_sql_operator_value(text_search_spec);
        let sql_suffix = crate::filter_sql_suffix_value::filter_sql_suffix_value(text_search_spec);
        quote::quote! {
            impl<'query_lt> pg_crud_common::pg_type_where_filter::PgTypeWhereFilter<'query_lt> for PgTypeWhereTextSearch {
                fn query_bind(self, mut query: pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>) -> Result<pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>, pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> {
                    let pattern = self.pattern().map_err(pg_crud_common::make_query_bind_error::make_query_bind_error)?;
                    if let Err(error) = query.as_mut().try_bind(String::from(pattern)) {
                        return Err(pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from(error));
                    }
                    Ok(query)
                }
                fn query_part(&self, increment: &mut dyn pg_crud_common::query_part_increment_mut::QueryPartIncrementMut, column: pg_crud_common::sql_column_ref::SqlColumnRef<'_>, add_operator: pg_crud_common::add_operator::AddOperator) -> Result<pg_crud_common::query_part_fragment::QueryPartFragment, pg_crud_common::query_part_error::QueryPartError> {
                    let parameter = increment.checked_add_one().ok_or_else(|| pg_crud_common::query_part_error::QueryPartError::CheckedAdd { location: location_macros::location!() })?;
                    let fragment = format!("{}{} {} ${parameter} {}", self.operator.to_query_part(add_operator), column, #sql_operator, #sql_suffix);
                    pg_crud_common::query_part_fragment::QueryPartFragment::try_from(fragment).map_err(pg_crud_common::query_part_error::QueryPartError::from)
                }
            }
        }
    };
    let text_search_token_stream = quote::quote! {
        #text_search_schema_token_stream
        #text_search_client_token_stream
        #text_search_bind_token_stream
    };
    let gend = quote::quote! {
        #text_search_token_stream
        #pg_type_token_stream
    };
    if let Err(error) =
        macro_helpers::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
            whole_write_into_file,
            constants_str::CODE_STYLE_GENERATE_WHERE_FILTERS_MACRO_NAME,
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&gend),
            &macro_helpers::format_with_cargofmt::FormatWithCargofmt::True,
        )
    {
        let message = format!("failed to write generated where-filter output: {error}");
        return crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream::from(
            quote::quote! { compile_error!(#message); },
        );
    }
    crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream::from(gend)
}
