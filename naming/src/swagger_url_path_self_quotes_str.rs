pub trait SwaggerUrlPathSelfQuotesStr {
    fn swagger_url_path_self_quotes_str(
        &self,
        v: crate::domain_types::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::domain_types::QuotedLiteral;
}

impl<T> SwaggerUrlPathSelfQuotesStr for T
where
    T: naming_common::domain_types::AsRefStrToSnakeCaseStr,
{
    fn swagger_url_path_self_quotes_str(
        &self,
        v: crate::domain_types::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::domain_types::QuotedLiteral {
        generate_quotes::domain_types::double_quoted_string(&format!(
            "/{}/{}",
            v.as_ref(),
            self.case()
        ))
    }
}
