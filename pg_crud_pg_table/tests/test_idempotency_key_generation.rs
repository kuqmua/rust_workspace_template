#![allow(
    unused_crate_dependencies,
    reason = "this integration target checks the public key generator; other package dependencies support library implementation"
)]

#[cfg(test)]
mod tests {
    #[test]
    fn test_generated_idempotency_key_is_canonical_uuid_v4() {
        assert!(
            pg_table::new_pg_table_idempotency_key::new_pg_table_idempotency_key().is_ok_and(
                |pg_table_idempotency_key| {
                    uuid::Uuid::parse_str(pg_table_idempotency_key.as_ref()).is_ok_and(
                        |generated_uuid| {
                            generated_uuid.get_version() == Some(uuid::Version::Random)
                                && generated_uuid.get_variant() == uuid::Variant::RFC4122
                                && generated_uuid.to_string() == pg_table_idempotency_key.as_ref()
                                && pg_table_idempotency_key.as_ref().len() == 36usize
                        },
                    )
                }
            )
        );
    }
}
