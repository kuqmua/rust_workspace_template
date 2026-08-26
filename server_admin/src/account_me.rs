#![allow(clippy::single_call_fn)] // route inventory registers this account operation once

pub(super) async fn account_me(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    super::account_me_context_view_ref::account_me_context_view_ref(&auth)
        .await
        .map(|context| super::shared::json_response::json_response(context.0))
}
