# Task 1.2 CRM Data Model Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Define the minimal Rust CRM domain model for users, leads, customers, and follow-up records with fixed lead and customer status enums.

**Architecture:** Keep the model layer inside `customer-management/src-tauri/src/` so the Tauri backend owns the source-of-truth domain contracts before persistence exists. Add one focused Rust module for the domain types and test it with Rust unit tests so later authentication, persistence, and commands can build on stable types.

**Tech Stack:** Rust, Tauri 2, Serde

---

### Task 1: Add the CRM domain model module

**Files:**
- Create: `customer-management/src-tauri/src/domain.rs`
- Modify: `customer-management/src-tauri/src/lib.rs`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing Rust tests**

Add unit tests that prove:
- `LeadStatus::all()` returns `new`, `assigned`, `in_progress`, `converted`, `closed`
- `CustomerStatus::all()` returns `active`, `silent`, `lost`
- `LeadRecord::new(...)` stores required and optional fields correctly
- `CustomerRecord::new(...)` stores required and optional fields correctly
- `FollowUpRecord::new_for_lead(...)` and `new_for_customer(...)` preserve target identity and optional next-follow-up time

- [ ] **Step 2: Run the Rust tests to verify they fail**

Run: `cargo test domain --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: FAIL because the `domain` module and model types do not exist yet.

- [ ] **Step 3: Implement the minimal domain types**

Create a small Rust module that defines:
- `UserRecord`
- `LeadStatus`
- `CustomerStatus`
- `LeadRecord`
- `CustomerRecord`
- `FollowUpTarget`
- `FollowUpRecord`

Use `String` for IDs and timestamps for now so persistence decisions stay in task `1.3`.

- [ ] **Step 4: Run the Rust tests to verify they pass**

Run: `cargo test domain --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**

Change this line in `openspec/changes/add-customer-management-system/tasks.md`:

```md
- [ ] 1.2 Define the minimal data model for users, leads, customers, and follow-up records, including fixed lead and customer status enums
```

to:

```md
- [x] 1.2 Define the minimal data model for users, leads, customers, and follow-up records, including fixed lead and customer status enums
```
