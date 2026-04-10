# Task 1.1 Tauri Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Choose the phase-one Tauri stack and initialize the smallest runnable customer-management project structure for the CRM MVP.

**Architecture:** Keep the workspace root for OpenSpec and docs, and place the application itself in `customer-management/` so implementation files stay isolated from planning artifacts. Use the official Tauri scaffold with the React + TypeScript template to keep the UI layer practical for later CRM forms and list/detail screens while remaining minimal.

**Tech Stack:** Tauri 2, React, TypeScript, Vite, npm, Rust

---

### Task 1: Scaffold the Tauri app foundation

**Files:**
- Create: `tests/task-1.1-tauri-foundation.test.mjs`
- Create: `customer-management/package.json`
- Create: `customer-management/src/`
- Create: `customer-management/src-tauri/`
- Modify: `openspec/changes/add-customer-management-system/tasks.md`

- [ ] **Step 1: Write the failing smoke test**

```js
import test from 'node:test'
import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'

test('task 1.1 scaffolds a tauri react typescript app in customer-management', () => {
  assert.equal(existsSync('customer-management/package.json'), true)
  assert.equal(existsSync('customer-management/src-tauri/tauri.conf.json'), true)

  const pkg = JSON.parse(readFileSync('desktop/package.json', 'utf8'))
  assert.equal(pkg.type, 'module')
  assert.ok(pkg.scripts.dev)
  assert.ok(pkg.scripts.build)
})
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `node --test tests/task-1.1-tauri-foundation.test.mjs`
Expected: FAIL because `customer-management/package.json` and `customer-management/src-tauri/tauri.conf.json` do not exist yet.

- [ ] **Step 3: Scaffold the minimal Tauri app**

Run:

```bash
npm create tauri-app@latest customer-management -- --manager npm --template react-ts --identifier com.customer.management --yes
```

This should create the smallest practical Tauri app for the CRM foundation while keeping project files under `customer-management/`.

- [ ] **Step 4: Run the test to verify it passes**

Run: `node --test tests/task-1.1-tauri-foundation.test.mjs`
Expected: PASS

- [ ] **Step 5: Mark the OpenSpec task complete**

Change this line in `openspec/changes/add-customer-management-system/tasks.md`:

```md
- [ ] 1.1 Choose the phase-one stack and initialize the smallest project structure needed to run the CRM MVP
```

to:

```md
- [x] 1.1 Choose the phase-one stack and initialize the smallest project structure needed to run the CRM MVP
```
