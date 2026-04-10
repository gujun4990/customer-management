INSERT INTO users (id, username, password_hash, is_admin)
VALUES
    ('user-admin', 'admin', 'pbkdf2_sha256$100000$seed-admin-salt$763c1c0d50e9f82b2c3540702bc019df7137c0447de9763e7dc60c4151dfde64', TRUE),
    ('user-sales', 'sales', 'pbkdf2_sha256$100000$seed-sales-salt$cafbf78a677e5928bfa165b38325fc3d73ed9f86ece7868ce9bb3be3dc78d5c8', FALSE)
ON CONFLICT (id) DO NOTHING;

INSERT INTO leads (id, name, phone, company, email, notes, address, owner_id, status)
VALUES
    (
        'lead-alice',
        'Alice Zhang',
        '13800138000',
        'Acme Co',
        'alice@example.com',
        'Priority inbound lead',
        'Shanghai',
        'user-sales',
        'new'
    ),
    (
        'lead-bob',
        'Bob Li',
        '13900139000',
        'Beta Ltd',
        NULL,
        'Requested a callback next week',
        'Hangzhou',
        'user-admin',
        'assigned'
    )
ON CONFLICT (id) DO NOTHING;

INSERT INTO customers (id, name, phone, company, email, notes, address, owner_id, status)
VALUES
    (
        'customer-carol',
        'Carol Chen',
        '13700137000',
        'Gamma Group',
        'carol@example.com',
        'Existing retained customer',
        'Suzhou',
        'user-admin',
        'active'
    )
ON CONFLICT (id) DO NOTHING;

INSERT INTO follow_ups (id, target_type, lead_id, customer_id, content, follow_up_time, method, result, next_follow_up_time)
VALUES
    (
        'follow-up-lead-alice',
        'lead',
        'lead-alice',
        NULL,
        'Initial qualification call completed',
        '2026-04-10T09:30:00Z',
        'phone',
        'qualified',
        '2026-04-12T09:30:00Z'
    ),
    (
        'follow-up-customer-carol',
        'customer',
        NULL,
        'customer-carol',
        'Shared onboarding summary',
        '2026-04-10T11:00:00Z',
        'email',
        'delivered',
        NULL
    )
ON CONFLICT (id) DO NOTHING;
