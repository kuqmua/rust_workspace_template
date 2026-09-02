#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub enum SynStructShapeRef<'shape_lt> {
    Named(crate::syn_fields_named_ref::SynFieldsNamedRef<'shape_lt>),
    Tuple(crate::syn_fields_unnamed_ref::SynFieldsUnnamedRef<'shape_lt>),
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
            syn::Fields::Named(fields) => {
                Self::Named(crate::syn_fields_named_ref::SynFieldsNamedRef::from(fields))
            }
            syn::Fields::Unnamed(fields) => Self::Tuple(
                crate::syn_fields_unnamed_ref::SynFieldsUnnamedRef::from(fields),
            ),
            syn::Fields::Unit => Self::Unit,
        })
    }
}
