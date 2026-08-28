// Root-owned module compatibility wrappers.
pub(crate) mod fetch_json {
    pub use crate::fetch_json::*;
}
pub(crate) mod mutation {
    pub use crate::domain_types_start_http_mutation::*;
}
pub(crate) mod url {
    pub use crate::url::*;
}
