pub fn generate_read_ids_and_create_into_where_eq_token_stream(
    read_ids_token_stream: &dyn quote::ToTokens,
    create_token_stream: &dyn quote::ToTokens,
    where_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();
    #[allow(
        non_snake_case,
        reason = "generated Rust identifiers intentionally mirror emitted naming tokens"
    )]
    let (CreateSnakeCase, ReadIdsAndCreateIntoWhereEqSnakeCase, ReadIdsSnakeCase) = (
        names.get_create_snake_case(),
        names.get_read_ids_and_create_into_where_eq_snake_case(),
        names.get_read_ids_snake_case(),
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
