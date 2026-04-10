# Task 2.1 Simple Auth Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement preset username/password login and current-user session handling in the Tauri backend.

**Architecture:** Keep authentication logic in a dedicated Rust module so password verification and session state stay separate from storage and future CRM commands. Use the existing PostgreSQL bootstrap for credential lookup and an in-memory session store managed by Tauri application state for the current signed-in user.

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL, PBKDF2-SHA256

---

### Task 1: Add the backend auth module

**Files:**
- Create: `customer-management/src-tauri/src/auth.rs`
- Create: `customer-management/src-tauri/tests/auth.rs`
- Modify: `customer-management/src-tauri/Cargo.toml`
- Modify: `customer-management/src-tauri/src/lib.rs`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing auth tests**

Add tests that prove:
- The seeded PBKDF2 password hash accepts the correct password and rejects the wrong one.
- Session state stores and clears the current signed-in user.
- Authenticating a fetched user record updates the current-user session.
- An optional PostgreSQL smoke test can sign in the seeded `admin` user when `TEST_DATABASE_URL` is present.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test --test auth --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: FAIL because the auth module does not exist yet.

- [ ] **Step 3: Implement the minimal auth layer**

Create a small Rust module that defines:
- password-hash verification for the seeded PBKDF2-SHA256 format
- an authenticated-user session store
- database-backed sign-in for preset username/password accounts
- Tauri commands for sign-in, current-user lookup, and sign-out

Do not add authorization guards, role-based access checks, or frontend login UI in this task.

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --test auth --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**

Change this line in `openspec/changes/add-customer-management-system/tasks.md`:

```md
- [ ] 2.1 Implement preset username/password login and current-user session handling
```

to:

```md
- [x] 2.1 Implement preset username/password login and current-user session handling
```
