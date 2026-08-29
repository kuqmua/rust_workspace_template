pub trait SwaggerUrlPathSelfQuotesStr {
    fn swagger_url_path_self_quotes_str(
        &self,
        v: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::quoted_literal::QuotedLiteral;
}

impl<T> SwaggerUrlPathSelfQuotesStr for T
where
    T: naming_common::domain_types::AsRefStrToSnakeCaseStr,
{
    fn swagger_url_path_self_quotes_str(
        &self,
        v: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::quoted_literal::QuotedLiteral {
        generate_quotes::double_quoted_string::double_quoted_string(&format!(
            "/{}/{}",
            v.as_ref(),
            self.case()
        ))
    }
}
