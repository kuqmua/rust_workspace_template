use super::domain_types::{StdAdminString, StdAdminStringTryFromStringError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefOwned, newtype::FromInner)]
pub struct SecrecyAdminString(secrecy::SecretBox<StdAdminString>);

impl std::fmt::Debug for SecrecyAdminString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
impl TryFrom<String> for SecrecyAdminString {
    type Error = StdAdminStringTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        StdAdminString::try_from(value)
            .map(|bounded| Self::from(secrecy::SecretBox::new(Box::new(bounded))))
    }
}
impl secrecy::ExposeSecret<StdAdminString> for SecrecyAdminString {
    fn expose_secret(&self) -> &StdAdminString {
        secrecy::ExposeSecret::expose_secret(&self.0)
    }
}
