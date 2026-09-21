#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
)]
#[getters(bare)]
#[derive(proc_macro_new::New)]
pub struct AdminRateLimitId {
    scope: crate::admin_text::AdminText,
    subject: crate::admin_text::AdminText,
}
impl AdminRateLimitId {
    fn decode_path_segment(
        admin_text: &crate::admin_text::AdminText,
    ) -> Option<crate::admin_text::AdminText> {
        let encoded = admin_text.as_ref().as_str();
        let bytes = encoded.as_bytes();
        let mut decoded = Vec::with_capacity(bytes.len());
        let mut index = 0usize;
        while index < bytes.len() {
            let byte = *bytes.get(index)?;
            if byte == b'%' {
                let end = index.checked_add(3usize)?;
                let hexadecimal =
                    std::str::from_utf8(bytes.get(index.checked_add(1usize)?..end)?).ok()?;
                decoded.push(u8::from_str_radix(hexadecimal, 16u32).ok()?);
                index = end;
            } else {
                decoded.push(byte);
                index = index.checked_add(1usize)?;
            }
        }
        crate::admin_text::AdminText::try_from(String::from_utf8(decoded).ok()?).ok()
    }

    #[must_use]
    pub fn from_frontend_path(
        admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        let (encoded_scope, encoded_subject) = admin_page_path_ref
            .record_identifier(crate::admin_data_table::AdminDataTable::RateLimits)?
            .split_once('/')?;
        Some(Self::new(
            Self::decode_path_segment(
                &crate::admin_text::AdminText::try_from(encoded_scope.to_owned()).ok()?,
            )?,
            Self::decode_path_segment(
                &crate::admin_text::AdminText::try_from(encoded_subject.to_owned()).ok()?,
            )?,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rate_limit_identifier_parses_from_detail_frontend_path() {
        let identifier = super::AdminRateLimitId::new(
            crate::admin_text::AdminText::try_from(constants_str::TEST_SIGN_IN_IP.to_owned())
                .expect(constants_str::DIAGNOSTIC_8B8860C0),
            crate::admin_text::AdminText::try_from(constants_str::VALUE_127_0_0_1.to_owned())
                .expect(constants_str::DIAGNOSTIC_3E5168C1),
        );
        let route_path = crate::admin_route_path::AdminRoutePath::from(identifier.clone());
        assert_eq!(
            super::AdminRateLimitId::from_frontend_path(
                crate::admin_page_path_ref::AdminPagePathRef::from(route_path.as_ref()),
            ),
            Some(identifier)
        );
        assert_eq!(
            super::AdminRateLimitId::from_frontend_path(
                crate::admin_page_path_ref::AdminPagePathRef::from(
                    constants_str::TEST_RATE_LIMIT_PERCENT_ENCODED_PATH,
                ),
            ),
            Some(super::AdminRateLimitId::new(
                crate::admin_text::AdminText::try_from(
                    constants_str::TEST_SIGN_IN_IP_LOGIN.to_owned(),
                )
                .expect(constants_str::DIAGNOSTIC_9E9A2FD7),
                crate::admin_text::AdminText::try_from(
                    constants_str::TEST_RATE_LIMIT_SUBJECT.to_owned(),
                )
                .expect(constants_str::DIAGNOSTIC_42D3D0CE),
            ))
        );
    }
}
