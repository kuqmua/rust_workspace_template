#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Fragments(pub(super) Vec<crate::fragment::Fragment>);

#[allow(
    clippy::useless_concat,
    reason = "the constants_str generator cannot depend on the crate that it generates"
)]
impl TryFrom<Vec<crate::fragment::Fragment>> for Fragments {
    type Error = syn::Error;
    fn try_from(value: Vec<crate::fragment::Fragment>) -> Result<Self, Self::Error> {
        if value.len() > crate::collection_max_len::COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                concat!("883ea6b2 too many fragments"),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
