use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Option<Thing>,
    pub name: String,
    pub domain: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCompanyRequest {
    pub name: String,
    pub domain: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCompanyRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyQuery {
    pub search: Option<String>,
    pub industry: Option<String>,
    pub tags: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct CompanyResponse {
    pub id: String,
    pub name: String,
    pub domain: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Company> for CompanyResponse {
    fn from(c: Company) -> Self {
        Self {
            id: c.id.map(|t| t.id.to_string()).unwrap_or_default(),
            name: c.name,
            domain: c.domain,
            industry: c.industry,
            size: c.size,
            tags: c.tags,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}
