#[derive(Debug, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct SynLocationField(syn::Field);
#[must_use]
pub fn location_syn_field() -> SynLocationField {
    SynLocationField(syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new("location", proc_macro2::Span::call_site())),
        colon_token: Some(syn::token::Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        ty: syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: crate::generate_simple_syn_punct::generate_simple_syn_punct([
                    "location_lib",
                    "location",
                    "Location",
                ])
                .into(),
            },
        }),
    })
}
