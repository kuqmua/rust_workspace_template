pub(crate) async fn assignment_action<Ids, Parse, Request, BuildRequest, Target, Run, RunFuture>(
    auth: crate::admin_auth_req::AdminAuthReq,
    expected: &crate::admin_html_form_text::AdminHtmlFormText,
    selected: &crate::std_admin_html_selected::StdAdminHtmlSelected,
    parse: Parse,
    path: server_admin_contract::admin_frontend_path::AdminFrontendPath,
    build_request: BuildRequest,
    target: Target,
    run: Run,
) -> axum::response::Response
where
    Parse: Fn(
        &crate::admin_html_form_text::AdminHtmlFormText,
    ) -> Result<Ids, crate::admin_error::AdminError>,
    BuildRequest: FnOnce(Ids, Ids) -> Request,
    Run: FnOnce(
        crate::admin_auth_req::AdminAuthReq,
        Target,
        crate::axum_admin_json::AxumAdminJson<Request>,
    ) -> RunFuture,
    RunFuture: Future<
        Output = Result<
            crate::axum_admin_response::AxumAdminResponse,
            crate::admin_error::AdminError,
        >,
    >,
{
    let (auth, expected, selected) = match (|| {
        let auth = crate::form_auth_impl::form_auth_impl(auth)?;
        let expected = parse(expected)?;
        let separator = constants_str::COMMA_SPACE.trim();
        let capacity = selected
            .iter()
            .map(|(_key, value)| value.len().get())
            .sum::<usize>()
            .saturating_add(
                selected
                    .len()
                    .get()
                    .saturating_sub(constants_usize::ONE)
                    .saturating_mul(separator.len()),
            );
        let text = selected.iter().map(|(_key, value)| value).enumerate().fold(
            String::with_capacity(capacity),
            |mut text, (index, value)| {
                if index > constants_usize::ZERO {
                    text.push_str(separator);
                }
                text.push_str(value.as_ref());
                text
            },
        );
        let selected_ids = crate::admin_html_form_text::AdminHtmlFormText::try_from(text)
            .map_err(|_error| crate::admin_error::AdminError::Validation)
            .and_then(|value| parse(&value))?;
        Ok::<_, crate::admin_error::AdminError>((auth, expected, selected_ids))
    })() {
        Ok(values) => values,
        Err(error) => return axum::response::IntoResponse::into_response(error),
    };
    crate::action_result_impl::action_result_impl(
        run(
            auth,
            target,
            crate::axum_admin_json::AxumAdminJson::from(build_request(expected, selected)),
        )
        .await,
        path,
    )
}
