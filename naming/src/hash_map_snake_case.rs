#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub struct HashMapSnakeCase;

impl std::fmt::Display for HashMapSnakeCase {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "hashmap")
    }
}

impl quote::ToTokens for HashMapSnakeCase {
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        quote::quote! {hashmap}.to_tokens(token_stream);
    }
}
