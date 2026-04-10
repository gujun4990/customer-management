## Requirements

### Requirement: Users can search leads and customers
The system SHALL allow authenticated users to search lead and customer records using core business attributes.

#### Scenario: Search leads by name or company
- **WHEN** an authenticated user searches leads by a full or partial name or company
- **THEN** the system returns matching lead records

#### Scenario: Search leads by contact detail
- **WHEN** an authenticated user searches leads by email or phone
- **THEN** the system returns matching lead records

#### Scenario: Search customers by name or company
- **WHEN** an authenticated user searches customers by a full or partial name or company
- **THEN** the system returns matching customer records

#### Scenario: Search customers by contact detail
- **WHEN** an authenticated user searches customers by email or phone
- **THEN** the system returns matching customer records

### Requirement: Users can filter leads and customers
The system SHALL allow authenticated users to filter lead and customer lists by owner and status.

#### Scenario: Filter leads by owner
- **WHEN** an authenticated user filters the lead list by owner
- **THEN** the system shows only leads assigned to that owner

#### Scenario: Filter leads by status
- **WHEN** an authenticated user filters the lead list by status
- **THEN** the system shows only leads with that status

#### Scenario: Filter customers by owner
- **WHEN** an authenticated user filters the customer list by owner
- **THEN** the system shows only customers assigned to that owner

#### Scenario: Filter customers by status
- **WHEN** an authenticated user filters the customer list by status
- **THEN** the system shows only customers with that status

#### Scenario: Clear filters and view all records
- **WHEN** an authenticated user removes all filters from a list
- **THEN** the system returns all records
