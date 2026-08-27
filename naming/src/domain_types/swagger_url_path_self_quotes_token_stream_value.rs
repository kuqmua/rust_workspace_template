#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::FromInner, newtype::ToTokens,
)]
pub struct SwaggerUrlPathSelfQuotesTokenStreamValue(
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
);
