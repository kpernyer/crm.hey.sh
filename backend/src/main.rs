use anyhow::Result;
use axum::{
    routing::{get, post, patch, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod ai;
mod config;
mod db;
mod domain;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;

// Re-export domain types for use in library context
pub use domain::*;

use db::Database;
use services::ContactService;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub contact_service: Arc<ContactService>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    // Initialize database
    let db = Database::new().await?;
    db.init_schema().await?;
    let db = Arc::new(db);

    // Initialize services
    let contact_service = Arc::new(ContactService::new(Arc::clone(&db)));

    let state = AppState {
        db,
        contact_service,
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        // Contacts
        .route("/api/contacts", get(handlers::contacts::list_contacts))
        .route("/api/contacts", post(handlers::contacts::create_contact))
        .route("/api/contacts/:id", get(handlers::contacts::get_contact))
        .route("/api/contacts/:id", patch(handlers::contacts::update_contact))
        .route("/api/contacts/:id", delete(handlers::contacts::delete_contact))
        .route("/api/contacts/:id/timeline", get(handlers::timeline::get_contact_timeline))
        // Companies
        .route("/api/companies", get(handlers::companies::list_companies))
        .route("/api/companies", post(handlers::companies::create_company))
        .route("/api/companies/:id", get(handlers::companies::get_company))
        .route("/api/companies/:id", patch(handlers::companies::update_company))
        .route("/api/companies/:id", delete(handlers::companies::delete_company))
        // Timeline
        .route("/api/timeline", post(handlers::timeline::create_timeline_entry))
        // Campaigns
        .route("/api/campaigns", get(handlers::campaigns::list_campaigns))
        .route("/api/campaigns", post(handlers::campaigns::create_campaign))
        .route("/api/campaigns/:id", get(handlers::campaigns::get_campaign))
        .route("/api/campaigns/:id", patch(handlers::campaigns::update_campaign))
        .route("/api/campaigns/:id/assets", get(handlers::campaigns::list_campaign_assets))
        .route("/api/campaigns/:id/assets", post(handlers::campaigns::generate_campaign_assets))
        .route("/api/campaigns/:id/execute", post(handlers::campaigns::execute_campaign))
        // Landing Pages
        .route("/api/landing-pages/generate", post(handlers::landing_pages::generate_landing_page))
        .route("/lp/:id", get(handlers::landing_pages::get_landing_page))
        .route("/lp/:id/submit", post(handlers::landing_pages::submit_landing_page_form))
        // Events
        .route("/api/events", get(handlers::events::list_events))
        .route("/api/events", post(handlers::events::create_event))
        .route("/api/events/:id", get(handlers::events::get_event))
        .route("/api/events/:id/invite", post(handlers::events::invite_to_event))
        .route("/api/events/:id/rsvp", post(handlers::events::rsvp_event))
        // Analytics
        .route("/api/analytics/campaign/:id", get(handlers::analytics::campaign_analytics))
        .route("/api/analytics/contacts", get(handlers::analytics::contacts_analytics))
        .route("/api/analytics/funnel", get(handlers::analytics::funnel_analytics))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = "0.0.0.0:8080";
    tracing::info!("Starting CRM server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
