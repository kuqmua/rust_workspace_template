mod collection_max_len;
mod constant;
mod constant_part;
mod constant_parts;
mod constants;
mod define_str_constants_input;
mod domain_types;
mod fragment;
mod fragments;
mod syn_ident;
mod syn_lit_str;
mod syn_visibility;

mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
}

pub(crate) use collection_max_len::COLLECTION_MAX_LEN;
pub(crate) use constant::Constant;
pub(crate) use constant_part::ConstantPart;
pub(crate) use constant_parts::ConstantParts;
pub(crate) use constants::Constants;
pub(crate) use fragment::Fragment;
pub(crate) use fragments::Fragments;
pub(crate) use syn_ident::SynIdent;
pub(crate) use syn_lit_str::SynLitStr;
pub(crate) use syn_visibility::SynVisibility;

#[proc_macro]
pub fn define_str_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<domain_types::DefineStrConstantsInput>(input) {
        Ok(parsed) => proc_macro::TokenStream::from(proc_macro2::TokenStream::from(parsed)),
        Err(error) => proc_macro::TokenStream::from(error.into_compile_error()),
    }
}
