#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(
    max = 36usize,
    min = 36usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.bytes().enumerate().all(|(index, byte)| if matches!(index, 8usize | 13usize | 18usize | 23usize) { byte == b'-' } else { byte.is_ascii_hexdigit() }),
    description = "administrator refresh token identifier"
)]
pub struct AdminRefreshTokenId(bounded_types::bounded_string::BoundedString<36usize, 36, true>);
impl AdminRefreshTokenId {
    #[must_use]
    pub fn from_frontend_path(
        admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        admin_page_path_ref
            .record_identifier(crate::admin_data_table::AdminDataTable::RefreshTokens)
            .map(str::to_owned)
            .map(Self::try_from)
            .and_then(Result::ok)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_refresh_token_identifier_parses_from_detail_frontend_path() {
        let expected_identifier_result = super::AdminRefreshTokenId::try_from(String::from(
            constants_str::TEST_REFRESH_TOKEN_ID,
        ));
        assert_eq!(
            expected_identifier_result.iter().count(),
            constants_usize::ONE
        );
        expected_identifier_result
            .ok()
            .into_iter()
            .for_each(|expected_identifier| {
                let route_path =
                    crate::admin_route_path::AdminRoutePath::from(expected_identifier.clone());
                let identifier = super::AdminRefreshTokenId::from_frontend_path(
                    crate::admin_page_path_ref::AdminPagePathRef::from(route_path.as_ref()),
                );
                assert_eq!(identifier, Some(expected_identifier));
            });
        assert!(
            super::AdminRefreshTokenId::from_frontend_path(
                crate::admin_page_path_ref::AdminPagePathRef::from(
                    crate::admin_data_table::AdminDataTable::RefreshTokens
                        .frontend_path()
                        .as_ref(),
                ),
            )
            .is_none()
        );
    }
}
