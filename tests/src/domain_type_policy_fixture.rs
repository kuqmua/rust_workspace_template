struct DomainId(u32);
struct DomainName(String);
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
