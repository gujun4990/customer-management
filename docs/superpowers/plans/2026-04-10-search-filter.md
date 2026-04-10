# Search and Filtering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 leads 和 customers 的搜索与过滤功能

**Architecture:** 在现有的 list_leads 和 list_customers 基础上添加搜索过滤参数

**Tech Stack:** Tauri + Rust + SQLx + PostgreSQL

---

### Task 1: 添加 lead 搜索过滤功能

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs` - 修改 list_leads_for_session

- [ ] **Step 1: 修改 list_leads_for_session 添加搜索过滤参数**

参数:
- `search: Option<String>` - 搜索 name, company, email, phone
- `owner_id: Option<String>` - 按 owner 过滤
- `status: Option<String>` - 按 status 过滤

SQL WHERE 条件:
```sql
WHERE ($1::text IS NULL OR name ILIKE $1 OR company ILIKE $1 OR email ILIKE $1 OR phone ILIKE $1)
  AND ($2::text IS NULL OR owner_id = $2)
  AND ($3::text IS NULL OR status::text = $3)
```

- [ ] **Step 2: 修改 list_leads Tauri command 添加参数**

添加 search, owner_id, status 参数并传递给函数

- [ ] **Step 3: 运行 cargo build 验证编译**

- [ ] **Step 4: Commit**

---

### Task 2: 添加 customer 搜索过滤功能

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs` - 修改 list_customers_for_session

- [ ] **Step 1: 修改 list_customers_for_session 添加搜索过滤参数**

与 lead 相同的模式

- [ ] **Step 2: 修改 list_customers Tauri command 添加参数**

- [ ] **Step 3: 运行 cargo build 验证编译**

- [ ] **Step 4: Commit**

---

### Task 3: 更新 tasks.md 状态

- [ ] **Step 1: 将 task 5.1 标记为完成**

- [ ] **Step 2: 继续 task 5.2 (lead conversion)**