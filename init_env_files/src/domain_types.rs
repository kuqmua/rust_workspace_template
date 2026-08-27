#[path = "domain_types/run_mode.rs"]
mod run_mode;
pub(crate) use run_mode::*;
#[path = "domain_types/initialization_status.rs"]
mod initialization_status;
pub(crate) use initialization_status::*;
#[path = "domain_types/initialization_entry.rs"]
mod initialization_entry;
pub(crate) use initialization_entry::*;
#[path = "domain_types/env_content.rs"]
mod env_content;
pub(crate) use env_content::*;
#[path = "domain_types/env_content_ref.rs"]
mod env_content_ref;
pub(crate) use env_content_ref::*;
#[path = "domain_types/env_key.rs"]
mod env_key;
pub(crate) use env_key::*;
#[path = "domain_types/env_keys.rs"]
mod env_keys;
pub(crate) use env_keys::*;
#[path = "domain_types/workspace_member.rs"]
mod workspace_member;
pub(crate) use workspace_member::*;
#[path = "domain_types/workspace_root_path_ref.rs"]
mod workspace_root_path_ref;
pub(crate) use workspace_root_path_ref::*;
#[path = "domain_types/init_path_ref.rs"]
mod init_path_ref;
pub(crate) use init_path_ref::*;
#[path = "domain_types/init_max_bytes.rs"]
mod init_max_bytes;
pub(crate) use init_max_bytes::*;
#[path = "domain_types/init_path_exists.rs"]
mod init_path_exists;
pub(crate) use init_path_exists::*;
#[path = "domain_types/init_entries.rs"]
mod init_entries;
pub(crate) use init_entries::*;
#[path = "domain_types/init_io_error.rs"]
mod init_io_error;
pub(crate) use init_io_error::*;
#[path = "domain_types/server_runtime_bounded_read_error.rs"]
mod server_runtime_bounded_read_error;
pub(crate) use server_runtime_bounded_read_error::*;
#[path = "domain_types/toml_init_error.rs"]
mod toml_init_error;
pub(crate) use toml_init_error::*;
#[path = "domain_types/init_string_error.rs"]
mod init_string_error;
pub(crate) use init_string_error::*;
#[path = "domain_types/initialize_error.rs"]
mod initialize_error;
pub(crate) use initialize_error::*;
#[path = "domain_types/environment_keys.rs"]
mod environment_keys;
use environment_keys::*;
#[path = "domain_types/initialize.rs"]
mod initialize;
pub(crate) use initialize::*;

#[cfg(test)]
mod tests {
    fn fixture() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rust-workspace-template-environment-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(root.join(constants_str::SERVICE))
            .expect("fdbf7411 fixture invariant must hold");
        std::fs::write(
            root.join(constants_str::CARGO_TOML),
            constants_str::WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE,
        )
        .expect("8e781c83 fixture invariant must hold");
        std::fs::write(
            root.join(constants_str::SERVICE_ENV_EXAMPLE),
            constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE,
        )
        .expect("f24fca72 fixture invariant must hold");
        root
    }
    #[test]
    fn dry_run_apply_and_repeat_are_safe_and_idempotent() {
        let root = fixture();
        let dry = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::DryRun,
        )
        .expect("93ce4136 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            dry.0
                .first()
                .expect(
                    "14b080ca dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::WouldCreate
        );
        assert!(!root.join("service/.env").exists());
        let applied = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("d58ed6a5 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            applied
                .0
                .first()
                .expect(
                    "c366cc59 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::Created
        );
        std::fs::write(
            root.join(constants_str::SERVICE_ENV),
            constants_str::SECRET_CUSTOM_NEWLINE,
        )
        .expect("2d67b058 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        let updated = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("546af7b6 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            updated
                .0
                .first()
                .expect(
                    "195600ec dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::Updated
        );
        let updated_content = std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
            .expect(
                "bd9f5208 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold",
            );
        assert!(updated_content.contains("SECRET=custom"));
        assert!(updated_content.contains("PUBLIC=value"));
        let repeated = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("a452843a dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            repeated
                .0
                .first()
                .expect(
                    "37a0752c dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::SkippedExisting
        );
        std::fs::remove_dir_all(root).expect(
            "bd9180ca dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold",
        );
    }
    #[test]
    fn escaping_member_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(constants_str::CARGO_TOML),
            constants_str::WORKSPACE_NEWLINE_MEMBERS_OUTSIDE_NEWLINE,
        )
        .expect("350646f2 escaping_member_is_rejected invariant must hold");
        assert!(matches!(
            super::initialize(
                super::WorkspaceRootPathRef::from(root.as_path()),
                super::RunMode::DryRun
            ),
            Err(super::InitializeError::InvalidMember { .. })
        ));
        std::fs::remove_dir_all(root)
            .expect("d9154402 escaping_member_is_rejected invariant must hold");
    }
    #[test]
    fn oversized_environment_example_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(constants_str::SERVICE_ENV_EXAMPLE),
            constants_str::A_ALT
                .repeat(constants_usize::VALUE_1_048_576.saturating_add(constants_usize::ONE)),
        )
        .expect("f6290e85 oversized_environment_example_is_rejected invariant must hold");
        assert!(matches!(
            super::initialize(
                super::WorkspaceRootPathRef::from(root.as_path()),
                super::RunMode::DryRun
            ),
            Err(super::InitializeError::ReadExample {
                source: super::ServerRuntimeBoundedReadError(
                    server_runtime_http::domain_types::BoundedReadError::ExceedsMaximum { .. }
                )
            })
        ));
        std::fs::remove_dir_all(root)
            .expect("7d83384c oversized_environment_example_is_rejected invariant must hold");
    }
}
