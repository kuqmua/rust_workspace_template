#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum UserMutationFormTarget {
    Ban(crate::user_ban_form::UserBanForm),
    Password(crate::user_password_form::UserPasswordForm),
}
