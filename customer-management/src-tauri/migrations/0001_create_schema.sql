CREATE TYPE lead_status AS ENUM ('new', 'assigned', 'in_progress', 'converted', 'closed');
CREATE TYPE customer_status AS ENUM ('active', 'silent', 'lost');
CREATE TYPE follow_up_target_type AS ENUM ('lead', 'customer');

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS leads (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,
    company TEXT NOT NULL,
    email TEXT,
    notes TEXT,
    address TEXT,
    owner_id TEXT NOT NULL REFERENCES users(id),
    status lead_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,
    company TEXT NOT NULL,
    email TEXT,
    notes TEXT,
    address TEXT,
    owner_id TEXT NOT NULL REFERENCES users(id),
    status customer_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS follow_ups (
    id TEXT PRIMARY KEY,
    target_type follow_up_target_type NOT NULL,
    lead_id TEXT REFERENCES leads(id),
    customer_id TEXT REFERENCES customers(id),
    content TEXT NOT NULL,
    follow_up_time TIMESTAMPTZ NOT NULL,
    method TEXT NOT NULL,
    result TEXT NOT NULL,
    next_follow_up_time TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (
        (target_type = 'lead' AND lead_id IS NOT NULL AND customer_id IS NULL)
        OR
        (target_type = 'customer' AND customer_id IS NOT NULL AND lead_id IS NULL)
    )
);
