#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct ReqwestRequest(reqwest::Request);

impl ReqwestRequest {
    pub(crate) fn headers_mut(&mut self) -> super::HttpOpentelemetryHeaderMapMut<'_> {
        super::HttpOpentelemetryHeaderMapMut::from(self.0.headers_mut())
    }

    pub(crate) fn host(&self) -> Option<super::HttpHostRef<'_>> {
        self.0.url().host_str().map(super::HttpHostRef::from)
    }

    pub(crate) fn into_inner(self) -> reqwest::Request {
        self.0
    }

    pub(crate) fn method(&self) -> super::HttpMethodRef<'_> {
        super::HttpMethodRef::from(self.0.method())
    }
}

impl TryFrom<super::ReqwestRequestBuilder> for ReqwestRequest {
    type Error = crate::domain_types::ReqwestError;

    fn try_from(value: super::ReqwestRequestBuilder) -> Result<Self, Self::Error> {
        value
            .0
            .build()
            .map(Self)
            .map_err(crate::domain_types::ReqwestError::from)
    }
}
