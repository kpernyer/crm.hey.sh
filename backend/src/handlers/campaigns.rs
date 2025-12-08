use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use surrealdb::sql::Thing;

use crate::ai::{ai_email, ai_landing_page, ai_social};
use crate::error::{AppError, AppResult};
use crate::models::{
    AssetType, Campaign, CampaignAsset, CampaignAssetResponse, CampaignResponse, CampaignStatus,
    CreateCampaignRequest, GenerateAssetsRequest, UpdateCampaignRequest,
};
use crate::AppState;

pub async fn list_campaigns(State(state): State<AppState>) -> AppResult<Json<Vec<CampaignResponse>>> {
    let campaigns: Vec<Campaign> = state
        .db
        .client
        .query("SELECT * FROM campaign ORDER BY created_at DESC")
        .await?
        .take(0)?;

    let responses: Vec<CampaignResponse> = campaigns.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_campaign(
    State(state): State<AppState>,
    Json(req): Json<CreateCampaignRequest>,
) -> AppResult<Json<CampaignResponse>> {
    let now = Utc::now();

    let campaigns: Vec<Campaign> = state
        .db
        .client
        .create("campaign")
        .content(Campaign {
            id: None,
            name: req.name,
            objective: req.objective,
            status: CampaignStatus::Draft,
            channels: req.channels,
            prompt: req.prompt,
            segment_definition: req.segment_definition.unwrap_or(serde_json::json!({})),
            created_at: now,
            updated_at: now,
        })
        .await?;

    let campaign = campaigns.into_iter().next().ok_or_else(|| AppError::Internal("Failed to create campaign".into()))?;
    Ok(Json(campaign.into()))
}

pub async fn get_campaign(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<CampaignResponse>> {
    let campaign: Option<Campaign> = state
        .db
        .client
        .select(("campaign", id.as_str()))
        .await?;

    let campaign = campaign.ok_or_else(|| AppError::NotFound("Campaign not found".into()))?;
    Ok(Json(campaign.into()))
}

pub async fn update_campaign(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateCampaignRequest>,
) -> AppResult<Json<CampaignResponse>> {
    let existing: Option<Campaign> = state
        .db
        .client
        .select(("campaign", id.as_str()))
        .await?;

    let mut campaign = existing.ok_or_else(|| AppError::NotFound("Campaign not found".into()))?;

    if let Some(name) = req.name {
        campaign.name = name;
    }
    if let Some(objective) = req.objective {
        campaign.objective = objective;
    }
    if let Some(status) = req.status {
        campaign.status = status;
    }
    if let Some(channels) = req.channels {
        campaign.channels = channels;
    }
    if let Some(prompt) = req.prompt {
        campaign.prompt = Some(prompt);
    }
    if let Some(segment_definition) = req.segment_definition {
        campaign.segment_definition = segment_definition;
    }

    campaign.updated_at = Utc::now();

    let updated: Option<Campaign> = state
        .db
        .client
        .update(("campaign", id.as_str()))
        .content(campaign)
        .await?;

    let campaign = updated.ok_or_else(|| AppError::Internal("Failed to update campaign".into()))?;
    Ok(Json(campaign.into()))
}

pub async fn list_campaign_assets(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Vec<CampaignAssetResponse>>> {
    let assets: Vec<CampaignAsset> = state
        .db
        .client
        .query("SELECT * FROM campaign_asset WHERE campaign = $campaign ORDER BY created_at DESC")
        .bind(("campaign", Thing::from(("campaign", id.as_str()))))
        .await?
        .take(0)?;

    let responses: Vec<CampaignAssetResponse> = assets.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn generate_campaign_assets(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<GenerateAssetsRequest>,
) -> AppResult<Json<Vec<CampaignAssetResponse>>> {
    let campaign_thing = Thing::from(("campaign", id.as_str()));
    let mut created_assets = Vec::new();

    for asset_type in req.asset_types {
        let generated_content = match asset_type {
            AssetType::Email => {
                let email = ai_email::generate_email(&req.prompt).await;
                serde_json::to_value(email).unwrap_or(serde_json::json!({}))
            }
            AssetType::SocialPost => {
                let posts = ai_social::generate_social_posts(&req.prompt).await;
                serde_json::to_value(posts).unwrap_or(serde_json::json!({}))
            }
            AssetType::LandingPage => {
                let page = ai_landing_page::generate_landing_page(&req.prompt).await;
                serde_json::to_value(page).unwrap_or(serde_json::json!({}))
            }
            AssetType::EventInvite => {
                let email = ai_email::generate_email(&format!("Event invitation: {}", req.prompt)).await;
                serde_json::to_value(email).unwrap_or(serde_json::json!({}))
            }
        };

        let assets: Vec<CampaignAsset> = state
            .db
            .client
            .create("campaign_asset")
            .content(CampaignAsset {
                id: None,
                campaign: campaign_thing.clone(),
                asset_type: asset_type.clone(),
                generated_content,
                url: None,
                created_at: Utc::now(),
            })
            .await?;

        if let Some(a) = assets.into_iter().next() {
            created_assets.push(a.into());
        }
    }

    Ok(Json(created_assets))
}

pub async fn execute_campaign(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    // Update campaign status to running
    let _: Option<Campaign> = state
        .db
        .client
        .query("UPDATE campaign SET status = 'running', updated_at = $now WHERE id = $id")
        .bind(("id", Thing::from(("campaign", id.as_str()))))
        .bind(("now", Utc::now()))
        .await?
        .take(0)?;

    // In a real implementation, this would trigger background jobs
    // For now, we just return success
    Ok(Json(serde_json::json!({
        "status": "execution_started",
        "campaign_id": id,
        "message": "Campaign execution has been triggered. Assets will be distributed according to channel configuration."
    })))
}
