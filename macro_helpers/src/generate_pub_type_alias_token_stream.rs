pub fn generate_pub_type_alias_token_stream(
    alias_type_name_token_stream: &dyn quote::ToTokens,
    alias_actual_type_name_token_stream: &dyn quote::ToTokens,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {pub type #alias_type_name_token_stream = #alias_actual_type_name_token_stream;}
        .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn generated_alias_preserves_generic_actual_type() {
        let actual =
            crate::generate_pub_type_alias_token_stream::generate_pub_type_alias_token_stream(
                &quote::quote!(Items),
                &quote::quote!(Vec<Option<u8>>),
            );
        assert_eq!(
            actual.as_ref().to_string(),
            quote::quote!(
                pub type Items = Vec<Option<u8>>;
            )
            .to_string()
        );
    }
}
