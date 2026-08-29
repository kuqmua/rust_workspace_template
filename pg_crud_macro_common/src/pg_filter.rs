pub trait PgFilter {
    fn maybe_generic(
        &self,
    ) -> Option<
        macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    >;

    fn prefix_where_self_upper_camel_case(
        &self,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream;

    fn ucc(&self) -> &'static dyn naming::display_plus_to_tokens::DisplayPlusToTokens;
}
