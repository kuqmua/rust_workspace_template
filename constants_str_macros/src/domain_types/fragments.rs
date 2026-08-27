#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct Fragments(pub(super) Vec<super::Fragment>);

#[allow(
    clippy::useless_concat,
    reason = "the constants_str generator cannot depend on the crate that it generates"
)]
impl TryFrom<Vec<super::Fragment>> for Fragments {
    type Error = syn::Error;
    fn try_from(value: Vec<super::Fragment>) -> Result<Self, Self::Error> {
        if value.len() > super::COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                concat!("883ea6b2 too many fragments"),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
