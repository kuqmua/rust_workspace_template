#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(in crate::domain_types::start) enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
}

impl AdminMutationMethod {
    pub(in crate::domain_types::start) const fn get(self) -> &'static str {
        match self {
            Self::Delete => constants_str::DELETE,
            Self::Patch => constants_str::PATCH,
            Self::Post => constants_str::POST,
        }
    }
}
