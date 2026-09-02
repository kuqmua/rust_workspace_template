#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::FromInner,
)]
pub struct ParseTokenStreamStrings(Vec<String>);
impl ParseTokenStreamStrings {
    #[must_use]
    pub fn into_generated_vec(
        self,
        parse_error_id_ref: crate::parse_error_id_ref::ParseErrorIdRef<'_>,
    ) -> crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec
    {
        self.0
            .into_iter()
            .map(
                |element| match element.as_str().parse::<proc_macro2::TokenStream>() {
                    Ok(parsed_token_stream) => parsed_token_stream.into(),
                    Err(error) => {
                        let message = format!("{}: {error}", parse_error_id_ref.as_ref());
                        quote::quote! {compile_error!(#message);}.into()
                    }
                },
            )
            .collect::<crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec>()
    }
}
