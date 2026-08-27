#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AdminButtonKind {
    Button,
    #[default]
    Submit,
}

impl AdminButtonKind {
    pub(super) const fn value(self) -> &'static str {
        match self {
            Self::Button => constants_str::VALUE_C3E2D78F,
            Self::Submit => constants_str::VALUE_75490BD7,
        }
    }
}
