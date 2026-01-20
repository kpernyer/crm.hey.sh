# CRM.HEY.SH Project Overview

## Project Mission
CRM.HEY.SH is a modern, founder-focused CRM & Campaign Engine built with Rust, React, and SurrealDB. The platform enables founders and small teams to efficiently manage customer relationships and execute multi-channel marketing campaigns with AI-powered assistance.

## Tech Stack Summary

### Backend Technologies
- **Language**: Rust (minimum 1.76+, target Edition 2024)
- **Framework**: Axum for REST APIs
- **Runtime**: Tokio for async operations
- **Database**: SurrealDB (multi-model: relational + document + graph)
- **AI Integration**: OpenRouter with specialized LLM tools

### Frontend Technologies
- **Web**: Next.js 14 with Tailwind CSS
- **Mobile**: React Native for iOS & Android
- **API Client**: Auto-generated from OpenAPI specs

### Infrastructure
- **Containerization**: Docker & Docker Compose
- **Orchestration**: Kubernetes (GKE)
- **Provisioning**: Terraform
- **CI/CD**: GitHub Actions
- **Build Tool**: Just (command runner)

## Monorepo Structure

```
crm.hey.sh/
├── backend/           # Rust/Axum API server
│   ├── src/
│   │   ├── handlers/  # HTTP route handlers
│   │   ├── models/    # Data models
│   │   ├── services/  # Business logic
│   │   ├── domain/    # Pure business logic
│   │   ├── repositories/ # Data access
│   │   ├── ai/        # AI integration stubs
│   │   └── llm_tools/ # LLM-powered tools
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

## Key Features

### Customer Relationship Management
- **Contact Management**: Full CRUD operations with detailed profiles and engagement tracking
- **Company Association**: Link contacts to companies with relationship mapping
- **Engagement Scoring**: Automatic scoring of contact interest and activity levels
- **Unified Timeline**: Comprehensive interaction history for each contact

### Campaign Engine
- **Multi-Channel**: Execute campaigns across email, social media, landing pages, and events
- **AI-Powered Content**: Generate personalized emails, social posts, and landing pages
- **Segmentation**: Advanced filtering to target specific contact groups
- **Automation**: Schedule and execute campaigns with minimal manual intervention

### Event Management
- **Event Creation**: Set up and manage events with RSVP tracking
- **Contact Invitations**: Automatically invite selected contacts to events
- **Attendance Tracking**: Monitor RSVPs and actual attendance
- **Follow-up Sequences**: Automated post-event engagement

### Analytics & Insights
- **Contact Breakdown**: Detailed analytics on contact engagement and demographics
- **Campaign Performance**: Track metrics across all campaign channels
- **Funnel Analysis**: Understand conversion from lead to customer
- **Engagement Trends**: Visualize engagement patterns over time

### AI Integration
- **Email Generation**: Create personalized email content based on contact profiles
- **Social Content**: Generate platform-appropriate social media posts
- **Landing Page Builder**: Automatically create custom landing pages for campaigns
- **Summarization**: AI-powered contact and engagement summaries

## Development Workflow

### Prerequisites
- Rust 1.75+
- Bun 1.0+ (for frontend)
- Node.js 20+ (for mobile)
- Docker & Docker Compose
- SurrealDB (or use Docker)
- [just](https://github.com/casey/just) command runner

### Quick Start Commands
From the repository root, common commands include:
```bash
just --list        # Show all available commands
just build         # Build all components
just test          # Test all components
just check         # Lint/check all components
just ci            # Run full CI pipeline
just install       # Install all dependencies
```

Component-specific commands:
```bash
just build-backend     # Build backend only
just build-frontend    # Build frontend only
just test-backend      # Test backend only
just dev-frontend      # Start frontend dev server
just run-backend       # Run backend server
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

## Deployment Architecture

### Production Infrastructure
- **Platform**: Google Cloud Platform (GCP)
- **Orchestration**: Google Kubernetes Engine (GKE)
- **Storage**: Managed through Kubernetes StatefulSets and Persistent Volume Claims
- **Load Balancing**: GKE Ingress controllers

### Deployment Process
1. **Infrastructure Setup**:
```bash
cd infra/terraform
cp terraform.tfvars.example terraform.tfvars
# Edit terraform.tfvars with your GCP project details
terraform init
terraform plan
terraform apply
```

2. **Configuration**:
```bash
cd infra/k8s
cp secrets.yaml.example secrets.yaml
# Edit secrets.yaml with your credentials
kubectl apply -f secrets.yaml
```

3. **Application Deployment**:
```bash
kubectl apply -f infra/k8s/
```

### Required GitHub Secrets
- `GCP_PROJECT_ID` - Your GCP project ID
- `GCP_SA_KEY` - Service account JSON key with appropriate permissions

## Team Structure and Ownership

As outlined in OWNERSHIP_PLAN.md, the project follows clear ownership patterns with specific team members responsible for different components:

- **Backend**: API design, business logic, data architecture
- **Frontend**: User experience, dashboard functionality, mobile web compatibility  
- **Mobile**: Native iOS/Android applications, device-specific features
- **AI/ML**: LLM integration, content generation, predictive analytics
- **DevOps**: Infrastructure, deployment, monitoring, security
- **Database**: Schema design, performance optimization, backup/recovery

## Future Roadmap

### Short-term Goals
- Complete gRPC service implementation for better microservice communication
- Enhance AI tooling with more sophisticated content generation
- Implement advanced analytics and reporting features
- Improve mobile application experience and functionality

### Long-term Vision
- Multi-tenant SaaS offering with enhanced security
- Expanded AI capabilities including conversation automation
- Integration marketplace for third-party tool connections
- Advanced workflow automation and business process management

## Contributing

The project follows Rust best practices and clean architecture principles. Contributions should:
- Follow the established layered architecture patterns
- Maintain the Rust 2024 edition and Tokio runtime standards
- Include comprehensive unit and integration tests
- Adhere to the documented error handling and observability practices
- Respect the separation of concerns between handlers, services, domain, and repositories

For more information on contributing, see the specific documentation files:
- `ARCHITECTURE.md` - System design and component interactions
- `CONVENTIONS.md` - Development practices and code style
- `LLM.md` - AI integration patterns and implementation details