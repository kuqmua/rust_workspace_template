#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum RuntimeTestKind {
    ApplicationLiveness,
    ApplicationReadiness,
    NotificationCreation,
    NotificationServiceLiveness,
    NotificationServiceReadiness,
}

impl std::fmt::Display for RuntimeTestKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ApplicationLiveness => constants_str::VALUE_2AE6635F,
            Self::ApplicationReadiness => constants_str::VALUE_27B02AA0,
            Self::NotificationCreation => constants_str::VALUE_D1712BA9,
            Self::NotificationServiceLiveness => constants_str::VALUE_FA6BAA20,
            Self::NotificationServiceReadiness => constants_str::VALUE_7595852C,
        })
    }
}
