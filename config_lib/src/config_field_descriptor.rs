use crate::{
    ConfigExampleValidity, ConfigFieldExampleRef, ConfigFieldRequirement, ConfigFieldSensitivity,
    ConfigRustTypeName, EnvVarNameRef, StdEnvVarOk,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub struct ConfigFieldDescriptor {
    env_name: EnvVarNameRef<'static>,
    example: ConfigFieldExampleRef<'static>,
    parser: fn(StdEnvVarOk) -> ConfigExampleValidity,
    rust_type_name: ConfigRustTypeName,
    requirement: ConfigFieldRequirement,
    sensitivity: ConfigFieldSensitivity,
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
        env_name: EnvVarNameRef<'static>,
        example: ConfigFieldExampleRef<'static>,
        parser: fn(StdEnvVarOk) -> ConfigExampleValidity,
        requirement: ConfigFieldRequirement,
        rust_type_name: ConfigRustTypeName,
        sensitivity: ConfigFieldSensitivity,
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
    pub const fn env_name(self) -> EnvVarNameRef<'static> {
        self.env_name
    }
    #[must_use]
    pub const fn example(self) -> ConfigFieldExampleRef<'static> {
        self.example
    }
    #[must_use]
    pub const fn requirement(self) -> ConfigFieldRequirement {
        self.requirement
    }
    #[must_use]
    pub const fn rust_type_name(self) -> ConfigRustTypeName {
        self.rust_type_name
    }
    #[must_use]
    pub const fn sensitivity(self) -> ConfigFieldSensitivity {
        self.sensitivity
    }
    #[must_use]
    pub fn validate_example(self, value: StdEnvVarOk) -> ConfigExampleValidity {
        (self.parser)(value)
    }
}
