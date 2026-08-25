#[proc_macro]
pub fn location(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    drop(input);
    quote::quote! {
        location_lib::domain_types::Location::new(
            file!(),
            location_lib::domain_types::LocationLine::from(
                std::num::NonZeroU32::new(line!()).unwrap_or(std::num::NonZeroU32::MIN),
            ),
            location_lib::domain_types::LocationColumn::from(
                std::num::NonZeroU32::new(column!()).unwrap_or(std::num::NonZeroU32::MIN),
            ),
            None,
        )
    }
    .into()
}
