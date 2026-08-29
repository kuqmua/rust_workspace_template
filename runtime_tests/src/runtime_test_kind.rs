#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
            Self::ApplicationLiveness => constants_str::test_fixtures::VALUE_2AE6635F,
            Self::ApplicationReadiness => constants_str::test_fixtures::VALUE_27B02AA0,
            Self::NotificationCreation => constants_str::test_fixtures::VALUE_D1712BA9,
            Self::NotificationServiceLiveness => constants_str::test_fixtures::VALUE_FA6BAA20,
            Self::NotificationServiceReadiness => constants_str::test_fixtures::VALUE_7595852C,
        })
    }
}
