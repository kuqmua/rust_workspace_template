pub(super) fn parse_admin_positive_u64(
    v: &crate::StdEnvVarOk,
) -> Result<std::num::NonZeroU64, super::TryFromStdEnvVarOkAdminPositiveU64Error> {
    let parsed = v.0.parse::<u64>().map_err(|admin_positive_u64_parsing| {
        super::TryFromStdEnvVarOkAdminPositiveU64Error::Parse {
            admin_positive_u64_parsing: crate::ParseIntError::from(admin_positive_u64_parsing),
        }
    })?;
    std::num::NonZeroU64::new(parsed).ok_or(super::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
}
