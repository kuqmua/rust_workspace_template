#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum ReadOrUpdate {
    Read,
    Update,
}
impl ReadOrUpdate {
    #[must_use]
    pub fn ucc(&self) -> &dyn naming::domain_types::DisplayPlusToTokens {
        match &self {
            Self::Read => &naming::domain_types::ReadUpperCamelCase,
            Self::Update => &naming::domain_types::UpdateUpperCamelCase,
        }
    }
}
