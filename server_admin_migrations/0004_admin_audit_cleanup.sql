CREATE OR REPLACE FUNCTION admin_audit_log_append_only() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' AND current_setting('app.admin_audit_cleanup', TRUE) = 'on' THEN
        RETURN OLD;
    END IF;
    RAISE EXCEPTION 'admin_audit_log is append-only';
END;
$$;
