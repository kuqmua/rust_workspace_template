pub fn generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream(
    import: &crate::domain_types::Import,
    read_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = crate::domain_types::token_emission::NamesCtx::new();
    #[allow(
        non_snake_case,
        reason = "generated Rust identifiers intentionally mirror emitted naming tokens"
    )]
    let (CreateSnakeCase, ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase, ReadIdsSnakeCase) = (
        names.get_create_snake_case(),
        names.get_read_ids_and_create_into_vec_where_eq_using_fields_snake_case(),
        names.get_read_ids_snake_case(),
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
