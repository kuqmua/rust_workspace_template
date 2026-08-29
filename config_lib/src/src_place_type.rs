#[derive(
    std::fmt::Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::Display,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::EnumFromStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum SrcPlaceType {
    #[default]
    Github,
    Src,
}
impl SrcPlaceType {
    #[must_use]
    pub fn from_env_or_default() -> Self {
        let default = Self::default();
        if let Err(error) = dotenv::dotenv() {
            tracing::warn!(
                error = %error,
                "dotenv initialization failed while resolving the source place type"
            );
        }
        let parsed = crate::env_var_result_var_error::EnvVarResultVarError::try_from(
            std::env::var(constants_str::catalog::ENV_NAMES_SRC_PLACE_TYPE),
        )
        .map_err(crate::env_parse_error::EnvParseError::from)
        .and_then(|env_v| {
            let env_var_name = crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                constants_str::catalog::ENV_NAMES_SRC_PLACE_TYPE,
            );
            let raw_v = env_v
                .0
                .map_err(|source| crate::env_parse_error::EnvParseError::Read {
                    name: crate::env_var_name::EnvVarName::try_from(env_var_name.0.to_owned())
                        .unwrap_or_else(crate::env_var_name::EnvVarName::from),
                    source: crate::env_var_error::EnvVarError::from(source),
                })?;
            raw_v
                .parse::<Self>()
                .map_err(|error| crate::env_parse_error::EnvParseError::Parse {
                    context: crate::parse_ctx_ref::ParseCtxRef::from(
                        constants_str::catalog::CONFIG_SRC_PLACE_TYPE_PARSE_CTX,
                    ),
                    detail: to_err_string::error_text::ErrorText::try_from(error)
                        .unwrap_or_else(to_err_string::error_text::ErrorText::from),
                })
        });
        match parsed {
            Ok(v) => v,
            Err(message) => {
                tracing::warn!(
                    error = %message,
                    default = ?default,
                    fix = constants_str::catalog::CONFIG_SRC_PLACE_TYPE_FIX_MSG,
                    "using the default source place type"
                );
                default
            }
        }
    }

    #[cfg(test)]
    pub(super) fn parse_src_place_type_from_env_var(
        v: crate::env_var_result_var_error::EnvVarResultVarError,
    ) -> Result<Self, crate::env_parse_error::EnvParseError> {
        crate::parse_from_env_var_from_str_tests::parse_from_env_var_from_str(
            v,
            crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                constants_str::catalog::ENV_NAMES_SRC_PLACE_TYPE,
            ),
            crate::parse_ctx_ref::ParseCtxRef::from(
                constants_str::catalog::CONFIG_SRC_PLACE_TYPE_PARSE_CTX,
            ),
        )
    }
}
