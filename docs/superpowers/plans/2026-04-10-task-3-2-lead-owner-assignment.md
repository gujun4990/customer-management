# Task 3.2 Lead Owner Assignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add lead owner assignment, restricted reassignment rules, and lead status handling.

**Architecture:** Extend the existing Tauri backend with owner assignment and reassignment protection. Only the current lead owner or an admin can reassign the lead owner. Use the existing auth session to determine the current user and check their permissions against the lead record.

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL

---

### Task 1: Add lead owner assignment

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs`
- Create: `customer-management/src-tauri/tests/lead_owner.rs`

- [ ] **Step 1: Write the failing lead owner tests**

Add tests that prove:
- A lead can be assigned an owner upon creation.
- The lead owner can be retrieved via the lead record.
- Only the current owner or admin can reassign the owner.
- Non-owner/non-admin reassignment returns an error.

```rust
// Test structure (pseudo-code)
#[test]
fn lead_owner_assignment_requires_auth() {
    // Sign in as non-owner
    // Try to reassign lead
    // Expect error: "only owner or admin can reassign"
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test --test lead_owner --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: FAIL because reassignment protection doesn't exist yet.

- [ ] **Step 3: Implement owner reassignment with permission check**

Add function `reassign_lead_owner` in `crm.rs`:
- Get the authenticated user from session
- Fetch the existing lead to get current owner
- Check if user is admin OR user.id == current_owner.id
- If not authorized, return error
- If authorized, update the owner_id

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --test lead_owner --manifest-path customer-management/src-tauri/Cargo.toml`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**

Change this line in `openspec/changes/add-customer-management-system/tasks.md`:
```
- [ ] 3.2 Add lead owner assignment, restricted reassignment rules, and lead status handling for new, assigned, in-progress, converted, and closed
```
to:
```
- [x] 3.2 Add lead owner assignment, restricted reassignment rules, and lead status handling for new, assigned, in-progress, converted, and closed
```