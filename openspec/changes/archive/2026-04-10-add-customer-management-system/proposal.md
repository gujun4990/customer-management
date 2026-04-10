## Why

The team needs a minimal usable CRM foundation that supports real day-to-day work instead of scattered lead and customer information. The first release should prove the core business loop works end to end: log in, create a lead, assign an owner, record follow-ups, convert the lead into a customer, and continue maintaining the customer record.

## What Changes

- Add simple login so team members can access the system with named accounts.
- Add lead management for creating, viewing, updating, and searching leads.
- Add customer management for creating, viewing, updating, and searching customers.
- Add owner assignment on leads and customers to support basic multi-user collaboration.
- Add follow-up records on leads and customers so activity history is preserved.
- Add lead-to-customer conversion while preserving required historical information.
- Keep the implementation intentionally small so it can serve as the base for later sales and after-sales expansion.

## Scope

- Admin-created preset username and password login only.
- Lead records and customer records as separate business objects.
- Search and structured filtering over agreed core fields.
- Follow-up history on both leads and customers.
- Owner assignment for collaboration.
- Lead conversion into customer records.
- Converted leads remain queryable through lead status filtering.
- Optional profile fields are limited to email, notes, and address in phase one.

## Non-Goals

- Complex roles, permissions, or approval workflows.
- Opportunity, order, or contract management.
- Automation, reminders, or reporting.
- External integrations.
- Broad CRM customization beyond the minimum required data fields.

## Capabilities

### New Capabilities
- `simple-auth`: Let team members sign in with a minimal authentication flow suitable for an internal MVP.
- `lead-management`: Manage lead records, ownership, status, and follow-up history before conversion.
- `customer-management`: Manage customer records, ownership, status, and follow-up history after conversion.
- `crm-search`: Search and filter leads and customers by core business attributes.
- `lead-conversion`: Convert a lead into a customer while preserving required record history.

### Modified Capabilities
- None.

## Risks

- Scope may expand into a full CRM if opportunities, contracts, reporting, or automation are added too early.
- A weak definition of simple login can create confusion around ownership and collaboration.
- If lead and customer boundaries are unclear, conversion behavior and later reporting will become unstable.
- If follow-up fields are underspecified, the first implementation may need avoidable schema changes.
- If status vocabularies are allowed to expand during implementation, the MVP may become harder to validate consistently.
- If owner reassignment rules are unclear, multi-user collaboration can become inconsistent.

## Impact

- Establishes the first operational CRM slice in an empty workspace.
- Requires foundational domain models for users, leads, customers, and follow-up records.
- Requires list/detail flows, search/filter behavior, and lead conversion handling.
