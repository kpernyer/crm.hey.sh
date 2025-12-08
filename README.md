# CRM.HEY.SH

A modern, founder-focused CRM & Campaign Engine built with Rust, React, and SurrealDB.

## Tech Stack

- **Backend**: Rust (Axum) with SurrealDB
- **Frontend Web**: Next.js / React with Tailwind CSS
- **Mobile**: React Native (iOS & Android)
- **Database**: SurrealDB
- **Infrastructure**: Docker, Kubernetes (GKE), Terraform
- **CI/CD**: GitHub Actions

## Project Structure

```
crm.hey.sh/
├── backend/           # Rust/Axum API server
│   ├── src/
│   │   ├── handlers/  # HTTP route handlers
│   │   ├── models/    # Data models
│   │   ├── services/  # Business logic
│   │   └── ai/        # AI integration stubs
│   └── schema/        # SurrealDB schema
├── frontend/          # Next.js web application
│   └── src/
│       ├── app/       # App router pages
│       ├── components/# React components
│       └── lib/       # API client & utilities
├── mobile/            # React Native app
│   └── src/
│       ├── screens/   # App screens
│       ├── navigation/# Navigation config
│       └── lib/       # Shared utilities
├── proto/             # Protobuf definitions (gRPC)
├── infra/
│   ├── k8s/          # Kubernetes manifests
│   └── terraform/    # GCP infrastructure
└── .github/workflows/ # CI/CD pipelines
```

## Features

- **Contacts & Companies**: Full CRM with contact management, company associations, and engagement scoring
- **Timeline**: Unified interaction timeline per contact
- **Campaigns**: Multi-channel campaign builder (email, social, landing pages, events)
- **Events**: Event management with RSVP tracking
- **Analytics**: Dashboard with campaign performance, funnel metrics, and engagement tracking
- **AI Integration**: Stubs for AI-powered content generation (email, social posts, landing pages)

## Getting Started

### Prerequisites

- Rust 1.75+
- Bun 1.0+ (for frontend)
- Node.js 20+ (for mobile)
- Docker & Docker Compose
- SurrealDB (or use Docker)
- [just](https://github.com/casey/just) command runner

### Using Just (Recommended)

This monorepo uses `just` for build orchestration. Install it first:

```bash
brew install just
# or: cargo install just
```

Common commands from the repo root:

```bash
just --list        # Show all available commands
just build         # Build all components
just test          # Test all components
just check         # Lint/check all components
just ci            # Run full CI pipeline
just install       # Install all dependencies
```

Per-component commands:

```bash
just build-backend     # Build backend only
just build-frontend    # Build frontend only
just test-backend      # Test backend only
just dev-frontend      # Start frontend dev server
just run-backend       # Run backend server
```

Each subdirectory has its own Justfile for local development:

```bash
cd backend && just --list   # See backend-specific commands
cd frontend && just --list  # See frontend-specific commands
cd mobile && just --list    # See mobile-specific commands
cd infra && just --list     # See infra/deploy commands
```

### Local Development

1. **Start SurrealDB**:
```bash
docker run -d --name surrealdb -p 8000:8000 surrealdb/surrealdb:latest start --user root --pass root
```

2. **Run Backend**:
```bash
cd backend
cp .env.example .env
cargo run
```

3. **Run Frontend**:
```bash
cd frontend
bun install
bun run dev
```

4. **Run Mobile** (requires React Native setup):
```bash
cd mobile
bun install
bun run ios  # or bun run android
```

### Using Docker Compose

Start all services:
```bash
docker-compose up -d
```

Services will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- SurrealDB: http://localhost:8000

## API Endpoints

### Contacts
- `GET /api/contacts` - List contacts
- `POST /api/contacts` - Create contact
- `GET /api/contacts/:id` - Get contact
- `PATCH /api/contacts/:id` - Update contact
- `DELETE /api/contacts/:id` - Delete contact
- `GET /api/contacts/:id/timeline` - Get contact timeline

### Companies
- `GET /api/companies` - List companies
- `POST /api/companies` - Create company
- `GET /api/companies/:id` - Get company
- `PATCH /api/companies/:id` - Update company
- `DELETE /api/companies/:id` - Delete company

### Campaigns
- `GET /api/campaigns` - List campaigns
- `POST /api/campaigns` - Create campaign
- `GET /api/campaigns/:id` - Get campaign
- `PATCH /api/campaigns/:id` - Update campaign
- `POST /api/campaigns/:id/assets` - Generate campaign assets
- `POST /api/campaigns/:id/execute` - Execute campaign

### Events
- `GET /api/events` - List events
- `POST /api/events` - Create event
- `GET /api/events/:id` - Get event
- `POST /api/events/:id/invite` - Invite contacts
- `POST /api/events/:id/rsvp` - RSVP to event

### Landing Pages
- `POST /api/landing-pages/generate` - Generate landing page
- `GET /lp/:id` - View landing page
- `POST /lp/:id/submit` - Submit form

### Analytics
- `GET /api/analytics/contacts` - Contact analytics
- `GET /api/analytics/campaign/:id` - Campaign analytics
- `GET /api/analytics/funnel` - Funnel analytics

## Deployment

### GCP/GKE Setup

1. **Configure Terraform**:
```bash
cd infra/terraform
cp terraform.tfvars.example terraform.tfvars
# Edit terraform.tfvars with your GCP project details
```

2. **Apply Infrastructure**:
```bash
terraform init
terraform plan
terraform apply
```

3. **Configure Kubernetes Secrets**:
```bash
cd infra/k8s
cp secrets.yaml.example secrets.yaml
# Edit secrets.yaml with your credentials
kubectl apply -f secrets.yaml
```

4. **Deploy**:
```bash
kubectl apply -f infra/k8s/
```

### GitHub Actions

Required secrets:
- `GCP_PROJECT_ID` - Your GCP project ID
- `GCP_SA_KEY` - Service account JSON key with appropriate permissions

## Environment Variables

### Backend
- `SURREALDB_URL` - SurrealDB connection URL
- `SURREALDB_NAMESPACE` - Database namespace
- `SURREALDB_DATABASE` - Database name
- `SURREALDB_USER` - Database username
- `SURREALDB_PASS` - Database password
- `JWT_SECRET` - JWT signing secret
- `RUST_LOG` - Log level (info, debug, trace)

### Frontend
- `NEXT_PUBLIC_API_URL` - Backend API URL

## License

Private - All rights reserved.
