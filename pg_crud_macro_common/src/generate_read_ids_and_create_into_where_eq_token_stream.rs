pub fn generate_read_ids_and_create_into_where_eq_token_stream(
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
