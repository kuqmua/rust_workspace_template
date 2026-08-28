#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use crate::domain_types::*;

pub fn generate_impl_pg_type_not_primary_key_for_identifier_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let identifier_create_upper_camel_case =
        naming::domain_types::parameter::SelfCreateUpperCamelCase::from_tokens(&identifier);
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let pg_type_not_primary_key_upper_camel_case =
        naming::domain_types::PgTypeNotPrimaryKeyUpperCamelCase;
    let pg_type_upper_camel_case = naming::domain_types::PgTypeUpperCamelCase;
    let create_upper_camel_case = naming::domain_types::CreateUpperCamelCase;
    quote::quote! {
        #allow_clippy_arbitrary_src_item_ordering
        impl #import::#pg_type_not_primary_key_upper_camel_case for #identifier {
            type #pg_type_upper_camel_case = Self;
            type #create_upper_camel_case = #identifier_create_upper_camel_case;
        }
    }
    .into()
}
