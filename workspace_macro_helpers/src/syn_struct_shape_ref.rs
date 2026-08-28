use crate::domain_types::{SynFieldsNamedRef, SynFieldsUnnamedRef};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub enum SynStructShapeRef<'shape_lt> {
    Named(SynFieldsNamedRef<'shape_lt>),
    Tuple(SynFieldsUnnamedRef<'shape_lt>),
    Unit,
}
impl<'shape_lt> TryFrom<&'shape_lt syn::DeriveInput> for SynStructShapeRef<'shape_lt> {
    type Error = syn::Error;
    fn try_from(value: &'shape_lt syn::DeriveInput) -> Result<Self, Self::Error> {
        let syn::Data::Struct(data) = &value.data else {
            return Err(syn::Error::new_spanned(
                value,
                constants_str::EXPECTED_A_STRUCT,
            ));
        };
        Ok(match &data.fields {
            syn::Fields::Named(fields) => Self::Named(SynFieldsNamedRef(fields)),
            syn::Fields::Unnamed(fields) => Self::Tuple(SynFieldsUnnamedRef(fields)),
            syn::Fields::Unit => Self::Unit,
        })
    }
}
