#[derive(Debug, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct SynLocationField(syn::Field);
#[must_use]
pub fn location_syn_field() -> SynLocationField {
    SynLocationField(syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new(
            str_constants::expr::S_1463,
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
                    str_constants::expr::S_1465,
                    str_constants::expr::S_1463,
                    str_constants::expr::S_0699,
                ])
                .into(),
            },
        }),
    })
}
