UPDATE system_settings
SET
    tab_title = COALESCE(tab_title, site_name),
    main_logo = COALESCE(main_logo, 'https://example.com/admin-logo.svg'),
    primary_color = COALESCE(primary_color, '#5b55e7'),
    organization_name = COALESCE(organization_name, site_name),
    organization_contacts = COALESCE(organization_contacts, 'support@example.com'),
    support_url = COALESCE(support_url, 'https://example.com/support');

ALTER TABLE system_settings
    ALTER COLUMN tab_title SET DEFAULT 'Admin',
    ALTER COLUMN tab_title SET NOT NULL,
    ALTER COLUMN main_logo SET DEFAULT 'https://example.com/admin-logo.svg',
    ALTER COLUMN main_logo SET NOT NULL,
    ALTER COLUMN primary_color SET DEFAULT '#5b55e7',
    ALTER COLUMN primary_color SET NOT NULL,
    ALTER COLUMN organization_name SET DEFAULT 'Admin',
    ALTER COLUMN organization_name SET NOT NULL,
    ALTER COLUMN organization_contacts SET DEFAULT 'support@example.com',
    ALTER COLUMN organization_contacts SET NOT NULL,
    ALTER COLUMN support_url SET DEFAULT 'https://example.com/support',
    ALTER COLUMN support_url SET NOT NULL;
