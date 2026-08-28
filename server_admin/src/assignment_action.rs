pub(in crate::domain_types::auth::html::actions) async fn assignment_action<
    Ids,
    Parse,
    Request,
    BuildRequest,
    Target,
    Run,
    RunFuture,
>(
    auth: super::super::super::AdminAuthReq,
    expected: &super::super::forms::AdminHtmlFormText,
    selected: super::super::forms::StdAdminHtmlSelected,
    parse: Parse,
    path: server_admin_contract::domain_types::AdminFrontendPath,
    build_request: BuildRequest,
    target: Target,
    run: Run,
) -> axum::response::Response
where
    Parse:
        Fn(&super::super::forms::AdminHtmlFormText) -> Result<Ids, super::super::super::AdminError>,
    BuildRequest: FnOnce(Ids, Ids) -> Request,
    Run: FnOnce(
        super::super::super::AdminAuthReq,
        Target,
        super::super::super::AxumAdminJson<Request>,
    ) -> RunFuture,
    RunFuture: Future<
        Output = Result<super::super::super::AxumAdminResponse, super::super::super::AdminError>,
    >,
{
    let (auth, expected, selected) =
        match super::super::authenticated_selected_form_impl::authenticated_selected_form_impl(
            auth, expected, selected, parse,
        ) {
            Ok(values) => values,
            Err(error) => return axum::response::IntoResponse::into_response(error),
        };
    super::super::action_result_impl::action_result_impl(
        run(
            auth,
            target,
            super::super::super::AxumAdminJson(build_request(expected, selected)),
        )
        .await,
        path,
    )
}
