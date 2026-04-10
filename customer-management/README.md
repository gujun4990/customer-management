# Customer Management System

A minimal CRM system built with Tauri, React, and PostgreSQL.

## Features

- **Lead Management**: Create, view, update, list leads with owner assignment and status tracking
- **Customer Management**: Manage customer records with ownership and lifecycle status
- **Follow-up Tracking**: Record follow-up activities on both leads and customers
- **Lead-to-Customer Conversion**: Convert eligible leads to customers while preserving history
- **Search & Filtering**: Search by name, company, email, phone; filter by owner and status

## Tech Stack

- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Rust + Tauri 2
- **Database**: PostgreSQL + SQLx

## Quick Start

### Prerequisites

- Node.js 20+
- Rust stable
- PostgreSQL 14+

### Database Setup

```bash
# Create database
createdb customer_management

# Set environment variable
export DATABASE_URL="postgresql://user:password@localhost/customer_management"
```

### Development

```bash
# Frontend
cd customer-management
npm install
npm run dev

# Backend (in another terminal)
cd customer-management/src-tauri
cargo run
```

### Build

```bash
# Build frontend
cd customer-management
npm run build

# Build Tauri app
cd customer-management
npm run tauri build
```

## Testing

```bash
# Run Rust tests
cd customer-management/src-tauri
TEST_DATABASE_URL="postgresql://postgres:postgres@localhost/customer_management" cargo test
```

## Project Structure

```
customer-management/
├── src/                    # React frontend
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── auth.rs         # Authentication
│   │   ├── crm.rs          # CRM business logic
│   │   ├── domain.rs       # Domain models
│   │   ├── storage.rs      # Database connection
│   │   └── lib.rs          # Tauri commands
│   ├── migrations/         # Database migrations
│   └── tests/              # Integration tests
└── package.json
```

## Default Users

The system comes with seeded users:

| Username | Password | Role   |
|----------|----------|--------|
| admin    | admin123 | Admin  |
| sales    | sales123 | User   |
| alice    | alice123 | User   |
| bob      | bob123   | User   |

## API Commands

- `sign_in` - Login with username/password
- `current_user` - Get current user
- `list_leads` - List leads (supports search, owner_id, status filters)
- `create_lead` - Create new lead
- `get_lead` - Get lead by ID
- `update_lead` - Update lead
- `reassign_lead_owner` - Reassign lead owner
- `create_lead_followup` - Add follow-up to lead
- `list_lead_followups` - List lead follow-ups
- `list_customers` - List customers (supports search, owner_id, status filters)
- `create_customer` - Create new customer
- `get_customer` - Get customer by ID
- `update_customer` - Update customer
- `reassign_customer_owner` - Reassign customer owner
- `create_customer_followup` - Add follow-up to customer
- `list_customer_followups` - List customer follow-ups
- `convert_lead_to_customer` - Convert lead to customer

## License

MIT