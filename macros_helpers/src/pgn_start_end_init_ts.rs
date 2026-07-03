#[must_use]
pub fn pgn_start_end_init_ts<ValueTokenStream>(value: &ValueTokenStream) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        let start = #value.offset;
        let end = start.saturating_add(#value.limit);
    }
}
