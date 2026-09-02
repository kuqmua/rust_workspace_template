#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_new::New, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminSessionBundle {
    access_token: crate::std_admin_access_token::StdAdminAccessToken,
    csrf_token: crate::admin_opaque_token::AdminOpaqueToken,
    refresh_token: crate::admin_refresh_token::AdminRefreshToken,
    #[getters(copy)]
    session_id: crate::admin_session_id::AdminSessionId,
}
