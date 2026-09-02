#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub struct SecrecySecretBoxString(
    secrecy::SecretBox<crate::std_config_secret_string::StdConfigSecretString>,
);
impl TryFrom<String> for SecrecySecretBoxString {
    type Error = crate::std_config_secret_string::StdConfigSecretStringTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        crate::std_config_secret_string::StdConfigSecretString::try_from(string)
            .map(|bounded| Self::from(secrecy::SecretBox::new(Box::new(bounded))))
    }
}
impl std::fmt::Debug for SecrecySecretBoxString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::REDACTED_ALT_3)
    }
}
