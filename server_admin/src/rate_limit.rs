pub(crate) use admin_rate_limit_scope::AdminRateLimitScope;
pub(crate) use enforce_rate_limit::enforce_rate_limit;

// Root-owned module compatibility wrappers.
mod admin_rate_limit_scope {
    pub use crate::admin_rate_limit_scope::*;
}
mod enforce_rate_limit {
    pub use crate::enforce_rate_limit::*;
}
