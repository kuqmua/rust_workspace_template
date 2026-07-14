struct DomainId(u32);
struct DomainName(String);
const DOMAIN_NAME_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DomainNameTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for DomainNameTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "domain name length {len} exceeds maximum {max}")
            }
        }
    }
}
impl From<DomainNameTryFromStringError> for DomainName {
    fn from(value: DomainNameTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for DomainName {
    type Error = DomainNameTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > DOMAIN_NAME_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: DOMAIN_NAME_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
struct DomainEntity {
    id: DomainId,
    name: DomainName,
}
enum DomainEvent {
    Created(DomainEntity),
}
fn mk_domain_entity(id: DomainId, name: DomainName) -> DomainEntity {
    DomainEntity { id, name }
}
fn domain_events(entity: DomainEntity) -> Vec<DomainEvent> {
    vec![DomainEvent::Created(entity)]
}
#[cfg(test)]
fn raw_type_test_only(v: u32) -> u32 {
    v
}
