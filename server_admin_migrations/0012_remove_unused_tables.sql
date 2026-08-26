DROP TABLE IF EXISTS mfa_recovery_codes;
DROP TABLE IF EXISTS user_mfa;
DROP TABLE IF EXISTS table_example;

ALTER TABLE access_sessions
DROP COLUMN IF EXISTS mfa_verified_at;

DELETE FROM permissions
WHERE name IN ('mfa_recovery_codes:read', 'user_mfa:read');
