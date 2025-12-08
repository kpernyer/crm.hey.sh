# Starting spec for hey.sh projects

AI Agent Instruction Document (Rust + React + SurrealDB + K8s)

##🚀 Project Overview

Build a modern founder-focused CRM & Campaign Engine for crm.hey.sh, hosted on Google Cloud, using the same styling and frontend design language as www.hey.sh.

This is a 2025 growth CRM with:
	•	Contacts / Companies / Timeline
	•	Campaigns (email, social, landing pages, events)
	•	AI-assisted content (stubs for now)
	•	Web app + iOS/Android app for on-the-go use

The AI agent should generate a complete monorepo skeleton with backend, frontend web, mobile app, infra, CI/CD.

## 🎯 Core Objectives
	1.	Simple but powerful CRM for contacts & companies.
	2.	Unified interaction timeline per contact/company.
	3.	Segmentation & tagging for targeted outreach.
	4.	Campaign builder (email + social + landing page + events).
	5.	Landing page generation from text prompts.
	6.	Event invitations and RSVP tracking.
	7.	Analytics dashboard (campaign performance, funnel, engagement).
	8.	AI integration stubs for content generation & summaries.
	9.	Clean Rust backend with SurrealDB.
	10.	Docker/K8s + Terraform + GitHub Actions for deploy to GCP.
	11.	API Gateway for public API, gRPC/Protobuf for internal services.
	12.	Web planning UI + mobile app for on-the-go usage.

## 🧱 Tech Stack

Backend
	•	Language: Rust
	•	Web framework: Axum (preferred)
	•	Auth: simple token-based for now (e.g. JWT), pluggable
	•	DB client: SurrealDB official client from Rust
	•	Architecture: modular, ready to split into services later
	•	Communication:
	•	External: REST/JSON behind an API Gateway
	•	Internal: gRPC using Protobuf

Database
	•	SurrealDB as primary data store
	•	Use SurrealQL to define schema & indexes
	•	Provide initial schema/migration scripts

Frontend Web
	•	React (or Next.js, matching www.hey.sh conventions)
	•	Use same styling system as www.hey.sh (colors, typography, components)
	•	Pages: Dashboard, Contacts, Companies, Campaigns, Events, Analytics, Settings

Mobile App
	•	React Native app for iOS + Android
	•	Core features:
	•	View/update contacts
	•	See timelines
	•	Quick notes & tasks
	•	View active campaigns & events
	•	Simple notifications (stub)
	•	Reuse types and API client from web where possible

Infra
	•	Containerization: Docker
	•	Orchestration: Kubernetes (GKE)
	•	IaC: Terraform for:
	•	GKE cluster
	•	SurrealDB hosting (container or managed if applicable)
	•	API Gateway
	•	Networking + basic IAM
	•	CI/CD: GitHub Actions for:
	•	Build & test backend
	•	Build & test frontend
	•	Build & test mobile (at least lint/build)
	•	Build Docker images
	•	Deploy to GKE via Terraform or kubectl

## 🗃️ Data Model (SurrealDB)

Define SurrealDB schema (SurrealQL) and corresponding Rust types.

contact
	•	id
	•	first_name
	•	last_name
	•	email
	•	phone (optional)
	•	linkedin_url
	•	tags (array)
	•	status (lead, customer, partner, investor, other)
	•	engagement_score (int/float)
	•	company -> company (relation)
	•	created_at, updated_at

company
	•	id
	•	name
	•	domain
	•	industry
	•	size (optional)
	•	tags (array)
	•	created_at, updated_at

timeline_entry
	•	id
	•	contact -> contact
	•	company -> company (optional)
	•	type (email_sent, email_open, email_click, social_touch, note, event_invite, event_attend, landing_page_visit, task, call)
	•	content (string)
	•	metadata (object)
	•	timestamp (datetime)

campaign
	•	id
	•	name
	•	objective (awareness, lead_gen, event, investor, early_adopters)
	•	status (draft, scheduled, running, completed)
	•	channels (array: email, social, landing_page, event)
	•	prompt (text)
	•	segment_definition (json / query)
	•	created_at, updated_at

campaign_asset
	•	id
	•	campaign -> campaign
	•	type (email, social_post, landing_page, event_invite)
	•	generated_content (object/json: subject, body, hero, sections, etc.)
	•	url (for landing pages)
	•	created_at

event
	•	id
	•	campaign -> campaign (optional)
	•	name
	•	type (webinar, meetup, AMA, demo, other)
	•	description
	•	start_time, end_time
	•	location (string or url)
	•	created_at

rsvp
	•	id
	•	event -> event
	•	contact -> contact
	•	status (invited, registered, attended, no_show)
	•	timestamp

## 🔌 Backend API Design

Expose REST endpoints behind API Gateway.

Contacts

GET    /api/contacts
POST   /api/contacts
GET    /api/contacts/{id}
PATCH  /api/contacts/{id}
DELETE /api/contacts/{id}

Companies

GET    /api/companies
POST   /api/companies
GET    /api/companies/{id}
PATCH  /api/companies/{id}
DELETE /api/companies/{id}

Timeline

GET   /api/contacts/{id}/timeline
POST  /api/timeline         # create manual entry (note, call, etc.)

Campaigns

GET   /api/campaigns
POST  /api/campaigns
GET   /api/campaigns/{id}
PATCH /api/campaigns/{id}

/api/campaigns/{id}/assets       # manage AI-generated assets
POST /api/campaigns/{id}/assets  # generate assets from prompt (stub AI)

/api/campaigns/{id}/execute      # trigger: create jobs for email/social/etc.
POST /api/campaigns/{id}/execute

Landing Pages

