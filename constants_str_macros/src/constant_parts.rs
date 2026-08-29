#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ConstantParts(pub(super) Vec<crate::constant_part::ConstantPart>);

impl TryFrom<Vec<crate::constant_part::ConstantPart>> for ConstantParts {
    type Error = syn::Error;
    fn try_from(value: Vec<crate::constant_part::ConstantPart>) -> Result<Self, Self::Error> {
        if value.len() > crate::collection_max_len::COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(c93f714a too many constant parts),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
