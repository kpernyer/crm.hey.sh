# CRM.HEY.SH Architecture

## High-Level System Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              CLIENTS                                            │
├───────────────────────┬───────────────────────┬─────────────────────────────────┤
│    Web Browser        │    iOS/Android App    │     API Consumers               │
│    (Next.js)          │    (React Native)     │     (curl, integrations)        │
└───────────┬───────────┴───────────┬───────────┴─────────────────┬───────────────┘
            │                       │                             │
            │ HTTP/REST             │ HTTP/REST                   │ HTTP/REST
            ▼                       ▼                             ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         RUST/AXUM BACKEND (:8080)                               │
│  ┌─────────────────────────────────────────────────────────────────────────┐    │
│  │                         REST API Layer                                   │    │
│  │   /api/contacts   /api/campaigns   /api/events   /api/analytics         │    │
│  │   /api/companies  /api/timeline    /api/landing-pages    /health        │    │
│  └─────────────────────────────────────────────────────────────────────────┘    │
│                                    │                                            │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐    │
│  │                         HANDLERS (HTTP Routing)                          │    │
│  │   contacts.rs  campaigns.rs  events.rs  analytics.rs  landing_pages.rs  │    │
│  └─────────────────────────────────┬───────────────────────────────────────┘    │
│                                    │                                            │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐    │
│  │                         SERVICES (Business Orchestration)                │    │
│  │   contact_service    campaign_executor    segment_builder               │    │
│  └─────────────────────────────────┬───────────────────────────────────────┘    │
│                                    │                                            │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐    │
│  │                         DOMAIN (Pure Business Logic)                     │    │
│  │   contact.rs    validation.rs    engagement.rs    errors.rs             │    │
│  └─────────────────────────────────┬───────────────────────────────────────┘    │
│                                    │                                            │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐    │
│  │                         REPOSITORIES (Data Access)                       │    │
│  │   contact_repository.rs  (SurrealDB queries)                            │    │
│  └─────────────────────────────────┬───────────────────────────────────────┘    │
│                                    │                                            │
│  ┌─────────────────────────────────▼───────────────────────────────────────┐    │
│  │                        LLM TOOLS & AI MODULES                            │    │
│  │   email_generator   social_post_generator   landing_page_generator      │    │
│  └─────────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────┬───────────────────────────────────────────┘
                                      │ WebSocket
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         SURREALDB (:8000)                                       │
│   Tables: contact, company, campaign, campaign_asset, event, rsvp, timeline    │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Container Breakdown

| Container | Technology | Port | Purpose |
|-----------|------------|------|---------|
| **backend** | Rust/Axum | 8080 | REST API, business logic |
| **frontend** | Next.js 14 | 3000 | Web dashboard |
| **mobile** | React Native | N/A | iOS/Android app |
| **surrealdb** | SurrealDB | 8000 | Document database |

---

## Backend Module Structure

```
backend/src/
├── main.rs                 # Server bootstrap, routing
├── config.rs               # YAML + env config loading
├── db.rs                   # SurrealDB connection & schema init
├── error.rs                # AppError with HTTP status mapping
├── secrets.rs              # Optional secret management
│
├── handlers/               # HTTP request handlers
│   ├── contacts.rs         # CRUD + timeline
│   ├── companies.rs        # CRUD
│   ├── campaigns.rs        # CRUD + AI asset generation + execute
│   ├── events.rs           # CRUD + RSVP
│   ├── landing_pages.rs    # Generate + serve
│   ├── timeline.rs         # Timeline entries
│   ├── analytics.rs        # Mocked metrics
│   └── health.rs           # Health check
│
├── services/               # Business orchestration
│   ├── contact_service.rs  # Email uniqueness, CRUD
│   ├── campaign_executor.rs# Multi-channel execution
│   └── segment_builder.rs  # Contact filtering
│
├── domain/                 # Pure business logic (no I/O)
│   ├── contact.rs          # Entity + status state machine
│   ├── validation.rs       # Input validators
│   ├── engagement.rs       # Scoring logic
│   └── errors.rs           # Domain errors
│
├── models/                 # Request/Response DTOs
│   ├── contact.rs
│   ├── company.rs
│   ├── campaign.rs
│   ├── event.rs
│   └── timeline.rs
│
├── repositories/           # Data access
│   └── contact_repository.rs
│
├── ai/                     # AI integration stubs
│   ├── ai_email.rs
│   ├── ai_social.rs
│   ├── ai_landing_page.rs
│   └── ai_summary.rs
│
└── llm_tools/              # LLM-powered tools
    ├── email_generator.rs
    ├── social_post_generator.rs
    ├── landing_page_generator.rs
    └── ...
```

---

## REST API Endpoints

