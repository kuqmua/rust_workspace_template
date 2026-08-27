#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, chars, serde, utoipa, validator = |value: &String| { let path = crate::domain_types::AdminPagePathRef::from(value.as_str()); crate::domain_types::AdminPage::from_path(path).is_some() || crate::domain_types::AdminDataTable::from_frontend_path(path).is_some() }, description = "administrator default route")]
pub struct AdminDefaultRoute(String);
