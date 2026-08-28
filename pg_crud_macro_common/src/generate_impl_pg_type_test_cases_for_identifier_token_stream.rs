pub fn generate_impl_pg_type_test_cases_for_identifier_token_stream(
    cfg_token_stream: &dyn quote::ToTokens,
    import: &crate::domain_types::Import,
    type_token_stream: &dyn quote::ToTokens,
    identifier: &dyn quote::ToTokens,
    optional_vec_create_token_stream: Option<&dyn quote::ToTokens>,
    read_ids_to_2_dimensions_vec_read_inner_token_stream: &dyn quote::ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_token_stream: &dyn quote::ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_token_stream: &dyn quote::ToTokens,
    update_to_read_ids_token_stream: &dyn quote::ToTokens,
    read_ids_to_optional_v_read_default_some_one_element_token_stream: &dyn quote::ToTokens,
    previous_read_and_optional_update_into_read_token_stream: &dyn quote::ToTokens,
    read_ids_and_create_into_read_token_stream: &dyn quote::ToTokens,
    read_ids_and_create_into_optional_v_read_token_stream: &dyn quote::ToTokens,
    read_ids_and_create_into_table_type_token_stream: &dyn quote::ToTokens,
    read_ids_and_create_into_where_eq_token_stream: &dyn quote::ToTokens,
    read_ids_and_create_into_vec_where_eq_using_fields_token_stream: &dyn quote::ToTokens,
    read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream: Option<
        &dyn quote::ToTokens,
    >,
    pg_type_optional_vec_where_greater_than_test_token_stream: Option<&dyn quote::ToTokens>,
    read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream: Option<
        &dyn quote::ToTokens,
    >,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = crate::domain_types::token_emission::NamesCtx::new();
    #[allow(
        non_snake_case,
        reason = "generated Rust identifiers intentionally mirror emitted naming tokens"
    )]
    let (
        AllowClippyArbitrarySrcItemOrdering,
        CreateUpperCamelCase,
        PgTypeOptionalVecWhereGreaterThanTestSnakeCase,
        PgTypeTestCasesUpperCamelCase,
        PgTypeUpperCamelCase,
        ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase,
        ReadIdsSnakeCase,
        ReadIdsUpperCamelCase,
        ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
        ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
        SelectUpperCamelCase,
        SelfUpperCamelCase,
        TableTypeSnakeCase,
        TableTypeUpperCamelCase,
        WhereUpperCamelCase,
    ) = (
        names.get_allow_clippy_arbitrary_src_item_ordering(),
        names.get_create_upper_camel_case(),
        names.get_pg_type_optional_vec_where_greater_than_test_snake_case(),
        names.get_pg_type_test_cases_upper_camel_case(),
        names.get_pg_type_upper_camel_case(),
        names.get_read_ids_and_table_type_into_pg_type_optional_where_greater_than_snake_case(),
        names.get_read_ids_snake_case(),
        names.get_read_ids_upper_camel_case(),
        names.get_read_inner_into_read_with_new_or_try_new_unwraped_snake_case(),
        names.get_read_inner_into_update_with_new_or_try_new_unwraped_snake_case(),
        names.get_select_upper_camel_case(),
        names.get_self_upper_camel_case(),
        names.get_table_type_snake_case(),
        names.get_table_type_upper_camel_case(),
        names.get_where_upper_camel_case(),
    );
    let self_pg_type_as_pg_type_token_stream = quote::quote! {<#SelfUpperCamelCase::#PgTypeUpperCamelCase as #import::#PgTypeUpperCamelCase>};
    let self_pg_type_as_pg_type_read_ids_token_stream =
        quote::quote! {#self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCase};
    let self_pg_type_as_pg_type_create_token_stream =
        quote::quote! {#self_pg_type_as_pg_type_token_stream::#CreateUpperCamelCase};
    let self_pg_type_as_pg_type_where_token_stream =
        quote::quote! {#self_pg_type_as_pg_type_token_stream::#WhereUpperCamelCase};
    let identifier_select_upper_camel_case =
        naming::domain_types::parameter::SelfSelectUpperCamelCase::from_tokens(&identifier);
    let optional_vec_create_token_stream_gnrtd: Option<
        macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    > = optional_vec_create_token_stream.map(|ts| {
        {
    let snippet_names_1 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (CreateUpperCamelCaseSnippet1, OptionalVecCreateSnakeCaseSnippet1) = (
            snippet_names_1.get_create_upper_camel_case(),
            snippet_names_1.get_optional_vec_create_snake_case(),
        );
        quote::quote! {
            fn #OptionalVecCreateSnakeCaseSnippet1() -> Option<Vec<#self_pg_type_as_pg_type_token_stream::#CreateUpperCamelCaseSnippet1>> {
                #ts
            }
        }
        .into()
}
    });
    let read_ids_to_2_dimensions_vec_read_inner_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_2 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (
            ReadIdsSnakeCaseSnippet2,
            ReadIdsTo2DimensionsVecReadInnerSnakeCaseSnippet2,
            ReadIdsUpperCamelCaseSnippet2,
            ReadInnerUpperCamelCaseSnippet2,
        ) = (
            snippet_names_2.get_read_ids_snake_case(),
            snippet_names_2.get_read_ids_to2_dimensions_vec_read_inner_snake_case(),
            snippet_names_2.get_read_ids_upper_camel_case(),
            snippet_names_2.get_read_inner_upper_camel_case(),
        );
        quote::quote! {
            fn #ReadIdsTo2DimensionsVecReadInnerSnakeCaseSnippet2(
                #ReadIdsSnakeCaseSnippet2: &#self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCaseSnippet2
            ) -> Vec<Vec<#self_pg_type_as_pg_type_token_stream::#ReadInnerUpperCamelCaseSnippet2>> {
                #read_ids_to_2_dimensions_vec_read_inner_token_stream
            }
        }
        .into()
    };
    let read_inner_into_read_with_new_or_try_new_unwraped_token_stream_gnrtd =
        super::generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream::generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream(
            &ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
            &type_token_stream,
            &self_pg_type_as_pg_type_token_stream,
            &naming::domain_types::ReadUpperCamelCase,
            &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_token_stream_gnrtd =
        super::generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream::generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream(
            &ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
            &type_token_stream,
            &self_pg_type_as_pg_type_token_stream,
            &naming::domain_types::UpdateUpperCamelCase,
            &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
        );
    let update_to_read_ids_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_3 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (ReadIdsUpperCamelCaseSnippet3, UpdateToReadIdsSnakeCaseSnippet3, UpdateUpperCamelCaseSnippet3, VSnakeCaseSnippet3) = (
            snippet_names_3.get_read_ids_upper_camel_case(),
            snippet_names_3.get_update_to_read_ids_snake_case(),
            snippet_names_3.get_update_upper_camel_case(),
            snippet_names_3.get_v_snake_case(),
        );
        quote::quote! {
            fn #UpdateToReadIdsSnakeCaseSnippet3(
                #VSnakeCaseSnippet3: &#self_pg_type_as_pg_type_token_stream::#UpdateUpperCamelCaseSnippet3
            ) -> #self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCaseSnippet3 {
                #update_to_read_ids_token_stream
            }
        }
        .into()
    };
    let read_ids_to_optional_v_read_default_some_one_element_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_4 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (
            ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCaseSnippet4,
            ReadIdsUpperCamelCaseSnippet4,
            ReadUpperCamelCaseSnippet4,
            VSnakeCaseSnippet4,
            VUpperCamelCaseSnippet4,
        ) = (
            snippet_names_4.get_read_ids_to_optional_v_read_default_some_one_element_snake_case(),
            snippet_names_4.get_read_ids_upper_camel_case(),
            snippet_names_4.get_read_upper_camel_case(),
            snippet_names_4.get_v_snake_case(),
            snippet_names_4.get_v_upper_camel_case(),
        );
        quote::quote! {
            fn #ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCaseSnippet4(
                #VSnakeCaseSnippet4: &#self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCaseSnippet4
            ) -> Option<#import::#VUpperCamelCaseSnippet4<#self_pg_type_as_pg_type_token_stream::#ReadUpperCamelCaseSnippet4>> {
                #read_ids_to_optional_v_read_default_some_one_element_token_stream
            }
        }
        .into()
    };
    let previous_read_and_optional_update_into_read_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_5 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (
            OptionalUpdateSnakeCaseSnippet5,
            PreviousReadAndOptionalUpdateIntoReadSnakeCaseSnippet5,
            ReadSnakeCaseSnippet5,
            ReadUpperCamelCaseSnippet5,
            UpdateUpperCamelCaseSnippet5,
        ) = (
            snippet_names_5.get_optional_update_snake_case(),
            snippet_names_5.get_previous_read_and_optional_update_into_read_snake_case(),
            snippet_names_5.get_read_snake_case(),
            snippet_names_5.get_read_upper_camel_case(),
            snippet_names_5.get_update_upper_camel_case(),
        );
        quote::quote! {
            fn #PreviousReadAndOptionalUpdateIntoReadSnakeCaseSnippet5(
                #ReadSnakeCaseSnippet5: #self_pg_type_as_pg_type_token_stream::#ReadUpperCamelCaseSnippet5,
                #OptionalUpdateSnakeCaseSnippet5: Option<#self_pg_type_as_pg_type_token_stream::#UpdateUpperCamelCaseSnippet5>,
            ) -> #self_pg_type_as_pg_type_token_stream::#ReadUpperCamelCaseSnippet5 {
                #previous_read_and_optional_update_into_read_token_stream
            }
        }
        .into()
    };
    let read_ids_and_create_into_read_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_6 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (
            CreateSnakeCaseSnippet6,
            CreateUpperCamelCaseSnippet6,
            ReadIdsAndCreateIntoReadSnakeCaseSnippet6,
            ReadIdsSnakeCaseSnippet6,
            ReadIdsUpperCamelCaseSnippet6,
            ReadUpperCamelCaseSnippet6,
        ) = (
            snippet_names_6.get_create_snake_case(),
            snippet_names_6.get_create_upper_camel_case(),
            snippet_names_6.get_read_ids_and_create_into_read_snake_case(),
            snippet_names_6.get_read_ids_snake_case(),
            snippet_names_6.get_read_ids_upper_camel_case(),
            snippet_names_6.get_read_upper_camel_case(),
        );
        quote::quote! {
            fn #ReadIdsAndCreateIntoReadSnakeCaseSnippet6(
                #ReadIdsSnakeCaseSnippet6: #self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCaseSnippet6,
                #CreateSnakeCaseSnippet6: #self_pg_type_as_pg_type_token_stream::#CreateUpperCamelCaseSnippet6
            ) -> #self_pg_type_as_pg_type_token_stream::#ReadUpperCamelCaseSnippet6 {
                #read_ids_and_create_into_read_token_stream
            }
        }
        .into()
    };
    let read_ids_and_create_into_optional_v_read_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_7 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (
            CreateSnakeCaseSnippet7,
            CreateUpperCamelCaseSnippet7,
            ReadIdsAndCreateIntoOptionalVReadSnakeCaseSnippet7,
            ReadIdsSnakeCaseSnippet7,
            ReadIdsUpperCamelCaseSnippet7,
            ReadUpperCamelCaseSnippet7,
            VUpperCamelCaseSnippet7,
        ) = (
            snippet_names_7.get_create_snake_case(),
            snippet_names_7.get_create_upper_camel_case(),
            snippet_names_7.get_read_ids_and_create_into_optional_v_read_snake_case(),
            snippet_names_7.get_read_ids_snake_case(),
            snippet_names_7.get_read_ids_upper_camel_case(),
            snippet_names_7.get_read_upper_camel_case(),
            snippet_names_7.get_v_upper_camel_case(),
        );
        quote::quote! {
            fn #ReadIdsAndCreateIntoOptionalVReadSnakeCaseSnippet7(
                #ReadIdsSnakeCaseSnippet7: #self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCaseSnippet7,
                #CreateSnakeCaseSnippet7: #self_pg_type_as_pg_type_token_stream::#CreateUpperCamelCaseSnippet7
            ) -> Option<#import::#VUpperCamelCaseSnippet7<#self_pg_type_as_pg_type_token_stream::#ReadUpperCamelCaseSnippet7>> {
                #read_ids_and_create_into_optional_v_read_token_stream
            }
        }
        .into()
    };
    let read_ids_and_create_into_table_type_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_8 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (
            CreateSnakeCaseSnippet8,
            CreateUpperCamelCaseSnippet8,
            ReadIdsAndCreateIntoTableTypeSnakeCaseSnippet8,
            ReadIdsSnakeCaseSnippet8,
            ReadIdsUpperCamelCaseSnippet8,
            TableTypeUpperCamelCaseSnippet8,
        ) = (
            snippet_names_8.get_create_snake_case(),
            snippet_names_8.get_create_upper_camel_case(),
            snippet_names_8.get_read_ids_and_create_into_table_type_snake_case(),
            snippet_names_8.get_read_ids_snake_case(),
            snippet_names_8.get_read_ids_upper_camel_case(),
            snippet_names_8.get_table_type_upper_camel_case(),
        );
        quote::quote! {
            fn #ReadIdsAndCreateIntoTableTypeSnakeCaseSnippet8(
                #ReadIdsSnakeCaseSnippet8: #self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCaseSnippet8,
                #CreateSnakeCaseSnippet8: #self_pg_type_as_pg_type_token_stream::#CreateUpperCamelCaseSnippet8
            ) -> #self_pg_type_as_pg_type_token_stream::#TableTypeUpperCamelCaseSnippet8 {
                #read_ids_and_create_into_table_type_token_stream
            }
        }
        .into()
    };
    let read_ids_and_create_into_where_eq_token_stream_gnrtd =
        super::generate_read_ids_and_create_into_where_eq_token_stream::generate_read_ids_and_create_into_where_eq_token_stream(
            &self_pg_type_as_pg_type_read_ids_token_stream,
            &self_pg_type_as_pg_type_create_token_stream,
            &self_pg_type_as_pg_type_where_token_stream,
            &read_ids_and_create_into_where_eq_token_stream,
        );
    let read_ids_and_create_into_vec_where_eq_using_fields_token_stream_gnrtd =
        super::generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream::generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream(
            import,
            &self_pg_type_as_pg_type_read_ids_token_stream,
            &self_pg_type_as_pg_type_create_token_stream,
            &self_pg_type_as_pg_type_where_token_stream,
            &read_ids_and_create_into_vec_where_eq_using_fields_token_stream,
        );
    let read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream_gnrtd: Option<
        macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    > =
        read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream.map(|ts| {
            {
    let snippet_names_9 = crate::domain_types::token_emission::NamesCtx::new();
        #[allow(non_snake_case, reason = "generated Rust identifiers intentionally mirror emitted naming tokens")]
        let (CreateSnakeCaseSnippet9, ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCaseSnippet9, ReadIdsSnakeCaseSnippet9) = (
            snippet_names_9.get_create_snake_case(),
            snippet_names_9.get_read_ids_and_create_into_optional_vec_where_eq_to_field_snake_case(),
            snippet_names_9.get_read_ids_snake_case(),
        );
        let return_type_token_stream =
            crate::domain_types::generate_optional_type_declaration_token_stream(
                &quote::quote! {#import::NotEmptyUniqueVec<#self_pg_type_as_pg_type_where_token_stream>},
            );
        quote::quote! {
            fn #ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCaseSnippet9(
                #ReadIdsSnakeCaseSnippet9: #self_pg_type_as_pg_type_read_ids_token_stream,
                #CreateSnakeCaseSnippet9: #self_pg_type_as_pg_type_create_token_stream
            ) -> #return_type_token_stream {
                #ts
            }
        }
        .into()
}
        });
    let pg_type_optional_vec_where_greater_than_test_token_stream_gnrtd =
        pg_type_optional_vec_where_greater_than_test_token_stream.map(|ts| {
            quote::quote! {
                fn #PgTypeOptionalVecWhereGreaterThanTestSnakeCase() -> Option<
                    #import::NotEmptyUniqueVec<
                        #import::PgTypeGreaterThanTest<
                            #SelfUpperCamelCase::#PgTypeUpperCamelCase
                        >
                    >
                > {
                    #ts
                }
            }
        });
    let read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream_gnrtd =
        read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream.map(|ts| {
            let read_ids_and_table_type_into_pg_type_optional_where_greater_than_snake_case =
                ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase;
            quote::quote! {
                fn #read_ids_and_table_type_into_pg_type_optional_where_greater_than_snake_case(
                    greater_than_variant: #import::PgTypeGreaterThanVariant,
                    #ReadIdsSnakeCase: #self_pg_type_as_pg_type_token_stream::#ReadIdsUpperCamelCase,
                    #TableTypeSnakeCase: #self_pg_type_as_pg_type_token_stream::#TableTypeUpperCamelCase,
                ) -> Option<#self_pg_type_as_pg_type_token_stream::#WhereUpperCamelCase> {
                    #ts
                }
            }
        });
    quote::quote! {
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(unused_qualifications)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_token_stream
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgTypeTestCasesUpperCamelCase for #identifier {
            type #PgTypeUpperCamelCase = #SelfUpperCamelCase;
            type #SelectUpperCamelCase = #identifier_select_upper_camel_case;
            #optional_vec_create_token_stream_gnrtd
            #read_ids_to_2_dimensions_vec_read_inner_token_stream_gnrtd
            #read_inner_into_read_with_new_or_try_new_unwraped_token_stream_gnrtd
            #read_inner_into_update_with_new_or_try_new_unwraped_token_stream_gnrtd
            #update_to_read_ids_token_stream_gnrtd
            #read_ids_to_optional_v_read_default_some_one_element_token_stream_gnrtd
            #previous_read_and_optional_update_into_read_token_stream_gnrtd
            #read_ids_and_create_into_read_token_stream_gnrtd
            #read_ids_and_create_into_optional_v_read_token_stream_gnrtd
            #read_ids_and_create_into_table_type_token_stream_gnrtd
            #read_ids_and_create_into_where_eq_token_stream_gnrtd
            #read_ids_and_create_into_vec_where_eq_using_fields_token_stream_gnrtd
            #read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream_gnrtd
            #pg_type_optional_vec_where_greater_than_test_token_stream_gnrtd
            #read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream_gnrtd
        }
    }
    .into()
}
