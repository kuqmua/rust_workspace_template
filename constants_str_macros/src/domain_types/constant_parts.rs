#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ConstantParts(pub(super) Vec<super::ConstantPart>);

impl TryFrom<Vec<super::ConstantPart>> for ConstantParts {
    type Error = syn::Error;
    fn try_from(value: Vec<super::ConstantPart>) -> Result<Self, Self::Error> {
        if value.len() > super::COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(c93f714a too many constant parts),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
