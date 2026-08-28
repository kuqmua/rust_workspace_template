use super::{MigrationsSourceError, validate_migrations_source};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = validate_migrations_source)]
pub struct MigrationsSource(String);
