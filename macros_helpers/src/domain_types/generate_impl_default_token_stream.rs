pub fn generate_impl_default_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        impl Default for #identifier {
            fn default() -> Self {
                #ts
            }
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn generated_default_impl_preserves_identifier_and_expression() {
        let actual = super::generate_impl_default_token_stream(
            &quote::quote!(Example),
            &quote::quote!(Self { value: 7u8 }),
        );
        let expected = quote::quote! {
            impl Default for Example {
                fn default() -> Self {
                    Self { value: 7u8 }
                }
            }
        };
        assert_eq!(actual.as_ref().to_string(), expected.to_string());
    }
}
