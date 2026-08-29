#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Eq,
    PartialEq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct HttpBearerTokenRef<'value_lt>(&'value_lt str);

impl std::fmt::Debug for HttpBearerTokenRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::catalog::REDACTED_ALT_3)
    }
}
