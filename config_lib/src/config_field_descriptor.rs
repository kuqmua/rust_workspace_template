#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub struct ConfigFieldDescriptor {
    #[getters(copy)]
    env_name: crate::env_var_name_ref::EnvVarNameRef<'static>,
    #[getters(copy)]
    example: crate::config_field_example_ref::ConfigFieldExampleRef<'static>,
    parser: fn(
        crate::std_env_var_ok::StdEnvVarOk,
    ) -> crate::config_example_validity::ConfigExampleValidity,
    #[getters(copy)]
    rust_type_name: crate::config_rust_type_name::ConfigRustTypeName,
    #[getters(copy)]
    requirement: crate::config_field_requirement::ConfigFieldRequirement,
    #[getters(copy)]
    sensitivity: crate::config_field_sensitivity::ConfigFieldSensitivity,
}
impl std::fmt::Debug for ConfigFieldDescriptor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::CONFIG_FIELD_DESCRIPTOR)
            .field(constants_str::ENV_NAME, &self.env_name)
            .field(constants_str::EXAMPLE, &self.example)
            .field(constants_str::REQUIRED, &self.requirement)
            .field(constants_str::RUST_TYPE_NAME, &self.rust_type_name)
            .field(constants_str::SENSITIVITY, &self.sensitivity)
            .finish_non_exhaustive()
    }
}
impl ConfigFieldDescriptor {
    #[must_use]
    pub const fn new(
        env_var_name_ref: crate::env_var_name_ref::EnvVarNameRef<'static>,
        config_field_example_ref: crate::config_field_example_ref::ConfigFieldExampleRef<'static>,
        parser: fn(
            crate::std_env_var_ok::StdEnvVarOk,
        ) -> crate::config_example_validity::ConfigExampleValidity,
        config_field_requirement: crate::config_field_requirement::ConfigFieldRequirement,
        config_rust_type_name: crate::config_rust_type_name::ConfigRustTypeName,
        config_field_sensitivity: crate::config_field_sensitivity::ConfigFieldSensitivity,
    ) -> Self {
        Self {
            env_name: env_var_name_ref,
            example: config_field_example_ref,
            parser,
            rust_type_name: config_rust_type_name,
            requirement: config_field_requirement,
            sensitivity: config_field_sensitivity,
        }
    }

    #[must_use]
    pub fn validate_example(
        self,
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> crate::config_example_validity::ConfigExampleValidity {
        (self.parser)(std_env_var_ok)
    }
}
