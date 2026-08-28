#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
}

impl AdminMutationMethod {
    pub(crate) const fn get(self) -> &'static str {
        match self {
            Self::Delete => constants_str::DELETE,
            Self::Patch => constants_str::PATCH,
            Self::Post => constants_str::POST,
        }
    }
}
