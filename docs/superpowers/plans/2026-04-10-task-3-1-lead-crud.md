# Task 3.1 Lead CRUD Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement create, read, update, and list flows for leads with required fields (name, phone, company, owner, status) and optional fields (email, notes, address).

**Architecture:** Extend the existing Tauri backend with SQL-backed CRUD operations for the lead table, reusing the PostgreSQL pool and auth session guard from previous tasks. Expose Tauri commands that perform lead creation, retrieval (by id and list), update, and deletion while enforcing required field validation and optional field handling.

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL

---

### Task 1: Add lead CRUD operations

**Files:**
- Create: `customer-management/src-tauri/src/lead.rs`
- Create: `customer-management/src-tauri/tests/lead_crud.rs`
- Modify: `customer-management/src-tauri/src/lib.rs`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing lead CRUD tests**
Add tests that prove:
- A lead can be created with all required fields and optional fields.
- A lead can be retrieved by id.
- A lead can be updated (partial and full).
- A lead can be deleted.
- Listing leads returns all leads.
- Required field validation rejects missing name, phone, company, owner, or status.
- Optional fields accept null values.
- Creation fails if the owner does not exist.
- Listing leads respects pagination (optional for phase one).

- [ ] **Step 2: Run the tests to verify they fail**
Run: `cargo test --test lead_crud --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: FAIL because the lead module does not exist yet.

- [ ] **Step 3: Implement the minimal lead CRUD layer**
Create a small Rust module that defines:
- Lead creation function with validation.
- Lead retrieval by id and list.
- Lead update function.
- Lead deletion function.
- Tauri commands for each operation, protected by the auth session.

Do not implement lead-to-customer conversion here (that is task 5.2).

- [ ] **Step 4: Run the tests to verify they pass**
Run: `cargo test --test lead_crud --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**
Change this line in `openspec/changes/add-customer-management-system/tasks.md`:
```
- [ ] 3.1 Implement create, view, update, and list flows for leads with required fields for name, phone, company, owner, and status, plus optional email, notes, and address
```
to:
```
- [x] 3.1 Implement create, view, update, and list flows for leads with required fields for name, phone, company, owner, and status, plus optional email, notes, and address
```