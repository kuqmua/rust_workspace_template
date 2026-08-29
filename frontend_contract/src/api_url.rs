#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::IntoInnerFrom,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct ApiUrl(String);

impl ApiUrl {
    pub fn push_path_segment(
        &mut self,
        segment: crate::api_url_path_segment_ref::ApiUrlPathSegmentRef<'_>,
    ) {
        if !self.0.ends_with('/') {
            self.0.push('/');
        }
        self.0.extend(percent_encoding::utf8_percent_encode(
            segment.0,
            crate::api_url_component_encode_set::API_URL_COMPONENT_ENCODE_SET,
        ));
    }

    pub fn push_query_pair(
        &mut self,
        name: crate::api_url_query_component_ref::ApiUrlQueryComponentRef<'_>,
        value: crate::api_url_query_component_ref::ApiUrlQueryComponentRef<'_>,
    ) {
        self.0.push(if self.0.contains('?') { '&' } else { '?' });
        [&name.0, &value.0]
            .into_iter()
            .enumerate()
            .for_each(|(idx, component)| {
                if idx == constants_usize::ONE {
                    self.0.push('=');
                }
                self.0.extend(percent_encoding::utf8_percent_encode(
                    component,
                    crate::api_url_component_encode_set::API_URL_COMPONENT_ENCODE_SET,
                ));
            });
    }
}
