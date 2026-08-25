#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_optional_vec_create_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CreateUpperCamelCase, OptionalVecCreateSnakeCase) = (
        &names.CreateUpperCamelCase,
        &names.OptionalVecCreateSnakeCase,
    );
    quote::quote! {
        fn #OptionalVecCreateSnakeCase() -> Option<Vec<#path_token_stream::#CreateUpperCamelCase>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_read_ids_to_2_dimensions_vec_read_inner_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        ReadIdsSnakeCase,
        ReadIdsTo2DimensionsVecReadInnerSnakeCase,
        ReadIdsUpperCamelCase,
        ReadInnerUpperCamelCase,
    ) = (
        &names.ReadIdsSnakeCase,
        &names.ReadIdsTo2DimensionsVecReadInnerSnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.ReadInnerUpperCamelCase,
    );
    quote::quote! {
        fn #ReadIdsTo2DimensionsVecReadInnerSnakeCase(
            #ReadIdsSnakeCase: &#path_token_stream::#ReadIdsUpperCamelCase
        ) -> Vec<Vec<#path_token_stream::#ReadInnerUpperCamelCase>> {
            #ts
        }
    }
    .into()
}
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
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_update_to_read_ids_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (ReadIdsUpperCamelCase, UpdateToReadIdsSnakeCase, UpdateUpperCamelCase, VSnakeCase) = (
        &names.ReadIdsUpperCamelCase,
        &names.UpdateToReadIdsSnakeCase,
        &names.UpdateUpperCamelCase,
        &names.VSnakeCase,
    );
    quote::quote! {
        fn #UpdateToReadIdsSnakeCase(
            #VSnakeCase: &#path_token_stream::#UpdateUpperCamelCase
        ) -> #path_token_stream::#ReadIdsUpperCamelCase {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_read_ids_to_optional_v_read_default_some_one_element_token_stream(
    import: crate::domain_types::Import,
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
        ReadIdsUpperCamelCase,
        ReadUpperCamelCase,
        VSnakeCase,
        VUpperCamelCase,
    ) = (
        &names.ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.ReadUpperCamelCase,
        &names.VSnakeCase,
        &names.VUpperCamelCase,
    );
    quote::quote! {
        fn #ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase(
            #VSnakeCase: &#path_token_stream::#ReadIdsUpperCamelCase
        ) -> Option<#import::#VUpperCamelCase<#path_token_stream::#ReadUpperCamelCase>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_previous_read_and_optional_update_into_read_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        OptionalUpdateSnakeCase,
        PreviousReadAndOptionalUpdateIntoReadSnakeCase,
        ReadSnakeCase,
        ReadUpperCamelCase,
        UpdateUpperCamelCase,
    ) = (
        &names.OptionalUpdateSnakeCase,
        &names.PreviousReadAndOptionalUpdateIntoReadSnakeCase,
        &names.ReadSnakeCase,
        &names.ReadUpperCamelCase,
        &names.UpdateUpperCamelCase,
    );
    quote::quote! {
        fn #PreviousReadAndOptionalUpdateIntoReadSnakeCase(
            #ReadSnakeCase: #path_token_stream::#ReadUpperCamelCase,
            #OptionalUpdateSnakeCase: Option<#path_token_stream::#UpdateUpperCamelCase>,
        ) -> #path_token_stream::#ReadUpperCamelCase {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_read_ids_and_create_into_read_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        CreateSnakeCase,
        CreateUpperCamelCase,
        ReadIdsAndCreateIntoReadSnakeCase,
        ReadIdsSnakeCase,
        ReadIdsUpperCamelCase,
        ReadUpperCamelCase,
    ) = (
        &names.CreateSnakeCase,
        &names.CreateUpperCamelCase,
        &names.ReadIdsAndCreateIntoReadSnakeCase,
        &names.ReadIdsSnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.ReadUpperCamelCase,
    );
    quote::quote! {
        fn #ReadIdsAndCreateIntoReadSnakeCase(
            #ReadIdsSnakeCase: #path_token_stream::#ReadIdsUpperCamelCase,
            #CreateSnakeCase: #path_token_stream::#CreateUpperCamelCase
        ) -> #path_token_stream::#ReadUpperCamelCase {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_read_ids_and_create_into_optional_v_read_token_stream(
    import: crate::domain_types::Import,
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        CreateSnakeCase,
        CreateUpperCamelCase,
        ReadIdsAndCreateIntoOptionalVReadSnakeCase,
        ReadIdsSnakeCase,
        ReadIdsUpperCamelCase,
        ReadUpperCamelCase,
        VUpperCamelCase,
    ) = (
        &names.CreateSnakeCase,
        &names.CreateUpperCamelCase,
        &names.ReadIdsAndCreateIntoOptionalVReadSnakeCase,
        &names.ReadIdsSnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.ReadUpperCamelCase,
        &names.VUpperCamelCase,
    );
    quote::quote! {
        fn #ReadIdsAndCreateIntoOptionalVReadSnakeCase(
            #ReadIdsSnakeCase: #path_token_stream::#ReadIdsUpperCamelCase,
            #CreateSnakeCase: #path_token_stream::#CreateUpperCamelCase
        ) -> Option<#import::#VUpperCamelCase<#path_token_stream::#ReadUpperCamelCase>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_read_ids_and_create_into_table_type_token_stream(
    path_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        CreateSnakeCase,
        CreateUpperCamelCase,
        ReadIdsAndCreateIntoTableTypeSnakeCase,
        ReadIdsSnakeCase,
        ReadIdsUpperCamelCase,
        TableTypeUpperCamelCase,
    ) = (
        &names.CreateSnakeCase,
        &names.CreateUpperCamelCase,
        &names.ReadIdsAndCreateIntoTableTypeSnakeCase,
        &names.ReadIdsSnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.TableTypeUpperCamelCase,
    );
    quote::quote! {
        fn #ReadIdsAndCreateIntoTableTypeSnakeCase(
            #ReadIdsSnakeCase: #path_token_stream::#ReadIdsUpperCamelCase,
            #CreateSnakeCase: #path_token_stream::#CreateUpperCamelCase
        ) -> #path_token_stream::#TableTypeUpperCamelCase {
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
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn generate_read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream(
    import: crate::domain_types::Import,
    read_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let names = crate::domain_types::NamesCtx::new();
    #[allow(non_snake_case)]
    let (CreateSnakeCase, ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase, ReadIdsSnakeCase) = (
        &names.CreateSnakeCase,
        &names.ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase,
        &names.ReadIdsSnakeCase,
    );
    let return_type_token_stream =
        crate::domain_types::generate_optional_type_declaration_token_stream(
            &quote::quote! {#import::NotEmptyUniqueVec<#where_token_stream>},
        );
    quote::quote! {
        fn #ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase(
            #ReadIdsSnakeCase: #read_ids_token_stream,
            #CreateSnakeCase: #create_token_stream
        ) -> #return_type_token_stream {
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
    let optional_vec_create_token_stream_gnrtd = optional_vec_create_token_stream.map(|ts| {
        generate_optional_vec_create_token_stream(&self_pg_type_as_pg_type_token_stream, ts)
    });
    let read_ids_to_2_dimensions_vec_read_inner_token_stream_gnrtd =
        generate_read_ids_to_2_dimensions_vec_read_inner_token_stream(
            &self_pg_type_as_pg_type_token_stream,
            &read_ids_to_2_dimensions_vec_read_inner_token_stream,
        );
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
    let update_to_read_ids_token_stream_gnrtd = generate_update_to_read_ids_token_stream(
        &self_pg_type_as_pg_type_token_stream,
        &update_to_read_ids_token_stream,
    );
    let read_ids_to_optional_v_read_default_some_one_element_token_stream_gnrtd =
        generate_read_ids_to_optional_v_read_default_some_one_element_token_stream(
            *import,
            &self_pg_type_as_pg_type_token_stream,
            &read_ids_to_optional_v_read_default_some_one_element_token_stream,
        );
    let previous_read_and_optional_update_into_read_token_stream_gnrtd =
        generate_previous_read_and_optional_update_into_read_token_stream(
            &self_pg_type_as_pg_type_token_stream,
            &previous_read_and_optional_update_into_read_token_stream,
        );
    let read_ids_and_create_into_read_token_stream_gnrtd =
        generate_read_ids_and_create_into_read_token_stream(
            &self_pg_type_as_pg_type_token_stream,
            &read_ids_and_create_into_read_token_stream,
        );
    let read_ids_and_create_into_optional_v_read_token_stream_gnrtd =
        generate_read_ids_and_create_into_optional_v_read_token_stream(
            *import,
            &self_pg_type_as_pg_type_token_stream,
            &read_ids_and_create_into_optional_v_read_token_stream,
        );
    let read_ids_and_create_into_table_type_token_stream_gnrtd =
        generate_read_ids_and_create_into_table_type_token_stream(
            &self_pg_type_as_pg_type_token_stream,
            &read_ids_and_create_into_table_type_token_stream,
        );
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
    let read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream_gnrtd =
        read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream.map(|ts| {
            generate_read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream(
                *import,
                &self_pg_type_as_pg_type_read_ids_token_stream,
                &self_pg_type_as_pg_type_create_token_stream,
                &self_pg_type_as_pg_type_where_token_stream,
                ts,
            )
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
