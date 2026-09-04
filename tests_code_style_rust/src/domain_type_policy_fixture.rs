#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_newtype_from_inner::FromInner)]
struct DomainId(u32);
#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DomainName(String);
const DOMAIN_NAME_MAX_LEN: usize = 1_048_576;
#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DomainNameTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for DomainNameTryFromStringError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(formatter, "domain domain_name length {len} exceeds maximum {max}")
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
#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DomainEntity {
    domain_id: DomainId,
    domain_name: DomainName,
}
#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum DomainEvent {
    Created(DomainEntity),
}
#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_newtype_from_inner::FromInner)]
struct DomainEvents(Vec<DomainEvent>);
fn make_domain_entity(domain_id: DomainId, domain_name: DomainName) -> DomainEntity {
    DomainEntity { domain_id, domain_name }
}
fn domain_events(domain_entity: DomainEntity) -> DomainEvents {
    DomainEvents::from(vec![DomainEvent::Created(domain_entity)])
}
#[cfg(test)]
fn raw_type_test_only(u32: u32) -> u32 {
    u32
}
