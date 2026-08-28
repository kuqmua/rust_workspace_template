use crate::domain_types::{DatabaseUrlError, validate_database_url};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = validate_database_url)]
pub struct DatabaseUrl(String);
