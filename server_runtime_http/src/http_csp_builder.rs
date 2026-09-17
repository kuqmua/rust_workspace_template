#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq,
)]
pub struct HttpCspBuilder(bounded_types::bounded_string::BoundedString<0usize, 4_096usize, false>);

impl TryFrom<String> for HttpCspBuilder {
    type Error = crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_4_096 {
            return Err(crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::TooLarge,
            })
    }
}

impl HttpCspBuilder {
    pub fn try_add(
        &mut self,
        http_csp_directive_name: &crate::http_csp_directive_name::HttpCspDirectiveName,
        values: &[crate::http_csp_directive_value::HttpCspDirectiveValue],
    ) -> Result<(), crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError> {
        let separator_bytes = if self.0.is_empty() {
            constants_usize::ZERO
        } else {
            constants_usize::TWO
        };
        let values_bytes = values
            .iter()
            .map(|value| value.as_str().len().saturating_add(constants_usize::ONE))
            .sum::<usize>();
        let added_bytes = separator_bytes
            .saturating_add(http_csp_directive_name.as_str().len())
            .saturating_add(values_bytes);
        if self.0.as_str().len().saturating_add(added_bytes) > constants_usize::VALUE_4_096 {
            return Err(crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge);
        }
        if !self.0.is_empty() {
            self.0
                .try_push_str(constants_str::HTTP_CSP_DIRECTIVE_SEPARATOR)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge,
                })?;
        }
        self.0
            .try_push_str(http_csp_directive_name.as_str())
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge,
            })?;
        values
            .iter()
            .try_for_each(|value| {
                self.0.try_push(' ')?;
                self.0.try_push_str(value.as_str())
            })
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge,
            })?;
        Ok(())
    }

    pub fn try_build(
        self,
    ) -> Result<
        crate::http_content_security_policy::HttpContentSecurityPolicy,
        crate::http_content_security_policy_error::HttpContentSecurityPolicyError,
    > {
        crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            self.0.into_string(),
        )
    }
}
