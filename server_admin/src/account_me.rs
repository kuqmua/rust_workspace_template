pub(crate) async fn account_me(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    crate::account_me_context_view_ref::account_me_context_view_ref(&auth)
        .await
        .map(|context| crate::shared::json_response::json_response(context.0))
}
