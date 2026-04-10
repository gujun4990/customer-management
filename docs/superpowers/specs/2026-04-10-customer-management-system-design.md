## Overview

This document captures the approved first-phase definition for a customer management system. The goal of phase one is not to build a full CRM platform, but to deliver a minimal usable CRM foundation that supports real team workflows and establishes a stable base for later expansion.

The phase-one product focus is a complete operational loop from lead intake to customer maintenance. The system must let a team log in, create and maintain leads and customers, assign ownership, record follow-up activity, search records, and convert leads into customers without losing key history.

## User Goals

- Provide a minimal usable CRM foundation for real team use.
- Support the core business loop from lead to customer.
- Allow team members to log in, collaborate, and track ownership.
- Create a stable domain model that can later expand into sales management and after-sales workflows.

## Scope

Phase one includes the following capabilities:

- Simple login.
- Lead management.
- Customer management.
- Lead-to-customer conversion.
- Follow-up record management.
- Search and filtering.
- Owner assignment for collaboration.
- Basic multi-user usage within one team or organization.

## Non-Goals

Phase one explicitly excludes:

- Complex permission and access-control systems.
- Opportunity, order, and contract management.
- Workflow automation.
- Reporting and analytics.
- External integrations.
- Advanced approval flows and complex organizational hierarchy support.

## Recommended Product Approach

The recommended approach is to build a CRM foundation first rather than a sales-first module or a pure customer-directory tool.

This approach was chosen because it best matches the requirement that the system support sales, customer management, after-sales work, and internal record keeping over time, while still keeping the first phase small enough to complete and validate. The first release should therefore focus on a single closed loop:

1. A user logs in.
2. A user creates a lead.
3. A lead is assigned to an owner.
4. The owner records follow-up activity.
5. The lead is converted into a customer.
6. The customer record continues to be maintained and followed up.

If this loop works cleanly, the phase-one objective is achieved.

## Key Constraints

- The scope must remain limited to a minimal usable CRM foundation.
- The product must support multiple users, but only with simple login.
- Collaboration should rely on owner assignment instead of fine-grained permissions.
- The main success condition is business-loop completeness, not feature breadth.
- The data model must be simple enough to ship quickly and stable enough to support later sales and service extensions.

## Core Domain Boundaries

### Lead

A lead is a pre-customer record that represents a potential customer before formal conversion. It should support core identifying information, contact information, an owner, status, and follow-up history.

### Customer

A customer is a converted or directly created managed account. It should support profile information, owner assignment, lifecycle status, and follow-up history.

### Follow-Up Record

A follow-up record captures team activity against a lead or customer. It should preserve history rather than overwrite it. At minimum, the design assumes note content and time context, with exact required fields still to be finalized.

### Owner

An owner is the responsible user for a lead or customer. Owner assignment is the main collaboration mechanism in phase one.

## Unknowns

- The exact login method is not yet defined.
- The exact rules for owner reassignment are not yet defined.
- The exact field mapping and validation rules during lead-to-customer conversion are not yet defined.
- The minimum required schema for a follow-up record is not yet fully defined.
- The exact search fields and filter dimensions still need to be finalized at implementation-planning time.

## Risks

### Scope Expansion

There is a strong risk that phase one grows into a full CRM platform by absorbing opportunities, contracts, approvals, reminders, analytics, or integrations. This must be actively prevented.

### Ambiguous Access Model

Using simple login with multi-user collaboration can create confusion if ownership, visibility, and edit expectations are not stated clearly.

### Unclear Lead-Customer Boundary

If the distinction between leads and customers is not explicit, the conversion flow and reporting logic will become unstable.

### Weak Acceptance Criteria

If acceptance is defined only as "usable," the team will not have a reliable completion signal. Acceptance must be tied to concrete business flows.

## Acceptance Criteria

Phase one is acceptable when all of the following are true:

- A user can log into the system through the chosen simple login method.
- A user can create, view, and edit leads.
- A user can create, view, and edit customers.
- Each lead and customer can have an assigned owner.
- A user can add follow-up records to both leads and customers.
- A user can search and filter leads or customers by the agreed core fields.
- A user can convert a lead into a customer.
- Lead conversion preserves required historical information.
- A team member can complete the full flow from lead creation to customer conversion and ongoing follow-up without needing opportunities, orders, contracts, reports, or integrations.

## Suggested Next Decisions

Before implementation planning, the next decisions to lock down are:

- Login method.
- Owner assignment and reassignment rules.
- Lead and customer status vocabulary.
- Follow-up record minimum field set.
- Search and filter field set.