### Health & Docs
| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/swagger-ui` | Interactive API docs |
| GET | `/api-docs/openapi.json` | OpenAPI spec |

### Contacts
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/contacts` | List (with filters) |
| POST | `/api/contacts` | Create contact |
| GET | `/api/contacts/:id` | Get by ID |
| PATCH | `/api/contacts/:id` | Update |
| DELETE | `/api/contacts/:id` | Delete |
| GET | `/api/contacts/:id/timeline` | Get timeline entries |

### Companies
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/companies` | List |
| POST | `/api/companies` | Create |
| GET | `/api/companies/:id` | Get |
| PATCH | `/api/companies/:id` | Update |
| DELETE | `/api/companies/:id` | Delete |

### Campaigns
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/campaigns` | List |
| POST | `/api/campaigns` | Create |
| GET | `/api/campaigns/:id` | Get |
| PATCH | `/api/campaigns/:id` | Update |
| POST | `/api/campaigns/:id/assets` | Generate AI assets |
| GET | `/api/campaigns/:id/assets` | List assets |
| POST | `/api/campaigns/:id/execute` | Execute campaign |

### Events
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/events` | List |
| POST | `/api/events` | Create |
| GET | `/api/events/:id` | Get |
| POST | `/api/events/:id/invite` | Invite contacts |
| POST | `/api/events/:id/rsvp` | Record RSVP |

### Landing Pages
| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/landing-pages/generate` | Generate page |
| GET | `/lp/:id` | View page |
| POST | `/lp/:id/submit` | Submit form |

### Analytics
| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/analytics/contacts` | Contact breakdown |
| GET | `/api/analytics/campaign/:id` | Campaign metrics |
| GET | `/api/analytics/funnel` | Funnel stages |

---

## Internal gRPC Services (Proto Definitions)

Located in `proto/` - defined but not yet fully implemented:

### CampaignService
```protobuf
rpc GenerateAssets(...)     → GenerateAssetsResponse
rpc ScheduleCampaign(...)   → ScheduleCampaignResponse
rpc ExecuteCampaign(...)    → ExecuteCampaignResponse
rpc GetCampaignStatus(...)  → GetCampaignStatusResponse
```

### NotificationService
```protobuf
rpc SendEmail(...)          → SendEmailResponse
rpc SendBatchEmails(...)    → SendBatchEmailsResponse
rpc SendPushNotification(...)→ SendPushResponse
rpc SendWebhook(...)        → SendWebhookResponse
```

### AnalyticsService
```protobuf
rpc GetCampaignAnalytics(...)→ GetCampaignAnalyticsResponse
rpc GetContactAnalytics(...) → GetContactAnalyticsResponse
rpc GetFunnelAnalytics(...)  → GetFunnelAnalyticsResponse
rpc TrackEvent(...)          → TrackEventResponse
```

---

## Communication Flows

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         EXTERNAL (REST)                                  │
│                                                                          │
│   Browser/Mobile ──HTTP/JSON──▶ Axum Backend ──WebSocket──▶ SurrealDB   │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│                         INTERNAL (Rust Layers)                           │
│                                                                          │
│   Handler ──▶ Service ──▶ Domain (validation) ──▶ Repository ──▶ DB     │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│                         FUTURE (gRPC)                                    │
│                                                                          │
│   CampaignService ──gRPC──▶ NotificationService (email/push)            │
│   Backend ──gRPC──▶ AnalyticsService (event tracking)                   │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## LLM Integration Architecture

The system includes a dedicated `llm_tools/` directory for integrating Large Language Model capabilities:

- **email_generator.rs**: Generates personalized email content
- **social_post_generator.rs**: Creates social media content
- **landing_page_generator.rs**: Builds custom landing pages
- **Other LLM-powered tools**: Extend functionality as needed

These tools are accessible via the backend's internal tool interface and can be orchestrated through campaigns and other business processes.

---

## Infrastructure (Production)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         GKE CLUSTER                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐ │
│  │ Backend Deploy  │  │ Frontend Deploy │  │ SurrealDB           │ │
│  │ (2 replicas)    │  │                 │  │ StatefulSet + PVC   │ │
│  └────────┬────────┘  └────────┬────────┘  └──────────┬──────────┘ │
│           │                    │                      │            │
│  ┌────────▼────────────────────▼──────────────────────▼──────────┐ │
│  │                    KUBERNETES SERVICES                         │ │
│  └────────────────────────────┬──────────────────────────────────┘ │
│                               │                                    │
│  ┌────────────────────────────▼──────────────────────────────────┐ │
│  │                         INGRESS                                │ │
│  └────────────────────────────┬──────────────────────────────────┘ │
└───────────────────────────────┼─────────────────────────────────────┘
                                │
                                ▼
                         External Traffic
```

Managed via Terraform (`infra/terraform/`) and Kubernetes manifests (`infra/kubernetes/`).

---

## Development Architecture

- **Monorepo Structure**: All components (backend, frontend, mobile) in a single repository
- **Justfile Orchestration**: Centralized build and development commands
- **Docker Compose**: Local development environment
- **Layered Architecture**: Clear separation of concerns (handlers → services → domain → repositories)