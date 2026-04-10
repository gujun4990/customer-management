# Task 4.2 Customer Owner Assignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development

**Goal:** Add customer owner assignment, restricted reassignment rules, and customer status handling.

**Architecture:** Similar to Task 3.2 (Lead Owner), but for customers with CustomerStatus (active, silent, lost).

**Tech Stack:** Rust, Tauri 2, SQLx, PostgreSQL

---

### Task 1: Add customer owner reassignment

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs`
- Create: `customer-management/src-tauri/tests/customer_owner.rs`

- [ ] **Step 1: Write failing tests** (RED)
- [ ] **Step 2: Verify tests fail**
- [ ] **Step 3: Implement reassignment function** (GREEN)
- [ ] **Step 4: Verify tests pass**
- [ ] **Step 5: Mark task complete**