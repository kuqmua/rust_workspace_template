#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

#[allow(
    unused_variables,
    reason = "the generator API preserves the repository type-based parameter name"
)]
pub fn generate_impl_de_for_struct_token_stream(
    identifier: &dyn naming::display_plus_to_tokens::DisplayPlusToTokens,
    syn_identifier_type_refs: crate::syn_identifier_type_refs::SynIdentifierTypeRefs<'_>,
    de_len: crate::de_len::DeLen,
    generate_type_token_stream: &dyn Fn(
        &syn::Ident,
        &syn::Type,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let raw_identifier_token_stream = quote::format_ident!("{}Raw", identifier.to_string());
    let identifier_types: &[(&syn::Ident, &syn::Type)] = syn_identifier_type_refs.into();
    let raw_fields_token_stream = identifier_types.iter().map(|(field, ty)| {
        let type_token_stream = generate_type_token_stream(field, ty);
        quote::quote! { #field: #type_token_stream, }
    });
    let try_from_fields_token_stream = identifier_types.iter().map(|(field, _)| {
        quote::quote! { raw.#field }
    });
    quote::quote! {
        #[derive(serde::Deserialize)]

        #[allow(clippy::arbitrary_source_item_ordering, reason = "generate impl de for struct token stream keeps declaration order aligned with generated layout or processing flow")]
        struct #raw_identifier_token_stream {
            #(#raw_fields_token_stream)*
        }

        #[allow(unused_qualifications, reason = "generate impl de for struct token stream keeps declaration order aligned with generated layout or processing flow")]

        #[allow(clippy::absolute_paths, reason = "generate impl de for struct token stream keeps declaration order aligned with generated layout or processing flow")]
        #allow_clippy_arbitrary_src_item_ordering
        const _: () = {
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #identifier {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let raw = <#raw_identifier_token_stream as _serde::Deserialize>::deserialize(__deserializer)?;
                    Self::try_new(#(#try_from_fields_token_stream),*).map_err(|error| _serde::de::Error::custom(format!("{error:?}")))
                }
            }
        };
    }.into()
}