POST /api/landing-pages/generate   # from text prompt
GET  /lp/{id}                      # public landing page
POST /lp/{id}/submit               # form submission -> contact + timeline

Events

POST /api/events
GET  /api/events
GET  /api/events/{id}

POST /api/events/{id}/invite       # invite segment
POST /api/events/{id}/rsvp         # RSVP endpoint

Analytics

GET /api/analytics/campaign/{id}
GET /api/analytics/contacts
GET /api/analytics/funnel

## 🛰️ Internal Service Communication (gRPC/Protobuf)

Define Protobuf contracts for internal services (can be separate modules for later):
	•	CampaignService
	•	Generate assets (calls AI service)
	•	Schedule executions
	•	AnalyticsService
	•	Aggregate per campaign
	•	Funnel metrics
	•	NotificationService
	•	Dispatch emails (via external provider)
	•	Dispatch webhooks or push notifications

Create .proto files and Rust service stubs, but implementations may be simple or mocked initially.

## 🎨 Frontend Web (React)

Use the same styling system as www.hey.sh (colors, typography, layout), with a clean dashboard UI.

Required pages/components
	1.	Dashboard
	•	Summary of: new contacts, active campaigns, upcoming events, key metrics.
	2.	Contacts
	•	Contacts list with filters/tags.
	•	Contact detail page:
	•	Basic info
	•	Company
	•	Tags
	•	Engagement score
	•	Timeline feed (scrollable)
	•	Button to add note / log call / view campaigns.
	3.	Companies
	•	Company list
	•	Detail page with associated contacts and notes.
	4.	Campaigns
	•	Campaign list
	•	Campaign detail:
	•	Objective, status, channels
	•	AI prompt
	•	Generated assets (emails, posts, landing pages)
	•	Execution history
	•	Key stats
	•	Campaign Builder UI:
	•	Select objective
	•	Define target segment (using simple filter builder)
	•	Toggle channels: email / social / landing_page / event
	•	Enter AI prompt
	•	Preview generated content (using stub AI)
	5.	Landing Page Preview
	•	WYSIWYG-like view showing what the generated landing page looks like.
	6.	Events
	•	Events list
	•	Event detail: description, attendees, RSVPs, link to campaign.
	7.	Analytics
	•	Simple charts/tables:
	•	Campaign performance
	•	Funnel: views → clicks → signups → deals (stub)
	•	Contacts engagement ranking.

## 📱 Mobile App (React Native)

Implement a minimal but useful v1 app:

Screens
	1.	Login/Onboarding (simple token/manual for now).
	2.	Home
	•	List of recent timeline items or “Today’s focus”.
	3.	Contacts
	•	Search + list
	•	Contact details + timeline
	•	Add note / log interaction
	4.	Events
	•	Upcoming events
	•	View attendees
	5.	Notifications (placeholder for now)

Use the same domain API (https://crm.hey.sh/api/...).

## 🤖 AI Integration Stubs

Create Rust modules (no real AI calls yet):
	•	ai_email.rs
	•	ai_social.rs
	•	ai_landing_page.rs
	•	ai_summary.rs

Each exposing functions like:

pub async fn generate_email(prompt: &str) -> GeneratedEmail { ... }
pub async fn generate_social_posts(prompt: &str) -> Vec<GeneratedPost> { ... }
pub async fn generate_landing_page(prompt: &str) -> GeneratedLandingPage { ... }
pub async fn summarize_timeline(entries: &[TimelineEntry]) -> String { ... }

For now, return hardcoded or template-based mock data.

## 🧪 CI/CD (GitHub + GitHub Actions)

Set up GitHub Actions workflows:
	1.	backend-ci.yml
	•	cargo fmt --check
	•	cargo clippy -- -D warnings
	•	cargo test
	•	Build Docker image
	2.	frontend-ci.yml
	•	Install deps
	•	Lint
	•	Run tests / build
	3.	mobile-ci.yml
	•	Basic React Native lint / TypeScript check
	4.	deploy.yml
	•	On main or tagged release:
	•	Build & push Docker images
	•	Terraform plan & apply, or
	•	kubectl apply manifests

## 🏗️ Infra (Terraform + K8s)

Create a minimal infra/ structure:
	•	terraform/
	•	GKE cluster
	•	SurrealDB deployment (StatefulSet or managed)
	•	API Gateway config
	•	Service accounts & IAM
	•	k8s/
	•	deployment-backend.yaml
	•	service-backend.yaml
	•	deployment-frontend.yaml
	•	service-frontend.yaml
	•	Ingress or Gateway resources

## 🧭 Development Priorities for the AI Agent
	1.	Create monorepo structure: backend/, frontend/, mobile/, infra/.
	2.	Implement Rust/Axum backend with SurrealDB integration and core models.
	3.	Implement React web frontend with basic pages (Dashboard, Contacts, Campaigns).
	4.	Implement React Native mobile skeleton (Home, Contacts, Events).
	5.	Add API Gateway–friendly routes and internal gRPC stubs.
	6.	Provide Dockerfiles and K8s manifests.
	7.	Add GitHub Actions and Terraform skeleton for GCP.

## 📝 Final Instruction to the AI Tool

Using this specification, generate a complete initial codebase for crm.hey.sh with:
	•	Rust Axum backend + SurrealDB
	•	React web frontend styled like www.hey.sh
	•	React Native mobile app
	•	REST API behind an API gateway and internal gRPC stubs
	•	Docker/K8s manifests, Terraform skeleton for GCP, and GitHub Actions CI/CD
Provide clear README instructions for local development and deployment.

You can paste this whole thing as a single system prompt / spec into Claude, Codex, or your terminal agent.
