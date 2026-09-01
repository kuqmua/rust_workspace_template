#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::CONFIG_FIELD_DESCRIPTOR)
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
        env_name: crate::env_var_name_ref::EnvVarNameRef<'static>,
        example: crate::config_field_example_ref::ConfigFieldExampleRef<'static>,
        parser: fn(
            crate::std_env_var_ok::StdEnvVarOk,
        ) -> crate::config_example_validity::ConfigExampleValidity,
        requirement: crate::config_field_requirement::ConfigFieldRequirement,
        rust_type_name: crate::config_rust_type_name::ConfigRustTypeName,
        sensitivity: crate::config_field_sensitivity::ConfigFieldSensitivity,
    ) -> Self {
        Self {
            env_name,
            example,
            parser,
            rust_type_name,
            requirement,
            sensitivity,
        }
    }

    #[must_use]
    pub fn validate_example(
        self,
        value: crate::std_env_var_ok::StdEnvVarOk,
    ) -> crate::config_example_validity::ConfigExampleValidity {
        (self.parser)(value)
    }
}
