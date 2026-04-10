## Requirements

### Requirement: Internal users can sign in with a simple login flow
The system SHALL allow internal users to sign in using administrator-precreated username and password accounts.

#### Scenario: Successful sign-in
- **WHEN** a user submits valid administrator-precreated username and password credentials
- **THEN** the system authenticates the user and establishes an authenticated session

#### Scenario: Rejected invalid credentials
- **WHEN** a user submits invalid credentials
- **THEN** the system rejects the sign-in attempt with a generic authentication failure response

### Requirement: CRM features require authentication
The system SHALL reject unauthenticated access to lead and customer management features.

#### Scenario: Unauthenticated user requests CRM data
- **WHEN** an unauthenticated request is made to a protected CRM feature
- **THEN** the system rejects the request and does not expose lead or customer data
