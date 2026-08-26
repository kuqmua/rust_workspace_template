pub fn pagination_start_end_initialization_token_stream(
    v: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    quote::quote! {
        let start = #v.pagination.start();
        let end = #v.pagination.end();
    }
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn generated_pagination_initialization_preserves_receiver() {
        let actual =
            super::pagination_start_end_initialization_token_stream(&quote::quote!(parameters));
        let expected = quote::quote! {
            let start = parameters.pagination.start();
            let end = parameters.pagination.end();
        };
        assert_eq!(actual.as_ref().to_string(), expected.to_string());
    }
}
