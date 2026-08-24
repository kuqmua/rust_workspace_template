const API_URL_COMPONENT_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApiUrlPathSegmentRef<'value_lt>(&'value_lt str);
impl<'value_lt> TryFrom<&'value_lt str> for ApiUrlPathSegmentRef<'value_lt> {
    type Error = ApiUrlBuildError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.contains('/')
            || matches!(value, constants_str::DOT | constants_str::DOT_DOT)
        {
            Err(ApiUrlBuildError::InvalidPathSegment)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct ApiUrlQueryComponentRef<'value_lt>(&'value_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::INVALID_API_URL_PATH_SEGMENT)]
pub enum ApiUrlBuildError {
    InvalidPathSegment,
}

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
    pub fn push_path_segment(&mut self, segment: ApiUrlPathSegmentRef<'_>) {
        if !self.0.ends_with('/') {
            self.0.push('/');
        }
        self.0.extend(percent_encoding::utf8_percent_encode(
            segment.0,
            API_URL_COMPONENT_ENCODE_SET,
        ));
    }

    pub fn push_query_pair(
        &mut self,
        name: ApiUrlQueryComponentRef<'_>,
        value: ApiUrlQueryComponentRef<'_>,
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
                    API_URL_COMPONENT_ENCODE_SET,
                ));
            });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn path_and_query_components_are_encoded() {
        let mut url = super::ApiUrl::try_from(String::from(constants_str::TEST_API_URL_BASE))
            .expect("17480cb4 path_and_query_components_are_encoded invariant must hold");
        url.push_path_segment(
            super::ApiUrlPathSegmentRef::try_from(constants_str::TEST_API_URL_SEGMENT)
                .expect("c013abc7 path_and_query_components_are_encoded invariant must hold"),
        );
        url.push_query_pair(
            constants_str::TEST_API_URL_QUERY_NAME.into(),
            constants_str::TEST_API_URL_QUERY_VALUE.into(),
        );
        assert_eq!(url.as_ref(), constants_str::TEST_API_URL_EXPECTED);
    }

    #[test]
    fn traversal_segments_are_rejected() {
        assert_eq!(
            super::ApiUrlPathSegmentRef::try_from(constants_str::DOT_DOT),
            Err(super::ApiUrlBuildError::InvalidPathSegment)
        );
    }
}
