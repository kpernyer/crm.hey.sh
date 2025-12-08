use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignObjective {
    Awareness,
    LeadGen,
    Event,
    Investor,
    EarlyAdopters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Running,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignChannel {
    Email,
    Social,
    LandingPage,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Option<Thing>,
    pub name: String,
    pub objective: CampaignObjective,
    pub status: CampaignStatus,
    pub channels: Vec<CampaignChannel>,
    pub prompt: Option<String>,
    pub segment_definition: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Email,
    SocialPost,
    LandingPage,
    EventInvite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignAsset {
    pub id: Option<Thing>,
    pub campaign: Thing,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    pub generated_content: serde_json::Value,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub objective: CampaignObjective,
    pub channels: Vec<CampaignChannel>,
    pub prompt: Option<String>,
    pub segment_definition: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub objective: Option<CampaignObjective>,
    pub status: Option<CampaignStatus>,
    pub channels: Option<Vec<CampaignChannel>>,
    pub prompt: Option<String>,
    pub segment_definition: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateAssetsRequest {
    pub prompt: String,
    pub asset_types: Vec<AssetType>,
}

#[derive(Debug, Serialize)]
pub struct CampaignResponse {
    pub id: String,
    pub name: String,
    pub objective: CampaignObjective,
    pub status: CampaignStatus,
    pub channels: Vec<CampaignChannel>,
    pub prompt: Option<String>,
    pub segment_definition: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Campaign> for CampaignResponse {
    fn from(c: Campaign) -> Self {
        Self {
            id: c.id.map(|t| t.id.to_string()).unwrap_or_default(),
            name: c.name,
            objective: c.objective,
            status: c.status,
            channels: c.channels,
            prompt: c.prompt,
            segment_definition: c.segment_definition,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CampaignAssetResponse {
    pub id: String,
    pub campaign_id: String,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    pub generated_content: serde_json::Value,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<CampaignAsset> for CampaignAssetResponse {
    fn from(a: CampaignAsset) -> Self {
        Self {
            id: a.id.map(|t| t.id.to_string()).unwrap_or_default(),
            campaign_id: a.campaign.id.to_string(),
            asset_type: a.asset_type,
            generated_content: a.generated_content,
            url: a.url,
            created_at: a.created_at,
        }
    }
}
