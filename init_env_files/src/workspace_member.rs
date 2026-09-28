#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
pub(crate) struct WorkspaceMember(
    bounded_types::bounded_string::BoundedString<1usize, 4_096usize, false>,
);
impl TryFrom<String> for WorkspaceMember {
    type Error = crate::init_string_error::InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 4_096usize {
            Err(Self::Error::Invalid)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Invalid,
                })
        }
    }
}
impl TryFrom<crate::toml_member_value::TomlMemberValue<'_>> for WorkspaceMember {
    type Error = crate::initialize_error::InitializeError;

    fn try_from(value: crate::toml_member_value::TomlMemberValue<'_>) -> Result<Self, Self::Error> {
        let raw_member = toml::Value::as_str(<&toml::Value>::from(value))
            .ok_or(Self::Error::InvalidMemberType)?;
        let member = Self::try_from(raw_member.to_owned())?;
        let member_path = std::path::Path::new(member.as_ref());
        if member_path.is_relative()
            && member_path
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_)))
        {
            Ok(member)
        } else {
            Err(Self::Error::InvalidMember { member })
        }
    }
}
