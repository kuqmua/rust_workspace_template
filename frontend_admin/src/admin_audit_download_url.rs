#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = 4_194_304usize)]
pub(crate) struct AdminAuditDownloadUrl(
    bounded_types::bounded_string::BoundedString<0usize, 4_194_304usize, false>,
);

impl TryFrom<&server_admin_contract::admin_audit_export_csv::AdminAuditExportCsv>
    for AdminAuditDownloadUrl
{
    type Error = AdminAuditDownloadUrlTryFromStringError;

    fn try_from(
        value: &server_admin_contract::admin_audit_export_csv::AdminAuditExportCsv,
    ) -> Result<Self, Self::Error> {
        let mut encoded = String::with_capacity(
            value
                .as_ref()
                .len()
                .saturating_mul(3usize)
                .saturating_add(constants_str::ADMIN_AUDIT_DOWNLOAD_PREFIX.len()),
        );
        encoded.push_str(constants_str::ADMIN_AUDIT_DOWNLOAD_PREFIX);
        value.as_ref().bytes().for_each(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                encoded.push(char::from(byte));
            } else {
                encoded.push('%');
                encoded.extend([byte >> 4u8, byte & 15u8].map(|nibble| {
                    char::from(match nibble {
                        0u8..=9u8 => b'0'.saturating_add(nibble),
                        _hexadecimal => b'A'.saturating_add(nibble.saturating_sub(10u8)),
                    })
                }));
            }
        });
        Self::try_from(encoded)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_audit_csv_download_encodes_delimiters_and_whitespace() {
        let csv = server_admin_contract::admin_audit_export_csv::AdminAuditExportCsv::try_from(
            [',', '"', '\n', '%', '+', ' ', '#', '?', '&', '\r']
                .into_iter()
                .collect::<String>(),
        )
        .expect(constants_str::DIAGNOSTIC_B105BD37);
        let download = crate::admin_audit_download_url::AdminAuditDownloadUrl::try_from(&csv)
            .expect(constants_str::DIAGNOSTIC_4832BE46);
        assert_eq!(
            download
                .as_ref()
                .strip_prefix(constants_str::ADMIN_AUDIT_DOWNLOAD_PREFIX),
            Some(constants_str::ADMIN_AUDIT_ENCODED_CSV_FIXTURE),
        );
    }
}
