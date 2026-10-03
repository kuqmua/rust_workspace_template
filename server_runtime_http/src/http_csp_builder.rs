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

#[cfg(test)]
mod tests {
    #[test]
    fn test_csp_builder_value_append_obeys_exact_limit_and_preserves_rejected_state() {
        let name = crate::http_csp_directive_name::HttpCspDirectiveName::try_from(
            constants_str::X.to_owned(),
        );
        assert!(name.is_ok_and(|http_csp_directive_name| {
            let value = crate::http_csp_directive_value::HttpCspDirectiveValue::try_from(
                constants_str::X.repeat(1024usize),
            );
            value.is_ok_and(|http_csp_directive_value| {
                [3068usize, 3069usize].into_iter().all(|length| {
                    let input = constants_str::X.repeat(length);
                    crate::http_csp_builder::HttpCspBuilder::try_from(input.clone()).is_ok_and(|mut builder| {
                        let before = builder.clone();
                        let result = builder.try_add(&http_csp_directive_name, std::slice::from_ref(&http_csp_directive_value));
                        if length == 3069usize {
                            return result == Err(crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge)
                                && builder == before;
                        }
                        let mut expected = input;
                        expected.push_str(constants_str::HTTP_CSP_DIRECTIVE_SEPARATOR);
                        expected.push_str(constants_str::X);
                        expected.push(' ');
                        expected.push_str(http_csp_directive_value.as_str());
                        result == Ok(()) && expected.len() == 4096usize
                            && builder.try_build().is_ok_and(|policy| policy.as_bytes() == expected.as_bytes())
                    })
                })
            })
        }));
    }

    #[test]
    fn test_csp_builder_rejects_oversized_input_and_preserves_state_on_append_failure() {
        assert_eq!(
            crate::http_csp_builder::HttpCspBuilder::try_from(constants_str::X.repeat(4097usize)),
            Err(crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge)
        );
        assert!(crate::http_csp_directive_name::HttpCspDirectiveName::try_from(constants_str::X.to_owned()).is_ok_and(|name| {
            [4093usize, 4094usize, 4096usize].into_iter().all(|length| {
                crate::http_csp_builder::HttpCspBuilder::try_from(constants_str::X.repeat(length)).is_ok_and(|mut builder| {
                    let before = builder.clone();
                    let result = builder.try_add(&name, &[]);
                    if length == 4093usize {
                        let mut expected = constants_str::X.repeat(length);
                        expected.push_str(constants_str::HTTP_CSP_DIRECTIVE_SEPARATOR);
                        expected.push_str(constants_str::X);
                        result == Ok(()) && crate::http_csp_builder::HttpCspBuilder::try_from(expected).is_ok_and(|after| after == builder)
                    } else {
                        result == Err(crate::http_csp_maximum_bytes_error::HttpCspMaximumBytesError::TooLarge)
                            && builder == before
                    }
                })
            })
        }));
    }
}
