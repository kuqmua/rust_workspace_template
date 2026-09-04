#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct SecrecyAdminString(secrecy::SecretBox<crate::std_admin_string::StdAdminString>);

impl std::fmt::Debug for SecrecyAdminString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::REDACTED_ALT_3)
    }
}
impl TryFrom<String> for SecrecyAdminString {
    type Error = crate::std_admin_string::StdAdminStringTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::std_admin_string::StdAdminString::try_from(value)
            .map(|bounded| Self::from(secrecy::SecretBox::new(Box::new(bounded))))
    }
}
impl secrecy::ExposeSecret<crate::std_admin_string::StdAdminString> for SecrecyAdminString {
    fn expose_secret(&self) -> &crate::std_admin_string::StdAdminString {
        secrecy::ExposeSecret::expose_secret(&self.0)
    }
}
