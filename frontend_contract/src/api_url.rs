#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::BoundedStringWrapper,
    newtype::IntoInnerFrom,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct ApiUrl(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);

impl ApiUrl {
    pub fn push_path_segment(
        &mut self,
        segment: crate::api_url_path_segment_ref::ApiUrlPathSegmentRef<'_>,
    ) -> Result<(), ApiUrlTryFromStringError> {
        let mut value = self.0.as_str().to_owned();
        if !value.ends_with('/') {
            value.push('/');
        }
        value.extend(percent_encoding::utf8_percent_encode(
            segment.get(),
            crate::api_url_component_encode_set::API_URL_COMPONENT_ENCODE_SET,
        ));
        *self = Self::try_from(value)?;
        Ok(())
    }

    pub fn push_query_pair(
        &mut self,
        name: crate::api_url_query_component_ref::ApiUrlQueryComponentRef<'_>,
        value: crate::api_url_query_component_ref::ApiUrlQueryComponentRef<'_>,
    ) -> Result<(), ApiUrlTryFromStringError> {
        let mut url = self.0.as_str().to_owned();
        url.push(if url.contains('?') { '&' } else { '?' });
        [name.get(), value.get()]
            .into_iter()
            .enumerate()
            .for_each(|(idx, component)| {
                if idx == constants_usize::ONE {
                    url.push('=');
                }
                url.extend(percent_encoding::utf8_percent_encode(
                    component,
                    crate::api_url_component_encode_set::API_URL_COMPONENT_ENCODE_SET,
                ));
            });
        *self = Self::try_from(url)?;
        Ok(())
    }
}
