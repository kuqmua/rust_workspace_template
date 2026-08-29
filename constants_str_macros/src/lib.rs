mod collection_max_len;
mod constant;
mod constant_part;
mod constant_parts;
mod constants;
mod define_str_constants_input;
mod fragment;
mod fragments;
mod syn_ident;
mod syn_lit_str;
mod syn_visibility;

mod keyword {
    syn::custom_keyword!(constants);
    syn::custom_keyword!(fragments);
}

#[proc_macro]
pub fn define_str_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<define_str_constants_input::DefineStrConstantsInput>(input) {
        Ok(parsed) => proc_macro::TokenStream::from(proc_macro2::TokenStream::from(parsed)),
        Err(error) => proc_macro::TokenStream::from(error.into_compile_error()),
    }
}
