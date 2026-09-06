#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos generates sibling props fields and builder methods from this component; their framework-defined visibility and method names cannot be changed on the source function"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the audit grid"
)]
pub(crate) fn AdminAuditDownload(
    admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    admin_page_offset: server_admin_contract::admin_page_offset::AdminPageOffset,
) -> impl leptos::prelude::IntoView {
    let state = leptos::prelude::RwSignal::new(
        crate::admin_audit_download_state::AdminAuditDownloadState::Idle,
    );
    let prepare = leptos::prelude::Callback::new(move |_event| {
        leptos::prelude::Set::set(
            &state,
            crate::admin_audit_download_state::AdminAuditDownloadState::Loading,
        );
        wasm_bindgen_futures::spawn_local(async move {
            let result = async {
                let suffix = format!(
                    "?{}={}&{}={}",
                    constants_str::ADMIN_LIMIT_QUERY_KEY,
                    u16::from(admin_page_limit),
                    constants_str::ADMIN_OFFSET_QUERY_KEY,
                    u32::from(admin_page_offset),
                );
                let url = crate::admin_api_url_with_suffix::admin_api_url_with_suffix(
                    server_admin_contract::admin_route::AdminRoute::AuditExport,
                    crate::admin_csr_api_url_suffix_ref::AdminCsrApiUrlSuffixRef::from(
                        suffix.as_str(),
                    ),
                )?;
                crate::fetch_json::fetch_json::<
                    server_admin_contract::admin_audit_export::AdminAuditExport,
                >(&url)
                .await
            }
            .await;
            let next_state = match result {
                Ok(export) => match crate::admin_audit_download_url::AdminAuditDownloadUrl::try_from(
                    export.csv(),
                ) {
                    Ok(url) => {
                        crate::admin_audit_download_state::AdminAuditDownloadState::Ready(url)
                    }
                    Err(error) => {
                        crate::admin_audit_download_state::AdminAuditDownloadState::EncodingFailed(
                            error,
                        )
                    }
                },
                Err(error) => {
                    crate::admin_audit_download_state::AdminAuditDownloadState::RequestFailed(error)
                }
            };
            leptos::prelude::Set::set(&state, next_state);
        });
    });
    leptos::view! {
        <div class="resource-actions audit-download-actions">
            {move || leptos::prelude::With::with(&state, |admin_audit_download_state| {
                match admin_audit_download_state {
                    crate::admin_audit_download_state::AdminAuditDownloadState::Ready(url) => leptos::prelude::IntoAny::into_any(leptos::view! {
                        <a class=crate::admin_button_variant::AdminButtonVariant::Secondary.class() href=url.as_ref().to_owned() download=constants_str::ADMIN_AUDIT_DOWNLOAD_FILENAME>{constants_str::ADMIN_AUDIT_DOWNLOAD_LABEL}</a>
                    }),
                    crate::admin_audit_download_state::AdminAuditDownloadState::Loading => leptos::prelude::IntoAny::into_any(leptos::view! { <crate::admin_spinner::AdminSpinner /> }),
                    current @ (crate::admin_audit_download_state::AdminAuditDownloadState::Idle
                    | crate::admin_audit_download_state::AdminAuditDownloadState::RequestFailed(_)
                    | crate::admin_audit_download_state::AdminAuditDownloadState::EncodingFailed(_)) => {
                        let error = match current {
                            crate::admin_audit_download_state::AdminAuditDownloadState::RequestFailed(error) => Some(error.to_string()),
                            crate::admin_audit_download_state::AdminAuditDownloadState::EncodingFailed(error) => Some(error.to_string()),
                            crate::admin_audit_download_state::AdminAuditDownloadState::Idle
                            | crate::admin_audit_download_state::AdminAuditDownloadState::Loading
                            | crate::admin_audit_download_state::AdminAuditDownloadState::Ready(_) => None,
                        };
                        leptos::prelude::IntoAny::into_any(leptos::view! {
                            {error.map(|message| leptos::view! { <crate::admin_alert::AdminAlert>{message}</crate::admin_alert::AdminAlert> })}
                            <crate::admin_button::AdminButton admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary on_click=prepare>{constants_str::ADMIN_AUDIT_PREPARE_DOWNLOAD_LABEL}</crate::admin_button::AdminButton>
                        })
                    }
                }
            })}
        </div>
    }
}
