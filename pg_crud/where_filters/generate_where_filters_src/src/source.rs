#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub struct ProcMacro2GenerateWhereFiltersInput<'input_lt>(&'input_lt proc_macro2::TokenStream);
#[derive(
    Debug, newtype::AsRefOwned, newtype::Display, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct ProcMacro2GenerateWhereFiltersTokenStream(proc_macro2::TokenStream);
#[derive(Clone, Copy, Debug, serde::Deserialize, optml::Optml)]
pub struct ParsedGenerateWhereFiltersConfig {
    pg_types_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
    whole_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
}
#[derive(Clone, Copy, Debug)]
pub struct BuiltGenerateWhereFiltersModel {
    config: ParsedGenerateWhereFiltersConfig,
    contract_valid: crate::model::FilterSpecValid,
}
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ValidatedGenerateWhereFiltersConfig(ParsedGenerateWhereFiltersConfig);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct SerdeJsonGenerateWhereFiltersError(serde_json::Error);
#[derive(Debug, thiserror::Error)]
pub enum GenerateWhereFiltersPipelineError {
    #[error("{}", str_constants::INVALID_FILTER_SPECIFICATION)]
    InvalidContract,
    #[error("{0}")]
    Parse(SerdeJsonGenerateWhereFiltersError),
}
pub fn parse_generate_where_filters(
    input: ProcMacro2GenerateWhereFiltersInput<'_>,
) -> Result<ParsedGenerateWhereFiltersConfig, GenerateWhereFiltersPipelineError> {
    serde_json::from_str(&input.as_ref().to_string()).map_err(|error| {
        GenerateWhereFiltersPipelineError::Parse(SerdeJsonGenerateWhereFiltersError::from(error))
    })
}
pub fn build_generate_where_filters(
    parsed: ParsedGenerateWhereFiltersConfig,
) -> Result<BuiltGenerateWhereFiltersModel, GenerateWhereFiltersPipelineError> {
    let valid = [
        crate::model::FilterSpec::adjacent(),
        crate::model::FilterSpec::before(),
        crate::model::FilterSpec::contains(),
        crate::model::FilterSpec::equality(),
        crate::model::FilterSpec::left_of(),
        crate::model::FilterSpec::overlaps(),
        crate::model::FilterSpec::right_of(),
        crate::model::FilterSpec::text_search(),
        crate::model::FilterSpec::within(),
    ]
    .into_iter()
    .all(|spec| crate::contract_tests::filter_spec_contract_is_valid(spec).get());
    Ok(BuiltGenerateWhereFiltersModel {
        config: parsed,
        contract_valid: crate::model::FilterSpecValid::from(valid),
    })
}
pub fn validate_generate_where_filters(
    built: BuiltGenerateWhereFiltersModel,
) -> Result<ValidatedGenerateWhereFiltersConfig, GenerateWhereFiltersPipelineError> {
    if built.contract_valid.get() {
        Ok(ValidatedGenerateWhereFiltersConfig::from(built.config))
    } else {
        Err(GenerateWhereFiltersPipelineError::InvalidContract)
    }
}
#[must_use]
pub fn generate_where_filters(
    input_token_stream: ProcMacro2GenerateWhereFiltersInput<'_>,
) -> ProcMacro2GenerateWhereFiltersTokenStream {
    match parse_generate_where_filters(input_token_stream)
        .and_then(build_generate_where_filters)
        .and_then(validate_generate_where_filters)
    {
        Ok(validated) => emit_generate_where_filters(validated),
        Err(error) => {
            let message = error.to_string();
            ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
#[must_use]
pub fn emit_generate_where_filters(
    validated: ValidatedGenerateWhereFiltersConfig,
) -> ProcMacro2GenerateWhereFiltersTokenStream {
    #[derive(Clone, optml::Optml)]
    enum Generic {
        False,
        True {
            maybe_extra_traits_token_stream: Option<proc_macro2::TokenStream>,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Clone, optml::Optml)]
    enum PgTypePtrn {
        Standard,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum PgTypeKind {
        Standard,
    }
    impl PgTypeKind {
        const fn format_argument(&self) -> &'static str {
            match &self {
                Self::Standard => str_constants::PG_CRUD_EMPTY_SQL_SUFFIX,
            }
        }
    }
    panic_location::panic_location();
    let generate_where_filters_config = validated.0;
    let column_snake_case = naming::ColumnSnakeCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let pub_snake_case = naming::PubSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let self_snake_case = naming::SelfSnakeCase;
    let v_snake_case = naming::VSnakeCase;
    let pg_crud_common_default_some_one_element = token_patterns::PgCrudCommonDefaultSomeOneElement;
    let pg_crud_common_default_some_one_element_call =
        token_patterns::PgCrudCommonDefaultSomeOneElementCall;
    let import = pg_crud_macros_common::Import::PgCrudCommon;
    let t_token_stream = quote::quote! {T};
    let t_ann_generic_token_stream = quote::quote! {<#t_token_stream>};
    let proc_macro2_token_stream_new = proc_macro2::TokenStream::new();
    let pub_v_t_token_stream = quote::quote! {#[schema(inline)] pub #v_snake_case: T};
    let v_default_some_one_element_token_stream = quote::quote! {
        #v_snake_case: #pg_crud_common_default_some_one_element_call
    };
    let generate_struct_token_stream =
        |filter_initialization_with_try_new_result_is_ok,
         generic: &Generic,
         identifier: &dyn quote::ToTokens,
         struct_extra_fields_token_stream: &dyn quote::ToTokens| {
            let maybe_pub_token_stream: &dyn quote::ToTokens =
                if filter_initialization_with_try_new_result_is_ok {
                    &proc_macro2_token_stream_new
                } else {
                    &pub_snake_case
                };
            macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize_if(if filter_initialization_with_try_new_result_is_ok {
                    macros_helpers::derive_token_stream_builder::DSerdeDeserialize::False
                } else {
                    macros_helpers::derive_token_stream_builder::DSerdeDeserialize::True
                })
                .d_schemars_json_schema()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
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
                        #maybe_pub_token_stream operator: #import::Operator,
                        #struct_extra_fields_token_stream
                    }},
                )
        };
    let generate_impl_default_some_one_element_token_stream =
        |generic: &Generic, identifier: &dyn quote::ToTokens, ts: &dyn quote::ToTokens| {
            pg_crud_macros_common::generate_impl_default_some_one_element_token_stream(
                &match &generic {
                    Generic::False => proc_macro2::TokenStream::new(),
                    Generic::True {
                        maybe_extra_traits_token_stream,
                    } => maybe_extra_traits_token_stream.as_ref().map_or_else(
                        || quote::quote! {<T: #pg_crud_common_default_some_one_element>},
                        |v| quote::quote! {<T: #v + #pg_crud_common_default_some_one_element>},
                    ),
                },
                &pg_crud_macros_common::Import::PgCrudCommon,
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
         increment_parameter_undrscr: &pg_crud_macros_common::IncrementParameterUndrscr,
         add_operator_undrscr: &pg_crud_macros_common::AddOperatorUndrscr,
         query_part_token_stream: &dyn quote::ToTokens,
         is_query_bind_mut: &pg_crud_macros_common::IsQueryBindMut,
         query_bind_token_stream: &dyn quote::ToTokens| {
            pg_crud_macros_common::impl_pg_type_where_filter_for_identifier_token_stream(
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
                &pg_crud_macros_common::ColumnParameterUndrscr::False,
                add_operator_undrscr,
                &query_part_token_stream,
                is_query_bind_mut,
                &query_bind_token_stream,
                &pg_crud_macros_common::Import::PgCrudCommon,
            )
        };
    let add_regex_case_and_v_declaration_token_stream = |ts: &dyn quote::ToTokens| {
        quote::quote! {
            #ts
            pub regex_case: RegexCase,
            pub #v_snake_case: RegexRegex
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
                pg_crud_macros_common::token_stream_helpers::generate_match_ok_or_return_err_token_stream(
                    &quote::quote! {#import::increment_checked_add_one_returning_increment(#increment_snake_case)},
                    &quote::quote! {v_25d59e01},
                );
            quote::quote! {
                let #ts = #match_token_stream;
            }
        };
    let v_match_increment_checked_add_one_initialization_token_stream =
        generate_match_increment_checked_add_one_initialization_token_stream(&v_snake_case);
    let self_operator_to_query_part_token_stream =
        quote::quote! {&#self_snake_case.operator.to_query_part(add_operator),};
    let generate_regex_query_part_format_token_stream =
        |v: &dyn std::fmt::Display,
         maybe_dimensions_ies_initialization_token_stream: &dyn quote::ToTokens,
         maybe_extra_parameters_token_stream: &dyn quote::ToTokens| {
            let format_token_stream = generate_quotes::dq_token_stream(&v);
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
                        #v_snake_case
                    ),
                )
                .is_err()
                {
                    return Err(#import::QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(#import::QueryPartFragment::try_from(query_part_28bc96ee)?)
            }
        };
    let if_let_err_query_try_bind_self_v_to_string_token_stream = quote::quote! {
        if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(#self_snake_case.#v_snake_case.to_string()) {
            return Err(#import::SqlxPostgresQueryBindError::from(#error_snake_case));
        }
        Ok(#query_snake_case)
    };
    let if_let_err_query_try_bind_self_v_token_stream = quote::quote! {
        if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(#self_snake_case.#v_snake_case) {
            return Err(#import::SqlxPostgresQueryBindError::from(#error_snake_case));
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
        quote::quote! {#[schema(inline)] pub #v_snake_case: Between<T>};
    let generate_match_query_bind_token_stream = |field_token_stream: &dyn quote::ToTokens| {
        pg_crud_macros_common::token_stream_helpers::generate_match_ok_assign_or_return_err_token_stream(
            &quote::quote! {#field_token_stream.query_bind(#query_snake_case)},
            &query_snake_case,
            &quote::quote! {v_f6d31bdd},
        )
    };
    let query_self_v_query_bind_token_stream = {
        let ts =
            generate_match_query_bind_token_stream(&quote::quote! {#self_snake_case.#v_snake_case});
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
                pg_crud_macros_common::token_stream_helpers::generate_match_ok_or_return_err_token_stream(
                    &quote::quote! {self.#field_token_stream.#fn_token_stream(#increment_snake_case, #column_snake_case, add_operator)},
                    &quote::quote! {v_0a22ee9a},
                );
            quote::quote! {
                let #identifier_token_stream = #match_token_stream;
            }
        };
    let v_match_self_v_query_part_initialization_token_stream =
        generate_identifier_match_field_fn_ok_v_return_err_token_stream(
            &v_snake_case,
            &v_snake_case,
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
    let is_query_bind_mut_true = pg_crud_macros_common::IsQueryBindMut::True;
    let is_query_bind_mut_false = pg_crud_macros_common::IsQueryBindMut::False;
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
                        #v_snake_case
                    ),
                )
                .is_err()
                {
                    return Err(#import::QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(#import::QueryPartFragment::try_from(query_part_1c95685d)?)
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
            |filter: &pg_crud_macros_common::filters::PgTypeFilter| {
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
                     generate_format_handle_str: &dyn Fn(&PgTypeKind) -> String| {
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
                            generate_maybe_dimensions_declaration_pub_v_t_token_stream(&maybe_dimensions_declaration_token_stream),
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                            pg_crud_macros_common::IncrementParameterUndrscr::False,
                            generate_query_part_format_with_v_token_stream(
                                &maybe_dimensions_ies_initialization_token_stream,
                                &generate_quotes::dq_token_stream(&generate_format_handle_str(&pg_type_kind)),
                                &maybe_extra_parameters_token_stream,
                            ),
                            is_query_bind_mut_true,
                            generate_two_token_stream(&maybe_dimensions_query_bind_token_stream, &query_bind_one_v_token_stream),
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
                            &str_constants::TEXT_ALT_11,
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
                            pg_crud_macros_common::IncrementParameterUndrscr::False,
                            {
                                let format_token_stream = generate_quotes::dq_token_stream(
                                    &format!("{{}}({{}}{} {{}})", pg_type_kind.format_argument()),
                                );
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
                                            #v_snake_case
                                        ),
                                    )
                                    .is_err()
                                    {
                                        return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                    }
                                    Ok(#import::QueryPartFragment::try_from(query_part_8d4535a3)?)
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
                                pub #v_snake_case: PgTypeNotEmptyUniqueVec<T>
                            },
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macros_common::IncrementParameterUndrscr::False,
                            {
                                let format_token_stream = generate_quotes::dq_token_stream(
                                    &format!("{{}}({{}}{} in (", pg_type_kind.format_argument()),
                                );
                                let if_write_is_err_token_stream =
                                macros_helpers::generate_if_write_is_err_token_stream::generate_if_write_is_err_token_stream(
                                    &quote::quote! {query_part_bce8c9ae, "${v_daedba9c},"},
                                    &quote::quote! {return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });},
                                );
                                quote::quote! {
                                    #maybe_dimensions_ies_initialization_token_stream
                                    let values = #self_snake_case.#v_snake_case.as_slice();
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
                                        return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                    }
                                    values.iter().try_for_each(|_| {
                                        let v_daedba9c = #import::increment_checked_add_one_returning_increment(#increment_snake_case)?;
                                        #if_write_is_err_token_stream
                                        Ok::<(), #import::QueryPartError>(())
                                    })?;
                                    let _: Option<char> = query_part_bce8c9ae.pop();
                                    query_part_bce8c9ae.push_str("))");
                                    Ok(#import::QueryPartFragment::try_from(query_part_bce8c9ae)?)
                                }
                            },
                            is_query_bind_mut_true,
                            quote::quote! {
                                #maybe_dimensions_query_bind_token_stream
                                for element in Vec::from(#self_snake_case.#v_snake_case) {
                                    if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(element) {
                                        return Err(#import::SqlxPostgresQueryBindError::from(#error_snake_case));
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
                            pg_crud_macros_common::IncrementParameterUndrscr::False,
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
                                pg_crud_macros_common::IncrementParameterUndrscr::True,
                                {
                                    let format_token_stream =
                                        generate_quotes::dq_token_stream(&format!(
                                            "{{}}({{}}{} {pg_syntax})",
                                            pg_type_kind.format_argument()
                                        ));
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
                                            return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                        }
                                        Ok(#import::QueryPartFragment::try_from(query_part_1a7fed15)?)
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
                                    pub encode_format: EncodeFormat,
                                    pub encoded_string_representation: String,
                                },
                                quote::quote! {
                                    #maybe_dimensions_default_initialization_token_stream
                                    encode_format: #pg_crud_common_default_some_one_element_call,
                                    encoded_string_representation: String::default()
                                },
                                pg_crud_macros_common::IncrementParameterUndrscr::False,
                                {
                                    let format_token_stream =
                                        generate_quotes::dq_token_stream(&format!(
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
                                                #v_snake_case
                                            ),
                                        )
                                        .is_err()
                                        {
                                            return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                        }
                                        Ok(#import::QueryPartFragment::try_from(query_part_7a76888d)?)
                                    }
                                },
                                is_query_bind_mut_true,
                                quote::quote! {
                                    #maybe_dimensions_query_bind_token_stream
                                    if let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(self.encoded_string_representation) {
                                        return Err(#import::SqlxPostgresQueryBindError::from(#error_snake_case));
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
                                pub #v_snake_case: #import::NotZeroUnsignedPartOfI32
                            },
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(
                                &maybe_dimensions_default_initialization_token_stream,
                            ),
                            pg_crud_macros_common::IncrementParameterUndrscr::False,
                            generate_query_part_format_with_v_token_stream(
                                &maybe_dimensions_ies_initialization_token_stream,
                                &generate_quotes::dq_token_stream(&format!(
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
                        crate::sql::filter_sql_operator(crate::model::FilterSpec::equality());
                    let generate_eq_operator_query_part_token_stream =
                        |maybe_dimensions_ies_initialization_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {
                                #maybe_dimensions_ies_initialization_token_stream
                                let operator = <T as #import::PgTypeEqOperator>::operator(&#self_snake_case.#v_snake_case);
                                let mut query_part_6e4b019d = String::with_capacity(48);
                                let write_result_6e4b019d = match operator {
                                    #import::EqOperator::Eq => {
                                        #v_match_increment_checked_add_one_initialization_token_stream
                                        std::fmt::Write::write_fmt(
                                            &mut query_part_6e4b019d,
                                            format_args!("{}({} {} ${v})", #self_operator_to_query_part_token_stream #column_snake_case, #equality_sql_operator),
                                        )
                                    },
                                    #import::EqOperator::IsNull => std::fmt::Write::write_fmt(
                                        &mut query_part_6e4b019d,
                                        format_args!("{}({} is null)", #self_operator_to_query_part_token_stream #column_snake_case),
                                    ),
                                };
                                if write_result_6e4b019d.is_err() {
                                    return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                }
                                Ok(#import::QueryPartFragment::try_from(query_part_6e4b019d)?)
                            }
                        };
                    let generate_eq_operator_query_bind_token_stream =
                        |ts: &dyn quote::ToTokens| {
                            quote::quote! {
                                #ts
                                if matches!(&<T as #import::PgTypeEqOperator>::operator(&#self_snake_case.#v_snake_case), #import::EqOperator::Eq)
                                    && let Err(#error_snake_case) = #query_snake_case.as_mut().try_bind(#self_snake_case.#v_snake_case)
                                {
                                    return Err(#import::SqlxPostgresQueryBindError::from(#error_snake_case));
                                }
                                Ok(#query_snake_case)
                            }
                        };
                    match &filter {
                    pg_crud_macros_common::filters::PgTypeFilter::Eq { .. } => {
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
                                    quote::quote! {#sqlx_type_pg_encode_token_stream + #import::PgTypeEqOperator}
                                }),
                            },
                            generate_maybe_dimensions_declaration_pub_v_t_token_stream(&maybe_dimensions_declaration_token_stream),
                            generate_maybe_dimensions_default_initialization_v_default_token_stream(&maybe_dimensions_default_initialization_token_stream),
                            pg_crud_macros_common::IncrementParameterUndrscr::False,
                            generate_eq_operator_query_part_token_stream(&maybe_dimensions_ies_initialization_token_stream),
                            is_query_bind_mut_true,
                            generate_eq_operator_query_bind_token_stream(&maybe_dimensions_query_bind_token_stream),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThan { .. } => {
                        generate_greater_than_token_stream(&pg_type_ptrn_standard)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::Between { .. } => generate_between_token_stream(&pg_type_ptrn_standard),
                    pg_crud_macros_common::filters::PgTypeFilter::In { .. } => generate_in_token_stream(&pg_type_ptrn_standard),
                    pg_crud_macros_common::filters::PgTypeFilter::Regex => generate_regex_token_stream(&pg_type_ptrn_standard),
                    pg_crud_macros_common::filters::PgTypeFilter::Before { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::model::FilterSpec::before().sql_operator(),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::CurrentDate => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &str_constants::CURRENT_DATE)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentDate => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &str_constants::CURRENT_DATE_ALT)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::CurrentTimestamp => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &str_constants::CURRENT_TIMESTAMP)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentTimestamp => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &str_constants::CURRENT_TIMESTAMP_ALT)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::CurrentTime => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &str_constants::CURRENT_TIME)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentTime => {
                        generate_pg_syntax_filter_token_stream(&pg_type_ptrn_standard, &str_constants::CURRENT_TIME_ALT)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::EqToEncodedStringRepresentation => {
                        generate_eq_to_encoded_string_representation_token_stream(&pg_type_ptrn_standard)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::FindRangesWithinGivenRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::model::FilterSpec::within().sql_operator(),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::FindRangesThatFullyContainTheGivenRange {
                        ..
                    } => generate_operator_cmp_filter_token_stream(
                        &pg_type_ptrn_standard,
                        &crate::model::FilterSpec::contains().sql_operator(),
                    ),
                    pg_crud_macros_common::filters::PgTypeFilter::StrictlyToLeftOfRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::model::FilterSpec::left_of().sql_operator(),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::StrictlyToRightOfRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::model::FilterSpec::right_of().sql_operator(),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::IncludedLowerBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, str_constants::LOWER, str_constants::PG_CRUD_EQUALITY_SQL_OPERATOR)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::ExcludedUpperBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, str_constants::UPPER, str_constants::PG_CRUD_EQUALITY_SQL_OPERATOR)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanIncludedLowerBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, str_constants::LOWER, str_constants::TEXT_ALT_11)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanExcludedUpperBound { .. } => {
                        generate_range_bound_cmp_filter_token_stream(&pg_type_ptrn_standard, str_constants::UPPER, str_constants::TEXT_ALT_11)
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::OverlapWithRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::model::FilterSpec::overlaps().sql_operator(),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::AdjacentWithRange { .. } => {
                        generate_operator_cmp_filter_token_stream(
                            &pg_type_ptrn_standard,
                            &crate::model::FilterSpec::adjacent().sql_operator(),
                        )
                    }
                    pg_crud_macros_common::filters::PgTypeFilter::RangeLen => {
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
                        &pg_crud_macros_common::AddOperatorUndrscr::False,
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
            <pg_crud_macros_common::filters::PgTypeFilter as strum::IntoEnumIterator>::iter()
                .map(|element| generate_filters_token_stream(&element));
        let gend = quote::quote! {#(#filter_array_token_stream)*};
        if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
            generate_where_filters_config.pg_types_write_into_file,
            str_constants::GENERATE_WHERE_FILTERS_PG_TYPES,
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&gend),
            &macros_helpers::ts_writer::FormatWithCargofmt::True,
        ) {
            let message = format!("failed to write generated where-filter PG types: {error}");
            return ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            );
        }
        gend
    };
    let imports_token_stream = quote::quote! {
        #[allow(clippy::wildcard_imports)]
        use super::*;
    };
    let text_search_spec = crate::model::FilterSpec::text_search();
    let text_search_schema_token_stream = crate::schema::text_search_token_stream(text_search_spec);
    let text_search_client_token_stream = crate::client::text_search_token_stream(text_search_spec);
    let text_search_bind_token_stream = crate::bind::text_search_token_stream(text_search_spec);
    let text_search_token_stream = quote::quote! {
        #text_search_schema_token_stream
        #text_search_client_token_stream
        #text_search_bind_token_stream
    };
    let generate_where_filters_mod = quote::format_ident!("generate_where_filters_mod");
    let gend = quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(unused_variables)]
        #[allow(clippy::absolute_paths)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #generate_where_filters_mod {
            #imports_token_stream
            #text_search_token_stream
            #pg_type_token_stream
        }
        pub use #generate_where_filters_mod::*;
    };
    if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
        generate_where_filters_config.whole_write_into_file,
        str_constants::CODE_STYLE_GENERATE_WHERE_FILTERS_MACRO_NAME,
        macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&gend),
        &macros_helpers::ts_writer::FormatWithCargofmt::True,
    ) {
        let message = format!("failed to write generated where-filter output: {error}");
        return ProcMacro2GenerateWhereFiltersTokenStream::from(
            quote::quote! { compile_error!(#message); },
        );
    }
    ProcMacro2GenerateWhereFiltersTokenStream::from(gend)
}

#[cfg(test)]
mod pipeline_tests {
    #[test]
    fn config_builds_and_validates_without_emitting_source() {
        let input = quote::quote! {{
            "pg_types_write_into_file": "False",
            "whole_write_into_file": "False"
        }};
        let parsed = super::parse_generate_where_filters(
            super::ProcMacro2GenerateWhereFiltersInput::from(&input),
        )
        .expect("4fb319d6");
        let built = super::build_generate_where_filters(parsed).expect("98c270ea");
        let _validated = super::validate_generate_where_filters(built).expect("e61243af");
    }
}
