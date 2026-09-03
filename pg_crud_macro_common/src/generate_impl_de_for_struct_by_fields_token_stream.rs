#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

#[allow(
    unused_variables,
    reason = "the generator API preserves the repository type-based parameter name"
)]
pub fn generate_impl_de_for_struct_by_fields_token_stream(
    identifier: &dyn naming::display_plus_to_tokens::DisplayPlusToTokens,
    syn_field_refs: crate::syn_field_refs::SynFieldRefs<'_>,
    de_len: crate::de_len::DeLen,
    generate_type_token_stream: &dyn Fn(
        &syn::Ident,
        &syn::Type,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let raw_identifier_token_stream = quote::format_ident!("{}Raw", identifier.to_string());
    let syn_fields: &[macro_helpers::syn_field::SynField] = syn_field_refs.into();
    let raw_fields_token_stream = syn_fields.iter().map(|field| {
        let ty = field.get_field_type().as_ref();
        let field_identifier = field.get_identifier().as_ref();
        let type_token_stream = generate_type_token_stream(field_identifier, ty);
        quote::quote! { #field_identifier: #type_token_stream, }
    });
    let try_from_fields_token_stream = syn_fields.iter().map(|field| {
        let field_identifier = field.get_identifier().as_ref();
        quote::quote! { raw.#field_identifier }
    });
    quote::quote! {
        #[derive(serde::Deserialize)]

        #[allow(clippy::arbitrary_source_item_ordering, reason = "generate impl de for struct by fields token stream keeps declaration order aligned with generated layout or processing flow")]
        struct #raw_identifier_token_stream {
            #(#raw_fields_token_stream)*
        }

        #[allow(unused_qualifications, reason = "generate impl de for struct by fields token stream keeps declaration order aligned with generated layout or processing flow")]

        #[allow(clippy::absolute_paths, reason = "generate impl de for struct by fields token stream keeps declaration order aligned with generated layout or processing flow")]
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
