## Requirements

### Requirement: Users can convert eligible leads into customers
The system SHALL allow authenticated users to convert a lead into a customer record when the lead status is `new`, `assigned`, or `in_progress`. The lead owner or an administrator can perform the conversion.

#### Scenario: Convert an eligible lead
- **WHEN** the lead owner or an administrator converts a lead with status `new`, `assigned`, or `in_progress`
- **THEN** the system creates a customer record with fields copied from the lead (name, phone, company, email, notes, address, owner)
- **AND** the system marks the lead status as `converted`
- **AND** the system creates a conversion record in the follow-up history

#### Scenario: Reject conversion of already converted lead
- **WHEN** a user attempts to convert a lead with status `converted`
- **THEN** the system rejects the conversion request

#### Scenario: Reject conversion of closed lead
- **WHEN** a user attempts to convert a lead with status `closed`
- **THEN** the system rejects the conversion request

#### Scenario: Non-owner cannot convert lead
- **WHEN** a user who is neither the lead owner nor an administrator attempts to convert a lead
- **THEN** the system rejects the conversion request

### Requirement: Lead conversion preserves history
The system SHALL preserve lead history and make the converted lead accessible through status filtering.

#### Scenario: Converted lead excluded from default list
- **WHEN** an authenticated user opens the lead list without filters
- **THEN** the system does not include converted leads in the results

#### Scenario: Converted lead accessible through filtering
- **WHEN** an authenticated user filters by status `converted`
- **THEN** the system shows the converted lead record

#### Scenario: Conversion creates follow-up record
- **WHEN** a lead is successfully converted
- **THEN** the system creates a follow-up entry with type `conversion` linking the lead and customer
