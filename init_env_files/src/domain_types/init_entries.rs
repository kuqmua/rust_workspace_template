use super::InitializationEntry;
use bounded_types::domain_types::vector::BoundedVec;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoIterator)]
pub(crate) struct InitEntries(pub(super) BoundedVec<InitializationEntry, 0, { usize::MAX }>);
