#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, PartialEq, Eq)]
pub struct AdminInputName(Box<str>);

impl From<&'static str> for AdminInputName {
    fn from(str: &'static str) -> Self {
        Self(Box::<str>::from(str))
    }
}

impl From<server_admin_contract::admin_setting_name::AdminSettingName> for AdminInputName {
    fn from(
        admin_setting_name: server_admin_contract::admin_setting_name::AdminSettingName,
    ) -> Self {
        Self(admin_setting_name.as_ref().to_owned().into_boxed_str())
    }
}

impl AsRef<str> for AdminInputName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
