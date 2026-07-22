const API_URL_MAX_LEN: usize = 1_048_576usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApiUrlPathSegmentRef<'value_lt>(&'value_lt str);
impl<'value_lt> TryFrom<&'value_lt str> for ApiUrlPathSegmentRef<'value_lt> {
    type Error = ApiUrlBuildError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.contains('/')
            || matches!(value, str_constants::DOT | str_constants::DOT_DOT)
        {
            Err(ApiUrlBuildError::InvalidPathSegment)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct ApiUrlQueryComponentRef<'value_lt>(&'value_lt str);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::DisplayConst, newtype::Error)]
#[display_const(str_constants::INVALID_API_URL_PATH_SEGMENT)]
pub enum ApiUrlBuildError {
    InvalidPathSegment,
}

#[derive(
    Clone, Debug, Eq, PartialEq, newtype::AsRefStr, newtype::BoundedString, newtype::IntoInnerFrom,
)]
#[bounded_string(max = API_URL_MAX_LEN)]
pub struct ApiUrl(String);
impl ApiUrl {
    pub fn push_path_segment(&mut self, segment: ApiUrlPathSegmentRef<'_>) {
        if !self.0.ends_with('/') {
            self.0.push('/');
        }
        let hex_digit = |nibble| match nibble {
            0u8 => '0',
            1u8 => '1',
            2u8 => '2',
            3u8 => '3',
            4u8 => '4',
            5u8 => '5',
            6u8 => '6',
            7u8 => '7',
            8u8 => '8',
            9u8 => '9',
            10u8 => 'A',
            11u8 => 'B',
            12u8 => 'C',
            13u8 => 'D',
            14u8 => 'E',
            _ => 'F',
        };
        segment.0.bytes().for_each(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b',') {
                self.0.push(char::from(byte));
            } else {
                self.0.push('%');
                self.0.push(hex_digit(byte >> 4u8));
                self.0.push(hex_digit(byte & 0x0fu8));
            }
        });
    }

    pub fn push_query_pair(
        &mut self,
        name: ApiUrlQueryComponentRef<'_>,
        value: ApiUrlQueryComponentRef<'_>,
    ) {
        self.0.push(if self.0.contains('?') { '&' } else { '?' });
        let hex_digit = |nibble| match nibble {
            0u8 => '0',
            1u8 => '1',
            2u8 => '2',
            3u8 => '3',
            4u8 => '4',
            5u8 => '5',
            6u8 => '6',
            7u8 => '7',
            8u8 => '8',
            9u8 => '9',
            10u8 => 'A',
            11u8 => 'B',
            12u8 => 'C',
            13u8 => 'D',
            14u8 => 'E',
            _ => 'F',
        };
        [&name.0, &value.0]
            .into_iter()
            .enumerate()
            .for_each(|(idx, component)| {
                if idx == 1usize {
                    self.0.push('=');
                }
                component.bytes().for_each(|byte| {
                    if byte.is_ascii_alphanumeric()
                        || matches!(byte, b'-' | b'.' | b'_' | b'~' | b',')
                    {
                        self.0.push(char::from(byte));
                    } else {
                        self.0.push('%');
                        self.0.push(hex_digit(byte >> 4u8));
                        self.0.push(hex_digit(byte & 0x0fu8));
                    }
                });
            });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn path_and_query_components_are_encoded() {
        let mut url = super::ApiUrl::try_from(String::from(str_constants::TEST_API_URL_BASE))
            .expect("17480cb4");
        url.push_path_segment(
            super::ApiUrlPathSegmentRef::try_from(str_constants::TEST_API_URL_SEGMENT)
                .expect("c013abc7"),
        );
        url.push_query_pair(
            str_constants::TEST_API_URL_QUERY_NAME.into(),
            str_constants::TEST_API_URL_QUERY_VALUE.into(),
        );
        assert_eq!(url.as_ref(), str_constants::TEST_API_URL_EXPECTED);
    }

    #[test]
    fn traversal_segments_are_rejected() {
        assert_eq!(
            super::ApiUrlPathSegmentRef::try_from(str_constants::DOT_DOT),
            Err(super::ApiUrlBuildError::InvalidPathSegment)
        );
    }
}
