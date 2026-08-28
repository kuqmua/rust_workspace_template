#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::AsRefStr, newtype::FromInner,
)]
pub struct SwaggerUrlPathSelfQuotesStrValue(generate_quotes::domain_types::QuotedLiteral);
