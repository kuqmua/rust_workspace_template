pub trait SwaggerUrlPathSelfQuotesStr {
    fn swagger_url_path_self_quotes_str(
        &self,
        v: super::SwaggerUrlPathPrefix<'_>,
    ) -> super::SwaggerUrlPathSelfQuotesStrValue;
}

impl<T> SwaggerUrlPathSelfQuotesStr for T
where
    T: naming_common::domain_types::AsRefStrToSnakeCaseStr,
{
    fn swagger_url_path_self_quotes_str(
        &self,
        v: super::SwaggerUrlPathPrefix<'_>,
    ) -> super::SwaggerUrlPathSelfQuotesStrValue {
        super::SwaggerUrlPathSelfQuotesStrValue::from(
            generate_quotes::domain_types::double_quoted_string(&format!(
                "/{}/{}",
                v.as_ref(),
                self.case()
            )),
        )
    }
}
