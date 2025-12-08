use axum::{
    extract::{Path, State},
    Json,
};

use crate::error::AppResult;
use crate::AppState;

#[derive(serde::Serialize)]
pub struct CampaignAnalytics {
    pub campaign_id: String,
    pub total_contacts: u64,
    pub emails_sent: u64,
    pub emails_opened: u64,
    pub emails_clicked: u64,
    pub landing_page_visits: u64,
    pub conversions: u64,
    pub open_rate: f64,
    pub click_rate: f64,
    pub conversion_rate: f64,
}

pub async fn campaign_analytics(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<CampaignAnalytics>> {
    // Mock analytics data - in production, this would aggregate from timeline entries
    Ok(Json(CampaignAnalytics {
        campaign_id: id,
        total_contacts: 1250,
        emails_sent: 1200,
        emails_opened: 480,
        emails_clicked: 96,
        landing_page_visits: 72,
        conversions: 18,
        open_rate: 40.0,
        click_rate: 8.0,
        conversion_rate: 1.5,
    }))
}

#[derive(serde::Serialize)]
pub struct ContactsAnalytics {
    pub total_contacts: u64,
    pub leads: u64,
    pub customers: u64,
    pub partners: u64,
    pub investors: u64,
    pub other: u64,
    pub avg_engagement_score: f64,
    pub new_this_month: u64,
    pub top_engaged: Vec<TopEngagedContact>,
}

#[derive(serde::Serialize)]
pub struct TopEngagedContact {
    pub id: String,
    pub name: String,
    pub engagement_score: f64,
}

pub async fn contacts_analytics(
    State(_state): State<AppState>,
) -> AppResult<Json<ContactsAnalytics>> {
    // Mock analytics data
    Ok(Json(ContactsAnalytics {
        total_contacts: 3500,
        leads: 2100,
        customers: 850,
        partners: 320,
        investors: 180,
        other: 50,
        avg_engagement_score: 42.5,
        new_this_month: 145,
        top_engaged: vec![
            TopEngagedContact {
                id: "1".to_string(),
                name: "John Smith".to_string(),
                engagement_score: 98.5,
            },
            TopEngagedContact {
                id: "2".to_string(),
                name: "Jane Doe".to_string(),
                engagement_score: 95.2,
            },
            TopEngagedContact {
                id: "3".to_string(),
                name: "Bob Johnson".to_string(),
                engagement_score: 92.8,
            },
        ],
    }))
}

#[derive(serde::Serialize)]
pub struct FunnelAnalytics {
    pub stages: Vec<FunnelStage>,
    pub overall_conversion_rate: f64,
}

#[derive(serde::Serialize)]
pub struct FunnelStage {
    pub name: String,
    pub count: u64,
    pub percentage: f64,
}

pub async fn funnel_analytics(State(_state): State<AppState>) -> AppResult<Json<FunnelAnalytics>> {
    // Mock funnel data
    Ok(Json(FunnelAnalytics {
        stages: vec![
            FunnelStage {
                name: "Visitors".to_string(),
                count: 10000,
                percentage: 100.0,
            },
            FunnelStage {
                name: "Leads".to_string(),
                count: 2100,
                percentage: 21.0,
            },
            FunnelStage {
                name: "Qualified".to_string(),
                count: 840,
                percentage: 8.4,
            },
            FunnelStage {
                name: "Opportunities".to_string(),
                count: 252,
                percentage: 2.52,
            },
            FunnelStage {
                name: "Customers".to_string(),
                count: 126,
                percentage: 1.26,
            },
        ],
        overall_conversion_rate: 1.26,
    }))
}
