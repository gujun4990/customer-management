## 1. Project Foundation

- [x] 1.1 Choose the phase-one stack and initialize the smallest project structure needed to run the CRM MVP
- [x] 1.2 Define the minimal data model for users, leads, customers, and follow-up records, including fixed lead and customer status enums
- [x] 1.3 Add persistence setup and seed data for administrator-precreated internal users and sample CRM records

## 2. Authentication

- [x] 2.1 Implement preset username/password login and current-user session handling
- [x] 2.2 Protect CRM actions so unauthenticated users cannot access lead or customer data

## 3. Lead Management

- [x] 3.1 Implement create, view, update, and list flows for leads with required fields for name, phone, company, owner, and status, plus optional email, notes, and address
- [x] 3.2 Add lead owner assignment, restricted reassignment rules, and lead status handling for new, assigned, in-progress, converted, and closed
- [x] 3.3 Implement lead follow-up entry creation and timeline display with content, time, method, result, and next-follow-up time

## 4. Customer Management

- [x] 4.1 Implement create, view, update, and list flows for customers with required fields for name, phone, company, owner, and status, plus optional email, notes, and address
- [x] 4.2 Add customer owner assignment, restricted reassignment rules, and customer status handling for active, silent, and lost
- [x] 4.3 Implement customer follow-up entry creation and timeline display with content, time, method, result, and next-follow-up time

## 5. Search And Conversion

- [x] 5.1 Implement structured search and filtering over the agreed lead and customer fields, including viewing converted leads through status filtering
- [x] 5.2 Implement lead-to-customer conversion with required field mapping, history preservation, and default exclusion of converted leads from the active lead list

## 6. Verification

- [x] 6.1 Add automated tests for login, unauthenticated access rejection, and current-user resolution
- [x] 6.2 Add automated tests for lead and customer CRUD, owner assignment, and follow-up history behavior
- [x] 6.3 Add automated tests for search/filter behavior and lead conversion rules
- [x] 6.4 Verify the end-to-end business flow from sign-in to ongoing customer follow-up
