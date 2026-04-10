# Lead-to-Customer Conversion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现线索转客户(lead-to-customer conversion)功能

**Architecture:** 将 lead 转换为 customer，记录转换历史，默认从活跃线索列表中排除已转换线索

**Tech Stack:** Tauri + Rust + SQLx + PostgreSQL

---

### Task 1: 编写失败的测试

**Files:**
- Create: `customer-management/src-tauri/tests/lead_conversion.rs`

测试用例：
1. `test_convert_lead_to_customer` - 线索转客户基本功能
2. `test_converted_lead_excluded_from_active_list` - 已转换线索不在默认列表
3. `test_converted_lead_available_with_status_filter` - 通过状态过滤可查看已转换线索
4. `test_non_convertible_lead_rejected` - 非可转换状态拒绝转换

---

### Task 2: 实现转换功能

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs`

- [ ] **Step 1: 实现 convert_lead_to_customer_for_session 函数**

参数: pool, session, lead_id, initial_status
- 验证 lead 存在且状态为可转换 (new/assigned/in_progress)
- 创建 customer 记录，映射字段: name, phone, company, email, notes, address, owner
- 记录转换历史到 follow_ups
- 更新 lead 状态为 converted
- 返回创建的 customer 记录

- [ ] **Step 2: 添加 Tauri command**

```rust
#[tauri::command]
pub async fn convert_lead_to_customer(
    state: tauri::State<'_, AppState>,
    lead_id: String,
    initial_status: String,
) -> Result<CustomerRecord, String>
```

- [ ] **Step 3: 运行 cargo build 验证编译**

- [ ] **Step 4: 运行测试验证通过**

---

### Task 3: 更新 tasks.md 状态

- [ ] **Step 1: 将 task 5.2 标记为完成**

- [ ] **Step 2: 继续 task 6.x (Verification)**