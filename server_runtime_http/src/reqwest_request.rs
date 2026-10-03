#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct ReqwestRequest(reqwest::Request);

impl ReqwestRequest {
    pub(crate) fn headers_mut(
        &mut self,
    ) -> crate::http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut<'_> {
        crate::http_opentelemetry_header_map_mut::HttpOpentelemetryHeaderMapMut::from(
            self.0.headers_mut(),
        )
    }

    pub(crate) fn host(&self) -> Option<crate::http_host_ref::HttpHostRef<'_>> {
        self.0
            .url()
            .host_str()
            .map(crate::http_host_ref::HttpHostRef::from)
    }

    pub(crate) fn method(&self) -> crate::http_method_ref::HttpMethodRef<'_> {
        crate::http_method_ref::HttpMethodRef::from(self.0.method())
    }
}

impl TryFrom<crate::reqwest_request_builder::ReqwestRequestBuilder> for ReqwestRequest {
    type Error = crate::reqwest_error::ReqwestError;

    fn try_from(
        value: crate::reqwest_request_builder::ReqwestRequestBuilder,
    ) -> Result<Self, Self::Error> {
        reqwest::RequestBuilder::from(value)
            .build()
            .map(Self)
            .map_err(crate::reqwest_error::ReqwestError::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_request_accessors_and_header_mutation_preserve_request() {
        let hostless = format!("{}:{}", constants_str::X, constants_str::X);
        assert!(
            [constants_str::HTTPS_EXAMPLE_COM, hostless.as_str(),]
                .into_iter()
                .all(|text| {
                    reqwest::Url::parse(text).is_ok_and(|url| {
                        [http::Method::POST, http::Method::DELETE]
                            .into_iter()
                            .all(|method| {
                                let mut reqwest_request =
                                    crate::reqwest_request::ReqwestRequest::from(
                                        reqwest::Request::new(method.clone(), url.clone()),
                                    );
                                let accessor_matches = reqwest_request.method().to_string()
                                    == method.as_str()
                                    && reqwest_request.host().map(|host| host.to_string())
                                        == url.host_str().map(str::to_owned);
                                let previous = reqwest_request.headers_mut().insert(
                                    http::header::CONTENT_TYPE,
                                    http::HeaderValue::from_static(constants_str::X),
                                );
                                let request = reqwest_request.into_inner();
                                accessor_matches
                                    && previous.is_none()
                                    && request.method() == method
                                    && request.url() == &url
                                    && request.headers().get(http::header::CONTENT_TYPE)
                                        == Some(&http::HeaderValue::from_static(constants_str::X))
                            })
                    })
                })
        );
    }
}
