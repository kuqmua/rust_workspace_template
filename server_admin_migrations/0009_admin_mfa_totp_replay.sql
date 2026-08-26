ALTER TABLE admin_user_mfa
    ADD COLUMN IF NOT EXISTS last_totp_counter BIGINT;

ALTER TABLE admin_user_mfa
    DROP CONSTRAINT IF EXISTS admin_user_mfa_last_totp_counter_nonnegative;

ALTER TABLE admin_user_mfa
    ADD CONSTRAINT admin_user_mfa_last_totp_counter_nonnegative
    CHECK (last_totp_counter IS NULL OR last_totp_counter >= 0);
