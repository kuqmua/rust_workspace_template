mod domain_types;

#[proc_macro]
pub fn define_str_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(domain_types::expand(
        domain_types::ProcMacroDefineStrConstantsInput::from(input),
    ))
}
