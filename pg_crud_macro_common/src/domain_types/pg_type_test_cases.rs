fn generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream(
    method_name_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    path_token_stream: &dyn quote::ToTokens,
    return_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (VSnakeCase,) = (&names.VSnakeCase,);
    quote::quote! {
        fn #method_name_token_stream(
            #VSnakeCase: #type_token_stream
        ) -> #path_token_stream::#return_type_token_stream {
            #ts
        }
    }
    .into()
}
pub fn generate_read_ids_and_create_into_where_eq_token_stream(
    read_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CreateSnakeCase, ReadIdsAndCreateIntoWhereEqSnakeCase, ReadIdsSnakeCase) = (
        &names.CreateSnakeCase,
        &names.ReadIdsAndCreateIntoWhereEqSnakeCase,
        &names.ReadIdsSnakeCase,
    );
    quote::quote! {
        fn #ReadIdsAndCreateIntoWhereEqSnakeCase(
            #ReadIdsSnakeCase: #read_ids_token_stream,
            #CreateSnakeCase: #create_token_stream
        ) -> #where_token_stream {
            #ts
        }
    }
    .into()
}
pub fn generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream(
    import: &crate::domain_types::Import,
    read_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CreateSnakeCase, ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase, ReadIdsSnakeCase) = (
        &names.CreateSnakeCase,
        &names.ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase,
        &names.ReadIdsSnakeCase,
    );
    quote::quote! {
        fn #ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase(
            #ReadIdsSnakeCase: #read_ids_token_stream,
            #CreateSnakeCase: #create_token_stream
        ) -> #import::NotEmptyUniqueVec<#where_token_stream> {
            #ts
        }
    }
    .into()
}
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
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
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
        &names.AllowClippyArbitrarySrcItemOrdering,
        &names.CreateUpperCamelCase,
        &names.PgTypeOptionalVecWhereGreaterThanTestSnakeCase,
        &names.PgTypeTestCasesUpperCamelCase,
        &names.PgTypeUpperCamelCase,
        &names.ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase,
        &names.ReadIdsSnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
        &names.ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
        &names.SelectUpperCamelCase,
        &names.SelfUpperCamelCase,
        &names.TableTypeSnakeCase,
        &names.TableTypeUpperCamelCase,
        &names.WhereUpperCamelCase,
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
        macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > = optional_vec_create_token_stream.map(|ts| {
        {
    let snippet_names_1 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (CreateUpperCamelCaseSnippet1, OptionalVecCreateSnakeCaseSnippet1) = (
            &snippet_names_1.CreateUpperCamelCase,
            &snippet_names_1.OptionalVecCreateSnakeCase,
        );
        quote::quote! {
            fn #OptionalVecCreateSnakeCaseSnippet1() -> Option<Vec<#self_pg_type_as_pg_type_token_stream::#CreateUpperCamelCaseSnippet1>> {
                #ts
            }
        }
        .into()
}
    });
    let read_ids_to_2_dimensions_vec_read_inner_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_2 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (
            ReadIdsSnakeCaseSnippet2,
            ReadIdsTo2DimensionsVecReadInnerSnakeCaseSnippet2,
            ReadIdsUpperCamelCaseSnippet2,
            ReadInnerUpperCamelCaseSnippet2,
        ) = (
            &snippet_names_2.ReadIdsSnakeCase,
            &snippet_names_2.ReadIdsTo2DimensionsVecReadInnerSnakeCase,
            &snippet_names_2.ReadIdsUpperCamelCase,
            &snippet_names_2.ReadInnerUpperCamelCase,
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
        generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream(
            &ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
            &type_token_stream,
            &self_pg_type_as_pg_type_token_stream,
            &naming::domain_types::ReadUpperCamelCase,
            &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_token_stream_gnrtd =
        generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream(
            &ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
            &type_token_stream,
            &self_pg_type_as_pg_type_token_stream,
            &naming::domain_types::UpdateUpperCamelCase,
            &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
        );
    let update_to_read_ids_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_3 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (ReadIdsUpperCamelCaseSnippet3, UpdateToReadIdsSnakeCaseSnippet3, UpdateUpperCamelCaseSnippet3, VSnakeCaseSnippet3) = (
            &snippet_names_3.ReadIdsUpperCamelCase,
            &snippet_names_3.UpdateToReadIdsSnakeCase,
            &snippet_names_3.UpdateUpperCamelCase,
            &snippet_names_3.VSnakeCase,
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
    let read_ids_to_optional_v_read_default_some_one_element_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_4 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (
            ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCaseSnippet4,
            ReadIdsUpperCamelCaseSnippet4,
            ReadUpperCamelCaseSnippet4,
            VSnakeCaseSnippet4,
            VUpperCamelCaseSnippet4,
        ) = (
            &snippet_names_4.ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
            &snippet_names_4.ReadIdsUpperCamelCase,
            &snippet_names_4.ReadUpperCamelCase,
            &snippet_names_4.VSnakeCase,
            &snippet_names_4.VUpperCamelCase,
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
    let previous_read_and_optional_update_into_read_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_5 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (
            OptionalUpdateSnakeCaseSnippet5,
            PreviousReadAndOptionalUpdateIntoReadSnakeCaseSnippet5,
            ReadSnakeCaseSnippet5,
            ReadUpperCamelCaseSnippet5,
            UpdateUpperCamelCaseSnippet5,
        ) = (
            &snippet_names_5.OptionalUpdateSnakeCase,
            &snippet_names_5.PreviousReadAndOptionalUpdateIntoReadSnakeCase,
            &snippet_names_5.ReadSnakeCase,
            &snippet_names_5.ReadUpperCamelCase,
            &snippet_names_5.UpdateUpperCamelCase,
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
    let read_ids_and_create_into_read_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_6 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (
            CreateSnakeCaseSnippet6,
            CreateUpperCamelCaseSnippet6,
            ReadIdsAndCreateIntoReadSnakeCaseSnippet6,
            ReadIdsSnakeCaseSnippet6,
            ReadIdsUpperCamelCaseSnippet6,
            ReadUpperCamelCaseSnippet6,
        ) = (
            &snippet_names_6.CreateSnakeCase,
            &snippet_names_6.CreateUpperCamelCase,
            &snippet_names_6.ReadIdsAndCreateIntoReadSnakeCase,
            &snippet_names_6.ReadIdsSnakeCase,
            &snippet_names_6.ReadIdsUpperCamelCase,
            &snippet_names_6.ReadUpperCamelCase,
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
    let read_ids_and_create_into_optional_v_read_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_7 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (
            CreateSnakeCaseSnippet7,
            CreateUpperCamelCaseSnippet7,
            ReadIdsAndCreateIntoOptionalVReadSnakeCaseSnippet7,
            ReadIdsSnakeCaseSnippet7,
            ReadIdsUpperCamelCaseSnippet7,
            ReadUpperCamelCaseSnippet7,
            VUpperCamelCaseSnippet7,
        ) = (
            &snippet_names_7.CreateSnakeCase,
            &snippet_names_7.CreateUpperCamelCase,
            &snippet_names_7.ReadIdsAndCreateIntoOptionalVReadSnakeCase,
            &snippet_names_7.ReadIdsSnakeCase,
            &snippet_names_7.ReadIdsUpperCamelCase,
            &snippet_names_7.ReadUpperCamelCase,
            &snippet_names_7.VUpperCamelCase,
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
    let read_ids_and_create_into_table_type_token_stream_gnrtd: macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream = {
        let snippet_names_8 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (
            CreateSnakeCaseSnippet8,
            CreateUpperCamelCaseSnippet8,
            ReadIdsAndCreateIntoTableTypeSnakeCaseSnippet8,
            ReadIdsSnakeCaseSnippet8,
            ReadIdsUpperCamelCaseSnippet8,
            TableTypeUpperCamelCaseSnippet8,
        ) = (
            &snippet_names_8.CreateSnakeCase,
            &snippet_names_8.CreateUpperCamelCase,
            &snippet_names_8.ReadIdsAndCreateIntoTableTypeSnakeCase,
            &snippet_names_8.ReadIdsSnakeCase,
            &snippet_names_8.ReadIdsUpperCamelCase,
            &snippet_names_8.TableTypeUpperCamelCase,
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
        generate_read_ids_and_create_into_where_eq_token_stream(
            &self_pg_type_as_pg_type_read_ids_token_stream,
            &self_pg_type_as_pg_type_create_token_stream,
            &self_pg_type_as_pg_type_where_token_stream,
            &read_ids_and_create_into_where_eq_token_stream,
        );
    let read_ids_and_create_into_vec_where_eq_using_fields_token_stream_gnrtd =
        generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream(
            import,
            &self_pg_type_as_pg_type_read_ids_token_stream,
            &self_pg_type_as_pg_type_create_token_stream,
            &self_pg_type_as_pg_type_where_token_stream,
            &read_ids_and_create_into_vec_where_eq_using_fields_token_stream,
        );
    let read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream_gnrtd: Option<
        macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    > =
        read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream.map(|ts| {
            {
    let snippet_names_9 = crate::domain_types::NamesCtx::new();
        #[allow(non_snake_case)]
        let (CreateSnakeCaseSnippet9, ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCaseSnippet9, ReadIdsSnakeCaseSnippet9) = (
            &snippet_names_9.CreateSnakeCase,
            &snippet_names_9.ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase,
            &snippet_names_9.ReadIdsSnakeCase,
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
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_token_stream
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
