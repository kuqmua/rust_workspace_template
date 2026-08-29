#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
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

    pub(crate) fn into_inner(self) -> reqwest::Request {
        self.0
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
        value
            .0
            .build()
            .map(Self)
            .map_err(crate::reqwest_error::ReqwestError::from)
    }
}
