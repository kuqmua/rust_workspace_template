#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq)]
pub struct HttpCspBuilder(String);

impl TryFrom<String> for HttpCspBuilder {
    type Error = super::HttpCspMaximumBytesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_4_096 {
            return Err(super::HttpCspMaximumBytesError);
        }
        Ok(Self(value))
    }
}

impl HttpCspBuilder {
    pub fn try_add(
        &mut self,
        name: &super::HttpCspDirectiveName,
        values: &[super::HttpCspDirectiveValue],
    ) -> Result<(), super::HttpCspMaximumBytesError> {
        let separator_bytes = if self.0.is_empty() {
            constants_usize::ZERO
        } else {
            constants_usize::TWO
        };
        let values_bytes = values
            .iter()
            .map(|value| value.0.len().saturating_add(constants_usize::ONE))
            .sum::<usize>();
        let added_bytes = separator_bytes
            .saturating_add(name.0.len())
            .saturating_add(values_bytes);
        if self.0.len().saturating_add(added_bytes) > constants_usize::VALUE_4_096 {
            return Err(super::HttpCspMaximumBytesError);
        }
        self.0.reserve(added_bytes);
        if !self.0.is_empty() {
            self.0.push_str(constants_str::HTTP_CSP_DIRECTIVE_SEPARATOR);
        }
        self.0.push_str(name.0.as_str());
        let _text = values.iter().fold(&mut self.0, |text, value| {
            text.push(' ');
            text.push_str(value.0.as_str());
            text
        });
        Ok(())
    }

    pub fn try_build(
        self,
    ) -> Result<
        crate::domain_types::HttpContentSecurityPolicy,
        crate::domain_types::HttpContentSecurityPolicyError,
    > {
        crate::domain_types::HttpContentSecurityPolicy::try_from(self.0)
    }
}
