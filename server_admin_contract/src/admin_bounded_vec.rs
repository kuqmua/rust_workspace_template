#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(
    from = "bounded_types::bounded_vec::BoundedVec<T, 0, { crate::admin_collection_max_items::ADMIN_COLLECTION_MAX_ITEMS }>"
)]
pub(crate) struct AdminBoundedVec<T>(
    bounded_types::bounded_vec::BoundedVec<
        T,
        0,
        { crate::admin_collection_max_items::ADMIN_COLLECTION_MAX_ITEMS },
    >,
);
impl<T> AdminBoundedVec<T> {
    pub(crate) const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T> From<[T; 0]> for AdminBoundedVec<T> {
    fn from(value: [T; 0]) -> Self {
        let _: [T; 0] = value;
        Self(bounded_types::bounded_vec::BoundedVec::from([]))
    }
}
impl<T> TryFrom<Vec<T>> for AdminBoundedVec<T> {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self)
            .map_err(|_error| crate::admin_collection_error::AdminCollectionError::TooLong)
    }
}
