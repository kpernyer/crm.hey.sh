use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use surrealdb::sql::Thing;

use crate::ai::ai_landing_page;
use crate::error::{AppError, AppResult};
use crate::models::{AssetType, CampaignAsset, Contact, ContactStatus, TimelineEntry, TimelineEntryType};
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct GenerateLandingPageRequest {
    pub prompt: String,
    pub campaign_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct LandingPageResponse {
    pub id: String,
    pub content: serde_json::Value,
    pub url: String,
}

pub async fn generate_landing_page(
    State(state): State<AppState>,
    Json(req): Json<GenerateLandingPageRequest>,
) -> AppResult<Json<LandingPageResponse>> {
    let generated = ai_landing_page::generate_landing_page(&req.prompt).await;
    let content = serde_json::to_value(&generated).unwrap_or(serde_json::json!({}));

    let campaign = req.campaign_id.map(|id| Thing::from(("campaign", id.as_str())));

    let assets: Vec<CampaignAsset> = state
        .db
        .client
        .create("campaign_asset")
        .content(CampaignAsset {
            id: None,
            campaign: campaign.unwrap_or_else(|| Thing::from(("campaign", "standalone"))),
            asset_type: AssetType::LandingPage,
            generated_content: content.clone(),
            url: None,
            created_at: Utc::now(),
        })
        .await?;

    let asset = assets.into_iter().next().ok_or_else(|| AppError::Internal("Failed to create landing page".into()))?;
    let id = asset.id.map(|t| t.id.to_string()).unwrap_or_default();

    Ok(Json(LandingPageResponse {
        id: id.clone(),
        content,
        url: format!("/lp/{}", id),
    }))
}

pub async fn get_landing_page(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let asset: Option<CampaignAsset> = state
        .db
        .client
        .select(("campaign_asset", id.as_str()))
        .await?;

    let asset = asset.ok_or_else(|| AppError::NotFound("Landing page not found".into()))?;

    Ok(Json(asset.generated_content))
}

#[derive(serde::Deserialize)]
pub struct LandingPageSubmission {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub company: Option<String>,
    pub message: Option<String>,
}

pub async fn submit_landing_page_form(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(submission): Json<LandingPageSubmission>,
) -> AppResult<Json<serde_json::Value>> {
    // Create or find contact
    let existing: Vec<Contact> = state
        .db
        .client
        .query("SELECT * FROM contact WHERE email = $email LIMIT 1")
        .bind(("email", &submission.email))
        .await?
        .take(0)?;

    let contact_id = if let Some(contact) = existing.first() {
        contact.id.clone().unwrap()
    } else {
        let now = Utc::now();
        let new_contacts: Vec<Contact> = state
            .db
            .client
            .create("contact")
            .content(Contact {
                id: None,
                first_name: submission.first_name.clone(),
                last_name: submission.last_name.clone(),
                email: submission.email.clone(),
                phone: None,
                linkedin_url: None,
                tags: vec!["landing_page_lead".to_string()],
                status: ContactStatus::Lead,
                engagement_score: 10.0,
                company: None,
                created_at: now,
                updated_at: now,
            })
            .await?;

        new_contacts
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("Failed to create contact".into()))?
            .id
            .unwrap()
    };

    // Create timeline entry for the landing page visit/submission
    let _: Vec<TimelineEntry> = state
        .db
        .client
        .create("timeline_entry")
        .content(TimelineEntry {
            id: None,
            contact: contact_id.clone(),
            company: None,
            entry_type: TimelineEntryType::LandingPageVisit,
            content: format!("Submitted form on landing page {}", id),
            metadata: serde_json::json!({
                "landing_page_id": id,
                "message": submission.message,
                "company": submission.company,
            }),
            timestamp: Utc::now(),
        })
        .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "contact_id": contact_id.id.to_string(),
        "message": "Thank you for your submission!"
    })))
}
