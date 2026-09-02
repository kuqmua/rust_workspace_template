#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, PartialEq, Eq)]
pub struct AdminInputName(Box<str>);

impl From<&'static str> for AdminInputName {
    fn from(value: &'static str) -> Self {
        Self(Box::<str>::from(value))
    }
}

impl From<server_admin_contract::admin_setting_name::AdminSettingName> for AdminInputName {
    fn from(value: server_admin_contract::admin_setting_name::AdminSettingName) -> Self {
        Self(value.as_ref().to_owned().into_boxed_str())
    }
}

impl AsRef<str> for AdminInputName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
