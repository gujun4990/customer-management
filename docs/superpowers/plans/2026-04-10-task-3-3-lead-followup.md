# Task 3.3 Lead Follow-Up Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement lead follow-up entry creation and timeline display.

**Architecture:** Add follow-up functions to existing Tauri backend. Follow-ups store content, time, method, result, and optional next-follow-up time. Timeline shows entries sorted by newest first.

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL

---

### Task 1: Add lead follow-up operations

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs`
- Create: `customer-management/src-tauri/tests/lead_followup.rs`

- [ ] **Step 1: Write the failing follow-up tests**

Add tests that prove:
- A follow-up can be created for a lead.
- Follow-ups can be retrieved for a lead.
- Timeline returns follow-ups sorted by newest first.
- Required fields are validated.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test lead_followup`
Expected: FAIL

- [ ] **Step 3: Implement follow-up functions**

Add to `crm.rs`:
- `create_lead_followup_for_session` - create follow-up entry
- `list_lead_followups_for_session` - get timeline
- Tauri commands

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test lead_followup`
Expected: PASS

- [ ] **Step 5: Mark task complete**

Change this line in `tasks.md`:
```
- [ ] 3.3 Implement lead follow-up entry creation and timeline display
```
to:
```
- [x] 3.3 Implement lead follow-up entry creation and timeline display
```