use super::SingleFlightInner;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
pub(super) struct ArcSingleFlightRwLock(
    pub(super) std::sync::Arc<std::sync::RwLock<SingleFlightInner>>,
);
