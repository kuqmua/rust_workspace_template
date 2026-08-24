#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct SynLocationField(syn::Field);
#[must_use]
pub fn location_syn_field() -> SynLocationField {
    SynLocationField::from(syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        modifiers: syn::FieldModifiers::default(),
        ident: Some(syn::Ident::new(
            str_constants::LOCATION_ALT,
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
