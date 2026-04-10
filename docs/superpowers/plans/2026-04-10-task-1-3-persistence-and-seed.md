# Task 1.3 Persistence And Seed Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add minimal SQLite persistence bootstrap and idempotent seed data for administrator-precreated users and sample CRM records.

**Architecture:** Keep persistence isolated in a dedicated Rust module so the Tauri backend owns schema creation and sample data loading before any commands or UI use it. Use SQLite through `rusqlite` with an in-memory test setup so the persistence contract is verified without coupling this task to app startup or runtime paths yet.

**Tech Stack:** Rust, Tauri 2, rusqlite, Serde

---

### Task 1: Add persistence bootstrap and seed data

**Files:**
- Create: `desktop/src-tauri/src/storage.rs`
- Create: `desktop/src-tauri/tests/storage.rs`
- Modify: `desktop/src-tauri/Cargo.toml`
- Modify: `desktop/src-tauri/src/lib.rs`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing storage tests**

Add integration tests that prove:
- `bootstrap_database` creates `users`, `leads`, `customers`, and `follow_ups` tables
- lead and customer status constraints are fixed to the approved enum wire values
- `seed_database` inserts administrator-precreated users and sample CRM records
- `seed_database` is idempotent and does not duplicate rows when run twice

- [ ] **Step 2: Run the storage tests to verify they fail**

Run: `cargo test --test storage --manifest-path desktop/src-tauri/Cargo.toml`
Expected: FAIL because the storage module and persistence functions do not exist yet.

- [ ] **Step 3: Implement the minimal SQLite bootstrap module**

Add a storage module that:
- opens or receives a SQLite connection
- creates the four core tables
- applies status `CHECK` constraints for leads and customers
- inserts a tiny deterministic seed set for users, leads, customers, and follow-ups
- returns a seed summary so later tasks can verify bootstrap state

- [ ] **Step 4: Run the storage tests to verify they pass**

Run: `cargo test --test storage --manifest-path desktop/src-tauri/Cargo.toml`
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
