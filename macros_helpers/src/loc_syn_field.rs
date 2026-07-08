#[derive(Debug)]
pub struct LocSynField(syn::Field);
impl From<LocSynField> for syn::Field {
    fn from(value: LocSynField) -> Self {
        value.0
    }
}
#[must_use]
pub fn loc_syn_field() -> LocSynField {
    LocSynField(syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new("loc", proc_macro2::Span::call_site())),
        colon_token: Some(syn::token::Colon {
            spans: [proc_macro2::Span::call_site()],
        }),
        ty: syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: crate::gen_simple_syn_punct::gen_simple_syn_punct([
                    "loc_lib", "loc", "Loc",
                ])
                .into(),
            },
        }),
    })
}
