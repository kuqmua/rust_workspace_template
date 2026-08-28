#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[must_use]
pub fn location_syn_field() -> SynLocationField {
    SynLocationField::from(syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        modifiers: syn::FieldModifiers::default(),
        ident: Some(syn::Ident::new(
            constants_str::LOCATION_ALT,
            proc_macro2::Span::call_site(),
        )),
        colon_token: Some(syn::token::Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        default: None,
        ty: syn::Type::Path(syn::TypePath {
            attrs: Vec::new(),
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments:
                    crate::domain_types::generate_simple_syn_punct::generate_simple_syn_punct([
                        constants_str::LOCATION_LIB,
                        constants_str::DOMAIN_TYPES,
                        constants_str::LOCATION,
                    ])
                    .into(),
            },
        }),
    })
}
pub use crate::syn_location_field::SynLocationField;
