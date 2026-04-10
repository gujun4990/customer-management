## Requirements

### Requirement: Users can manage leads
The system SHALL allow authenticated users to create, view, update, and list lead records. Each lead MUST have `name`, `phone`, `company`, `owner`, and status. Each lead MAY have `email`, `notes`, and `address`. Lead status MUST be one of: `new`, `assigned`, `in_progress`, `converted`, `closed`.

#### Scenario: Create a lead
- **WHEN** an authenticated user submits a valid lead record
- **THEN** the system creates the lead and makes it available in the lead list

#### Scenario: View a lead
- **WHEN** an authenticated user requests a specific lead by ID
- **THEN** the system returns the lead record with all its fields

#### Scenario: Update a lead
- **WHEN** an authenticated user saves valid changes to an existing lead
- **THEN** the system persists the updated lead details

#### Scenario: View the lead list
- **WHEN** an authenticated user opens the lead list
- **THEN** the system returns lead records with key summary fields

### Requirement: Leads support ownership and follow-up history
The system SHALL allow each lead to have an assigned owner and a follow-up history. Only the lead owner or an administrator can create follow-ups or reassign the owner.

#### Scenario: Assign a lead owner
- **WHEN** the lead owner or an administrator assigns a new owner to a lead
- **THEN** the system stores the new owner on the lead record

#### Scenario: Restricted owner reassignment
- **WHEN** a user who is neither the current owner nor an administrator attempts to change the lead owner
- **THEN** the system rejects the reassignment request

#### Scenario: Add a lead follow-up
- **WHEN** the lead owner or an administrator submits a valid follow-up entry on a lead
- **THEN** the system stores the follow-up entry in the lead's history

#### Scenario: View lead follow-up history
- **WHEN** an authenticated user requests follow-up entries for a lead
- **THEN** the system returns the follow-up entries ordered by time, newest first

### Requirement: Converted leads remain accessible through filtering
The system SHALL preserve converted lead records and allow users to review them through lead status filtering.

#### Scenario: View converted leads through filtering
- **WHEN** an authenticated user filters the lead list by status `converted`
- **THEN** the system shows converted lead records

#### Scenario: Default lead list excludes converted leads
- **WHEN** an authenticated user opens the lead list without filters
- **THEN** the system does not include converted leads in the results
