pub trait SwaggerUrlPathSelfQuotesStr {
    fn swagger_url_path_self_quotes_str(
        &self,
        swagger_url_path_prefix: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> Result<
        generate_quotes::quoted_literal::QuotedLiteral,
        generate_quotes::quoted_literal::QuotedLiteralTryFromStringError,
    >;
}

impl<T> SwaggerUrlPathSelfQuotesStr for T
where
    T: naming_common::domain_types::AsRefStrToSnakeCaseStr,
{
    fn swagger_url_path_self_quotes_str(
        &self,
        swagger_url_path_prefix: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> Result<
        generate_quotes::quoted_literal::QuotedLiteral,
        generate_quotes::quoted_literal::QuotedLiteralTryFromStringError,
    > {
        let path_name = self.try_case().map_err(|error| match error {
            naming_common::case_string::CaseStringTryFromStringError::InvalidBounds {
                min,
                max,
            } => generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::InvalidBounds {
                min,
                max,
            },
            naming_common::case_string::CaseStringTryFromStringError::TooShort { len, min } => {
                generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::TooShort {
                    len,
                    min,
                }
            }
            naming_common::case_string::CaseStringTryFromStringError::TooLong { len, max } => {
                generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::TooLong {
                    len,
                    max,
                }
            }
            naming_common::case_string::CaseStringTryFromStringError::ContainsNul => {
                generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::ContainsNul
            }
            naming_common::case_string::CaseStringTryFromStringError::InvalidValue => {
                generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::InvalidValue
            }
        })?;
        generate_quotes::double_quoted_string::double_quoted_string(&format!(
            "/{}/{}",
            swagger_url_path_prefix.as_ref(),
            path_name
        ))
    }
}
