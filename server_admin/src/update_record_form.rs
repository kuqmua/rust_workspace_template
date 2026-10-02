pub(crate) async fn update_record_form(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_record_update_form: crate::admin_record_update_form::AdminRecordUpdateForm,
) -> crate::axum_admin_response::AxumAdminResponse {
    let response = match admin_record_update_form {
        crate::admin_record_update_form::AdminRecordUpdateForm::User {
            admin_user_id,
            axum_admin_form,
        } if &admin_user_id == axum_admin_form.get_user_id() => {
            crate::update_user::update_user(admin_auth_request, axum_admin_form)
                .await
                .unwrap_or_else(axum::response::IntoResponse::into_response)
        }
        crate::admin_record_update_form::AdminRecordUpdateForm::Role {
            admin_role_id,
            axum_admin_form,
        } if &admin_role_id == axum_admin_form.get_role_id() => {
            crate::update_role::update_role(admin_auth_request, axum_admin_form)
                .await
                .unwrap_or_else(axum::response::IntoResponse::into_response)
        }
        crate::admin_record_update_form::AdminRecordUpdateForm::User { .. }
        | crate::admin_record_update_form::AdminRecordUpdateForm::Role { .. } => {
            axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Validation)
        }
    };
    crate::axum_admin_response::AxumAdminResponse::from(response)
}
