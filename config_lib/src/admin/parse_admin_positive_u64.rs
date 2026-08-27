pub(super) fn parse_admin_positive_u64(
    v: &super::super::StdEnvVarOk,
) -> Result<super::super::ConfigNonZeroU64, super::TryFromStdEnvVarOkAdminPositiveU64Error> {
    let parsed = v.0.parse::<u64>().map_err(|admin_positive_u64_parsing| {
        super::TryFromStdEnvVarOkAdminPositiveU64Error::Parse {
            admin_positive_u64_parsing: super::AdminPositiveU64ParsingError::from(
                super::super::ParseIntError::from(admin_positive_u64_parsing),
            ),
        }
    })?;
    std::num::NonZeroU64::new(parsed)
        .map(super::super::ConfigNonZeroU64::from)
        .ok_or(super::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
}
