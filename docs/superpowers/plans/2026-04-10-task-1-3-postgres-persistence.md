# Task 1.3 PostgreSQL Persistence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add PostgreSQL-backed persistence setup and seed data for administrator-precreated users and sample CRM records in the Tauri backend.

**Architecture:** Replace the temporary SQLite storage layer with a small SQLx PostgreSQL module in `customer-management/src-tauri/src/storage.rs`. Keep schema and seed data in versioned SQL migration files so the Tauri backend has a stable bootstrap path before authentication or CRUD commands are added.

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL

---

### Task 1: Add PostgreSQL bootstrap and seed support

**Files:**
- Create: `customer-management/src-tauri/migrations/0001_create_schema.sql`
- Create: `customer-management/src-tauri/migrations/0002_seed_data.sql`
- Create: `customer-management/src-tauri/tests/storage.rs`
- Modify: `customer-management/src-tauri/Cargo.toml`
- Modify: `customer-management/src-tauri/src/storage.rs`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing storage tests**

Add tests that prove:
- `DatabaseConfig::from_env()` reads `DATABASE_URL` and defaults max connections sanely.
- Schema migration defines PostgreSQL enum types and tables for users, leads, customers, and follow_ups.
- Seed migration includes administrator-precreated users and sample CRM records.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test --test storage --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: FAIL because the current storage layer is SQLite-based and the PostgreSQL setup does not exist yet.

- [ ] **Step 3: Implement the minimal PostgreSQL storage layer**

Replace the SQLite module with:
- `DatabaseConfig`
- `SeedSummary`
- SQLx `PgPool` bootstrap helpers
- SQL migrations for schema and seed data

Use migration files for the actual schema and seed inserts. Do not add login logic, CRUD queries, or repository abstractions yet.

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --test storage --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**

Change this line in `openspec/changes/add-customer-management-system/tasks.md`:

```md
- [ ] 1.3 Add persistence setup and seed data for administrator-precreated internal users and sample CRM records
```

to:

```md
- [x] 1.3 Add persistence setup and seed data for administrator-precreated internal users and sample CRM records
```
