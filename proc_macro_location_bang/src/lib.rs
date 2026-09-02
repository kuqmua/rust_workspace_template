#[proc_macro]
pub fn location(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    drop(token_stream);
    quote::quote! {
        location_lib::location::Location::new(
            file!(),
            location_lib::location_line::LocationLine::from(
                std::num::NonZeroU32::new(line!()).unwrap_or(std::num::NonZeroU32::MIN),
            ),
            location_lib::location_column::LocationColumn::from(
                std::num::NonZeroU32::new(column!()).unwrap_or(std::num::NonZeroU32::MIN),
            ),
            None,
        )
    }
    .into()
}
