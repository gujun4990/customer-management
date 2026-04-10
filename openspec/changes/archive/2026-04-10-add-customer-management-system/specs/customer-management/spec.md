## Requirements

### Requirement: Users can manage customers
The system SHALL allow authenticated users to create, view, update, and list customer records. Each customer MUST have `name`, `phone`, `company`, `owner`, and status. Each customer MAY have `email`, `notes`, and `address`. Customer status MUST be one of: `active`, `silent`, `lost`.

#### Scenario: Create a customer
- **WHEN** an authenticated user submits a valid customer record
- **THEN** the system creates the customer and makes it available in the customer list

#### Scenario: View a customer
- **WHEN** an authenticated user requests a specific customer by ID
- **THEN** the system returns the customer record with all its fields

#### Scenario: Update a customer
- **WHEN** an authenticated user saves valid changes to an existing customer
- **THEN** the system persists the updated customer details

#### Scenario: View the customer list
- **WHEN** an authenticated user opens the customer list
- **THEN** the system returns customer records with key summary fields

### Requirement: Customers support ownership and follow-up history
The system SHALL allow each customer to have an assigned owner and a follow-up history. Only the customer owner or an administrator can create follow-ups or reassign the owner.

#### Scenario: Assign a customer owner
- **WHEN** the customer owner or an administrator assigns a new owner to a customer
- **THEN** the system stores the new owner on the customer record

#### Scenario: Restricted customer owner reassignment
- **WHEN** a user who is neither the current owner nor an administrator attempts to change the customer owner
- **THEN** the system rejects the reassignment request

#### Scenario: Add a customer follow-up
- **WHEN** the customer owner or an administrator submits a valid follow-up entry on a customer
- **THEN** the system stores the follow-up entry in the customer's history

#### Scenario: View customer follow-up history
- **WHEN** an authenticated user requests follow-up entries for a customer
- **THEN** the system returns the follow-up entries ordered by time, newest first
