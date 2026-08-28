use super::{ADMIN_COLLECTION_MAX_ITEMS, AdminCollectionError};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(
    from = "bounded_types::domain_types::vector::BoundedVec<T, 0, { ADMIN_COLLECTION_MAX_ITEMS }>"
)]
pub(crate) struct AdminBoundedVec<T>(
    bounded_types::domain_types::vector::BoundedVec<T, 0, { ADMIN_COLLECTION_MAX_ITEMS }>,
);
impl<T> AdminBoundedVec<T> {
    pub(crate) const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T> From<[T; 0]> for AdminBoundedVec<T> {
    fn from(_value: [T; 0]) -> Self {
        Self(bounded_types::domain_types::vector::BoundedVec::from([]))
    }
}
impl<T> TryFrom<Vec<T>> for AdminBoundedVec<T> {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|_error| AdminCollectionError::TooLong)
    }
}
