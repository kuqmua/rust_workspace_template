use crate::{StdConfigSecretString, StdConfigSecretStringTryFromStringError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefOwned, newtype::FromInner)]
pub struct SecrecySecretBoxString(secrecy::SecretBox<StdConfigSecretString>);
impl TryFrom<String> for SecrecySecretBoxString {
    type Error = StdConfigSecretStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        StdConfigSecretString::try_from(value)
            .map(|bounded| Self::from(secrecy::SecretBox::new(Box::new(bounded))))
    }
}
impl std::fmt::Debug for SecrecySecretBoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
