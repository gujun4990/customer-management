## Context

This change defines the smallest useful CRM foundation for the project. The product goal is not a full CRM platform; it is a narrow, usable workflow that supports internal team members managing leads and customers together in one system.

The first release must support these business steps:

1. A user signs in.
2. A user creates or updates a lead.
3. A lead is assigned to an owner.
4. The owner records follow-up activity.
5. The lead is converted into a customer.
6. The customer record continues to be maintained and followed up.

The biggest constraint is scope control. The design must leave room for later expansion without adding advanced CRM features now.

## Goals / Non-Goals

**Goals:**
- Provide a simple authenticated entry point for internal users.
- Keep leads and customers as separate but related business objects.
- Preserve follow-up history instead of overwriting it.
- Support basic multi-user collaboration through owner assignment.
- Make the lead-to-customer flow explicit and testable.

**Non-Goals:**
- Role-based access control, approval chains, or complex visibility rules.
- Opportunity, order, contract, or billing flows.
- Notifications, automation, dashboards, or analytics.
- External system synchronization.

## Architecture

The MVP should use a small three-part architecture:

- `auth`: handles simple login and current-user resolution.
- `crm domain`: handles leads, customers, owners, follow-ups, and lead conversion rules.
- `delivery layer`: exposes the system through HTTP endpoints and minimal list/detail pages or equivalent UI flows.

This separation keeps authentication concerns away from CRM rules, and keeps business rules away from transport or UI code.

## Modules

### Authentication module
- Sign in with administrator-precreated username and password accounts.
- Resolve the current user for create/update/assign/follow-up actions.

### Lead module
- Create, view, update, and search leads.
- Track owner and lead status.
- Attach follow-up records.
- Allow owner reassignment only for the current owner or an administrator.

### Customer module
- Create, view, update, and search customers.
- Track owner and customer lifecycle status.
- Attach follow-up records.
- Allow owner reassignment only for the current owner or an administrator.

### Conversion module
- Convert a lead into a customer.
- Copy or map approved fields from lead to customer.
- Preserve required historical context so the conversion is auditable in-product.

### Search module
- Support deterministic search and filtering on agreed fields such as name, contact info, owner, and status.
- Work across leads and customers without introducing full-text search complexity.

## Data Model

The MVP should use a minimal relational shape:

- `users`: internal system users who can sign in.
- `leads`: pre-customer records with owner, status, and contact fields.
- `customers`: managed customer records with owner, status, and contact fields.
- `follow_ups`: append-only activity records linked to either a lead or a customer.

Key design choices:
- Leads and customers stay separate to keep conversion explicit.
- Follow-ups stay separate from lead and customer records so history is preserved cleanly.
- Owner is stored directly on leads and customers because owner assignment is the core collaboration mechanism.
- User accounts are provisioned by an administrator up front, so phase one does not need self-service registration or password recovery.

Required status vocabularies:
- Lead status: `new`, `assigned`, `in_progress`, `converted`, `closed`
- Customer status: `active`, `silent`, `lost`

Minimum required lead and customer fields:
- `name`
- `phone`
- `company`
- `owner_id`
- `status`

Supported optional lead and customer fields for phase one:
- `email`
- `notes`
- `address`

Minimum follow-up fields:
- `content`
- `follow_up_time`
- `method`
- `result`
- `next_follow_up_time`

## Call Flow

### Sign-in flow
1. User submits credentials.
2. Auth module validates credentials.
3. System establishes the current session or equivalent auth state.
4. CRM actions are executed in the context of the signed-in user.

### Lead follow-up flow
1. User opens a lead record.
2. User submits a follow-up entry.
3. CRM domain validates the record target and input fields.
4. System stores the follow-up as a new history entry.
5. Lead detail view reloads with newest activity first.

### Lead conversion flow
1. User opens a lead record.
2. User triggers conversion.
3. Conversion module validates that the lead is convertible.
4. System creates a customer record from the approved mapped fields.
5. System marks the lead as converted and excludes it from the default active lead list.
6. System preserves required conversion history for later review.

### Converted lead review flow
1. User opens the lead list.
2. User applies the lead status filter for `converted`.
3. System returns converted lead records without mixing them into the default active lead view.

## Configuration

The MVP should keep configuration minimal:

- Authentication configuration for administrator-precreated accounts.
- Application environment settings for persistence and runtime.
- Seed or bootstrap configuration for initial internal users.
- Fixed default status configuration for newly created leads and customers if the implementation keeps them configurable.

Configuration should avoid feature toggles or broad customization in phase one.

## Error Handling

The system should handle a small, explicit set of errors consistently:

- Authentication failure: reject sign-in with a generic invalid-credentials response.
- Validation failure: reject missing or malformed lead, customer, owner, or follow-up input with field-level errors.
- Not found: return a consistent response when a requested lead, customer, or user does not exist.
- Conversion conflict: reject lead conversion when the record is already converted or missing required data.
- Authorization boundary: if simple login still restricts anonymous access, reject unauthenticated requests consistently.
- Reassignment conflict: reject owner reassignment when the acting user is neither the current owner nor an administrator.

The MVP should prefer simple, deterministic errors over complex recovery logic.

## Testing Strategy

Testing should stay focused on business-loop correctness:

- Unit tests for validation and conversion rules.
- Service or domain tests for owner assignment, follow-up creation, and lead conversion.
- Endpoint or integration tests for sign-in, lead CRUD, customer CRUD, search/filter behavior, and conversion flow.
- One end-to-end test for the full phase-one path: sign in -> create lead -> assign owner -> add follow-up -> convert to customer -> continue follow-up on customer.

The highest-priority tests are the ones that prove the business loop works without opportunities, contracts, reporting, or integrations.

## Risks / Trade-offs

- [Simple login may be too weak for later collaboration needs] -> Keep auth isolated so stronger access controls can be added later.
- [Lead and customer duplication may feel redundant] -> Accept the extra model now because it keeps conversion explicit and easier to reason about.
- [Search expectations may grow into full-text discovery] -> Limit phase one to structured search and filtering over agreed fields.
- [Follow-up schema may expand quickly] -> Start with the smallest useful record shape and keep follow-ups in their own table or model.

## Migration Plan

There is no legacy system to migrate in this workspace. Implementation should begin with authentication and core CRM data models, then add CRUD flows, then add follow-up handling, and finally add conversion and end-to-end verification.

Rollback is straightforward because this change introduces new functionality rather than replacing existing production behavior.

## Open Questions

No remaining critical product-scope questions for phase one.
