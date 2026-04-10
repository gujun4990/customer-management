# Customer Follow-up Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现客户跟进记录的创建(create_customer_followup)和列表显示(list_customer_followups)功能

**Architecture:** 基于已有的 lead follow-up 实现模式，为 customer 添加相同功能

**Tech Stack:** Tauri + Rust + SQLx + PostgreSQL

---

### Task 1: 实现 create_customer_followup_for_session

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs:762-821`

- [ ] **Step 1: 编写失败的测试**

```rust
#[tokio::test]
async fn test_create_customer_followup_not_owner() {
    // 测试非owner用户不能创建customer follow-up
}
```

- [ ] **Step 2: 基于 lead follow-up 模式实现 create_customer_followup_for_session**

参考 `create_lead_followup_for_session` (line 762-821)，复制并修改:
- 查询 customer 而非 lead
- 使用 `target_type = 'customer'` 和 `customer_id` 字段
- 使用 `FollowUpRecord::new_for_customer` 构造返回

```rust
pub async fn create_customer_followup_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
    content: &str,
    follow_up_time: &str,
    method: &str,
    result: &str,
    next_follow_up_time: Option<&str>,
) -> Result<FollowUpRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let customer = sqlx::query_as::<_, CustomerRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM customers
        JOIN users ON users.id = customers.owner_id
        WHERE customers.id = $1
        "#,
    )
    .bind(customer_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?
    .ok_or_else(CrmError::data_unavailable)?;

    if customer.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_followup());
    }

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO follow_ups (id, target_type, customer_id, content, follow_up_time, method, result, next_follow_up_time)
        VALUES ($1, 'customer', $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&id)
    .bind(customer_id)
    .bind(content)
    .bind(follow_up_time)
    .bind(method)
    .bind(result)
    .bind(next_follow_up_time)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(FollowUpRecord::new_for_customer(
        &id,
        customer_id,
        content,
        follow_up_time,
        method,
        result,
        next_follow_up_time,
    ))
}
```

- [ ] **Step 3: 运行 cargo build 验证编译**

Run: `cd customer-management/src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add customer-management/src-tauri/src/crm.rs
git commit -m "feat: add create_customer_followup_for_session"
```

---

### Task 2: 实现 list_customer_followups_for_session

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs:823-857`

- [ ] **Step 1: 基于 lead follow-up 模式实现 list_customer_followups_for_session**

参考 `list_lead_followups_for_session` (line 823-857)，复制并修改:
- 查询 target_type = 'customer' 的记录
- 使用 customer_id 而非 lead_id

```rust
pub async fn list_customer_followups_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
) -> Result<Vec<FollowUpRecord>, CrmError> {
    ensure_authenticated_session(session).map_err(|_| CrmError::authentication_required())?;

    let rows = sqlx::query_as::<_, FollowUpRow>(
        r#"
        SELECT id, target_type, COALESCE(lead_id, customer_id) AS target_id, 
               content, follow_up_time, method, result, next_follow_up_time
        FROM follow_ups
        WHERE target_type = 'customer' AND customer_id = $1
        ORDER BY follow_up_time DESC
        "#,
    )
    .bind(customer_id)
    .fetch_all(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    rows.into_iter()
        .map(|row| {
            Ok(FollowUpRecord {
                id: row.id,
                target: FollowUpTarget::Customer(row.target_id),
                content: row.content,
                follow_up_time: row.follow_up_time,
                method: row.method,
                result: row.result,
                next_follow_up_time: row.next_follow_up_time,
            })
        })
        .collect()
}
```

- [ ] **Step 2: 运行 cargo build 验证编译**

Run: `cd customer-management/src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add customer-management/src-tauri/src/crm.rs
git commit -m "feat: add list_customer_followups_for_session"
```

---

### Task 3: 导出 Tauri command

**Files:**
- Modify: `customer-management/src-tauri/src/crm.rs:859-1061`
- Modify: `customer-management/src-tauri/src/lib.rs:25-41`

- [ ] **Step 1: 添加 create_customer_followup Tauri command**

在 crm.rs 末尾添加类似 `create_lead_followup` 的 command:

```rust
#[tauri::command]
pub async fn create_customer_followup(
    state: tauri::State<'_, AppState>,
    customer_id: String,
    content: String,
    follow_up_time: String,
    method: String,
    result: String,
    next_follow_up_time: Option<String>,
) -> Result<FollowUpRecord, String> {
    create_customer_followup_for_session(
        &state.pool,
        &state.session,
        &customer_id,
        &content,
        &follow_up_time,
        &method,
        &result,
        next_follow_up_time.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())
}
```

- [ ] **Step 2: 添加 list_customer_followups Tauri command**

在 crm.rs 末尾添加:

```rust
#[tauri::command]
pub async fn list_customer_followups(
    state: tauri::State<'_, AppState>,
    customer_id: String,
) -> Result<Vec<FollowUpRecord>, String> {
    list_customer_followups_for_session(&state.pool, &state.session, &customer_id)
        .await
        .map_err(|e| e.to_string())
}
```

- [ ] **Step 3: 在 lib.rs 中注册 command handlers**

在 `generate_handler!` 中添加:
- `crm::create_customer_followup`
- `crm::list_customer_followups`

```rust
.invoke_handler(tauri::generate_handler![
    // ... existing handlers ...
    crm::create_customer_followup,
    crm::list_customer_followups,
])
```

- [ ] **Step 4: 运行 cargo build 验证编译**

Run: `cd customer-management/src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add customer-management/src-tauri/src/crm.rs customer-management/src-tauri/src/lib.rs
git commit -m "feat: add customer follow-up Tauri commands"
```

---

### Task 4: 更新 tasks.md 状态

- [ ] **Step 1: 将 task 4.3 标记为完成**

在 `openspec/changes/add-customer-management-system/tasks.md` 中:
- [x] 4.3 Implement customer follow-up entry creation and timeline display with content, time, method, result, and next-follow-up time