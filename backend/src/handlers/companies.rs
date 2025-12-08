use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;

use crate::error::{AppError, AppResult};
use crate::models::{
    Company, CompanyQuery, CompanyResponse, CreateCompanyRequest, UpdateCompanyRequest,
};
use crate::AppState;

pub async fn list_companies(
    State(state): State<AppState>,
    Query(query): Query<CompanyQuery>,
) -> AppResult<Json<Vec<CompanyResponse>>> {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let companies: Vec<Company> = state
        .db
        .client
        .query("SELECT * FROM company ORDER BY created_at DESC LIMIT $limit START $offset")
        .bind(("limit", limit))
        .bind(("offset", offset))
        .await?
        .take(0)?;

    let responses: Vec<CompanyResponse> = companies.into_iter().map(Into::into).collect();
    Ok(Json(responses))
}

pub async fn create_company(
    State(state): State<AppState>,
    Json(req): Json<CreateCompanyRequest>,
) -> AppResult<Json<CompanyResponse>> {
    let now = Utc::now();

    let companies: Vec<Company> = state
        .db
        .client
        .create("company")
        .content(Company {
            id: None,
            name: req.name,
            domain: req.domain,
            industry: req.industry,
            size: req.size,
            tags: req.tags.unwrap_or_default(),
            created_at: now,
            updated_at: now,
        })
        .await?;

    let company = companies.into_iter().next().ok_or_else(|| AppError::Internal("Failed to create company".into()))?;
    Ok(Json(company.into()))
}

pub async fn get_company(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<CompanyResponse>> {
    let company: Option<Company> = state
        .db
        .client
        .select(("company", id.as_str()))
        .await?;

    let company = company.ok_or_else(|| AppError::NotFound("Company not found".into()))?;
    Ok(Json(company.into()))
}

pub async fn update_company(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateCompanyRequest>,
) -> AppResult<Json<CompanyResponse>> {
    let existing: Option<Company> = state
        .db
        .client
        .select(("company", id.as_str()))
        .await?;

    let mut company = existing.ok_or_else(|| AppError::NotFound("Company not found".into()))?;

    if let Some(name) = req.name {
        company.name = name;
    }
    if let Some(domain) = req.domain {
        company.domain = Some(domain);
    }
    if let Some(industry) = req.industry {
        company.industry = Some(industry);
    }
    if let Some(size) = req.size {
        company.size = Some(size);
    }
    if let Some(tags) = req.tags {
        company.tags = tags;
    }

    company.updated_at = Utc::now();

    let updated: Option<Company> = state
        .db
        .client
        .update(("company", id.as_str()))
        .content(company)
        .await?;

    let company = updated.ok_or_else(|| AppError::Internal("Failed to update company".into()))?;
    Ok(Json(company.into()))
}

pub async fn delete_company(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let _: Option<Company> = state
        .db
        .client
        .delete(("company", id.as_str()))
        .await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}
