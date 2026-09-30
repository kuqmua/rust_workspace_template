ALTER TABLE refresh_tokens ADD COLUMN session_id UUID;
UPDATE refresh_tokens
SET revoked_at = GREATEST(NOW(), created_at)
WHERE revoked_at IS NULL;
ALTER TABLE refresh_tokens
ADD CONSTRAINT refresh_tokens_active_session_required
CHECK (revoked_at IS NOT NULL OR session_id IS NOT NULL);
CREATE INDEX refresh_tokens_active_session_idx
ON refresh_tokens (session_id)
WHERE revoked_at IS NULL;
