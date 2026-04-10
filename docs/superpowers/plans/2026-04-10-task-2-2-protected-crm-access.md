# Task 2.2 Protected CRM Access Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Protect lead and customer data access so unauthenticated users cannot read CRM records.

**Architecture:** Reuse the Tauri backend session from task 2.1 and add a small auth guard that is applied before CRM data queries run. Expose two minimal read commands for leads and customers so task 2.2 has a real protected surface without implementing the broader CRUD flows reserved for later tasks.

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL

---

### Task 1: Add protected CRM read actions

**Files:**
- Create: `customer-management/src-tauri/src/crm.rs`
- Create: `customer-management/src-tauri/tests/crm_access.rs`
- Modify: `customer-management/src-tauri/src/auth.rs`
- Modify: `customer-management/src-tauri/src/lib.rs`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing tests**

Add tests that prove:
- unauthenticated CRM reads are rejected with a generic auth-required error
- authenticated CRM reads can return seeded lead and customer rows
- logging out blocks CRM reads again

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test --test crm_access --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: FAIL because protected CRM access functions do not exist yet.

- [ ] **Step 3: Implement the minimal protected CRM layer**

Create a small Rust module that defines:
- lead/customer row loading from PostgreSQL
- an auth guard using the current session
- Tauri commands for protected lead and customer list access

Do not implement create, update, detail, filtering, or authorization rules beyond rejecting unauthenticated access.

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --test crm_access --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**

Change this line in `openspec/changes/add-customer-management-system/tasks.md`:

```md
- [ ] 2.2 Protect CRM actions so unauthenticated users cannot access lead or customer data
```

to:

```md
- [x] 2.2 Protect CRM actions so unauthenticated users cannot access lead or customer data
```
