#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct DefaultRegexPattern;
impl From<DefaultRegexPattern> for crate::regex_regex::RegexRegex {
    fn from(_value: DefaultRegexPattern) -> Self {
        Self(String::from(constants_str::catalog::A_Z_PLUS))
    }
}
