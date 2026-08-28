#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Constants(pub(super) Vec<super::Constant>);

impl TryFrom<Vec<super::Constant>> for Constants {
    type Error = syn::Error;
    fn try_from(value: Vec<super::Constant>) -> Result<Self, Self::Error> {
        if value.len() > super::COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(2bd1b963 too many constants),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
