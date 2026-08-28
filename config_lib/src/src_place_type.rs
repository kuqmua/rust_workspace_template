use crate::{
    EnvParseError, EnvVarResultVarError, ParseCtxRef, ParseEnvVarNameRef,
    parse_from_env_var_from_str,
};

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
        let parsed =
            EnvVarResultVarError::try_from(std::env::var(constants_str::ENV_NAMES_SRC_PLACE_TYPE))
                .map_err(EnvParseError::from)
                .and_then(Self::parse_src_place_type_from_env_var);
        match parsed {
            Ok(v) => v,
            Err(message) => {
                tracing::warn!(
                    error = %message,
                    default = ?default,
                    fix = constants_str::CONFIG_SRC_PLACE_TYPE_FIX_MSG,
                    "using the default source place type"
                );
                default
            }
        }
    }

    #[allow(
        clippy::single_call_fn,
        reason = "fallible parser remains directly testable behind the defaulting facade"
    )]
    pub(super) fn parse_src_place_type_from_env_var(
        v: EnvVarResultVarError,
    ) -> Result<Self, EnvParseError> {
        parse_from_env_var_from_str(
            v,
            ParseEnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            ParseCtxRef::from(constants_str::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        )
    }
}
