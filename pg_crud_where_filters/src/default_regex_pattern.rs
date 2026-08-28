use super::RegexRegex;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct DefaultRegexPattern;
impl From<DefaultRegexPattern> for RegexRegex {
    fn from(_value: DefaultRegexPattern) -> Self {
        Self(String::from(constants_str::A_Z_PLUS))
    }
}
