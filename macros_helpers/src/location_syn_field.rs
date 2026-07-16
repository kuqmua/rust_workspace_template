#[derive(Debug, newtype::IntoInnerFrom)]
pub struct SynLocationField(syn::Field);
#[must_use]
pub fn location_syn_field() -> SynLocationField {
    SynLocationField(syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new(
            str_constants::LOCATION_ALT,
            proc_macro2::Span::call_site(),
        )),
        colon_token: Some(syn::token::Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        ty: syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: crate::generate_simple_syn_punct::generate_simple_syn_punct([
                    str_constants::LOCATION_LIB,
                    str_constants::LOCATION_ALT,
                    str_constants::LOCATION,
                ])
                .into(),
            },
        }),
    })
}
