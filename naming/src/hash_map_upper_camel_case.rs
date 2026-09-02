#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub struct HashMapUpperCamelCase;

impl std::fmt::Display for HashMapUpperCamelCase {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "HashMap")
    }
}

impl quote::ToTokens for HashMapUpperCamelCase {
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        quote::quote! {HashMap}.to_tokens(token_stream);
    }
}
