#[test]
fn test_table_pipeline_parse_errors_preserve_parser_diagnostics_without_emitting_source() {
    let cases = [
        proc_macro2::TokenStream::new(),
        quote::quote! { let table = 1; },
        quote::quote! { fn table() {} },
        quote::quote! { struct },
        quote::quote! { struct Table { id: } },
        quote::quote! { struct Table; struct Other; },
    ];
    assert!(cases.into_iter().all(|input| {
        let Err(expected_error) = syn::parse2::<syn::DeriveInput>(input.clone()) else {
            return false;
        };
        let Err(crate::generate_pg_table_pipeline_error::GeneratePgTablePipelineError::Parse(
            pipeline_error,
        )) = crate::parse_generate_pg_table::parse_generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        )
        else {
            return false;
        };
        let actual_error = syn::Error::from(pipeline_error);
        assert_eq!(actual_error.to_string(), expected_error.to_string());
        assert_eq!(
            actual_error.to_compile_error().to_string(),
            expected_error.to_compile_error().to_string(),
        );
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        assert_eq!(
            proc_macro2::TokenStream::from(generated).to_string(),
            expected_error.to_compile_error().to_string(),
        );
        true
    }));
}
