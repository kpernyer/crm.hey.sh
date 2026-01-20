//! MCP request handlers
//!
//! Handles JSON-RPC requests and dispatches to appropriate tool implementations.

use serde_json::{json, Value};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use tracing::{debug, error, info};

use crate::config::Config;
use crate::error::McpError;
use crate::protocol::*;
use crate::tools::get_tool_definitions;

/// Initialize database connection
pub async fn init_db(config: &Config) -> Result<Surreal<Client>, McpError> {
    let db = Surreal::new::<Ws>(&config.db_url)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    db.use_ns(&config.db_namespace)
        .use_db(&config.db_name)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    info!("Connected to database: {}", config.db_url);
    Ok(db)
}

/// Handle incoming JSON-RPC request
pub async fn handle_request(db: &Surreal<Client>, request: JsonRpcRequest) -> JsonRpcResponse {
    debug!("Handling request: {}", request.method);

    match request.method.as_str() {
        "initialize" => handle_initialize(request.id),
        "initialized" => JsonRpcResponse::success(request.id, json!({})),
        "tools/list" => handle_list_tools(request.id),
        "tools/call" => handle_call_tool(db, request.id, request.params).await,
        "resources/list" => handle_list_resources(request.id),
        "resources/read" => handle_read_resource(db, request.id, request.params).await,
        "ping" => JsonRpcResponse::success(request.id, json!({})),
        _ => {
            error!("Unknown method: {}", request.method);
            JsonRpcResponse::error(
                request.id,
                -32601,
                format!("Method not found: {}", request.method),
            )
        }
    }
}

fn handle_initialize(id: Option<Value>) -> JsonRpcResponse {
    let result = InitializeResult {
        protocol_version: "2024-11-05".into(),
        capabilities: ServerCapabilities {
            tools: ToolsCapability { list_changed: false },
            resources: ResourcesCapability {
                subscribe: false,
                list_changed: false,
            },
        },
        server_info: ServerInfo {
            name: "crm-mcp-server".into(),
            version: env!("CARGO_PKG_VERSION").into(),
        },
    };
    JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
}

fn handle_list_tools(id: Option<Value>) -> JsonRpcResponse {
    let tools = get_tool_definitions();
    JsonRpcResponse::success(id, json!({ "tools": tools }))
}

async fn handle_call_tool(
    db: &Surreal<Client>,
    id: Option<Value>,
    params: Option<Value>,
) -> JsonRpcResponse {
    let params = match params {
        Some(p) => p,
        None => {
            return JsonRpcResponse::error(id, -32602, "Missing params".into());
        }
    };

    let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

    info!("Calling tool: {} with args: {}", tool_name, arguments);

    let result = match tool_name {
        "search_contacts" => search_contacts(db, arguments).await,
        "get_contact_details" => get_contact_details(db, arguments).await,
        "create_contact" => create_contact(db, arguments).await,
        "update_contact" => update_contact(db, arguments).await,
        "log_interaction" => log_interaction(db, arguments).await,
        "suggest_campaign_contacts" => suggest_campaign_contacts(db, arguments).await,
        "draft_campaign_content" => draft_campaign_content(arguments).await,
        "get_pipeline_summary" => get_pipeline_summary(db, arguments).await,
        "get_engagement_insights" => get_engagement_insights(db, arguments).await,
        _ => Err(McpError::ToolNotFound(tool_name.into())),
    };

    match result {
        Ok(content) => JsonRpcResponse::success(
            id,
            json!({
                "content": [{ "type": "text", "text": content }]
            }),
        ),
        Err(e) => JsonRpcResponse::success(
            id,
            json!({
                "content": [{ "type": "text", "text": format!("Error: {}", e) }],
                "isError": true
            }),
        ),
    }
}

fn handle_list_resources(id: Option<Value>) -> JsonRpcResponse {
    let resources = vec![
        ResourceDefinition {
            uri: "crm://contacts/recent".into(),
            name: "Recent Contacts".into(),
            description: "Contacts added or updated in the last 7 days".into(),
            mime_type: "application/json".into(),
        },
        ResourceDefinition {
            uri: "crm://pipeline/summary".into(),
            name: "Pipeline Summary".into(),
            description: "Current contact counts by status".into(),
            mime_type: "application/json".into(),
        },
    ];
    JsonRpcResponse::success(id, json!({ "resources": resources }))
}

async fn handle_read_resource(
    db: &Surreal<Client>,
    id: Option<Value>,
    params: Option<Value>,
) -> JsonRpcResponse {
    let uri = params
        .as_ref()
        .and_then(|p| p.get("uri"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let result = match uri {
        "crm://contacts/recent" => get_recent_contacts(db).await,
        "crm://pipeline/summary" => get_pipeline_summary(db, json!({})).await,
        _ => Err(McpError::InvalidRequest(format!("Unknown resource: {}", uri))),
    };

    match result {
        Ok(content) => JsonRpcResponse::success(
            id,
            json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": content
                }]
            }),
        ),
        Err(e) => JsonRpcResponse::error(id, e.error_code(), e.to_string()),
    }
}

// =============================================================================
// Tool Implementations
// =============================================================================

async fn search_contacts(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let query = args.get("query").and_then(|v| v.as_str());
    let status = args.get("status").and_then(|v| v.as_str());
    let tags: Option<Vec<&str>> = args
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect());
    let min_engagement = args.get("min_engagement").and_then(|v| v.as_f64());
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20);

    // Build SurrealQL query
    let mut conditions = Vec::new();
    let mut bindings: Vec<(&str, Value)> = Vec::new();

    if let Some(q) = query {
        conditions.push("(first_name CONTAINS $query OR last_name CONTAINS $query OR email CONTAINS $query)");
        bindings.push(("query", json!(q)));
    }

    if let Some(s) = status {
        conditions.push("status = $status");
        bindings.push(("status", json!(s)));
    }

    if let Some(t) = &tags {
        conditions.push("tags CONTAINSANY $tags");
        bindings.push(("tags", json!(t)));
    }

    if let Some(e) = min_engagement {
        conditions.push("engagement_score >= $min_engagement");
        bindings.push(("min_engagement", json!(e)));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT id, first_name, last_name, email, status, tags, engagement_score, company FROM contact {} ORDER BY engagement_score DESC LIMIT {}",
        where_clause, limit
    );

    let mut query_builder = db.query(&sql);
    for (name, value) in bindings {
        query_builder = query_builder.bind((name, value));
    }

    let mut result = query_builder
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let contacts: Vec<Value> = result.take(0).map_err(|e| McpError::Database(e.to_string()))?;

    let response = json!({
        "contacts": contacts,
        "count": contacts.len(),
        "query_params": {
            "query": query,
            "status": status,
            "tags": tags,
            "min_engagement": min_engagement,
            "limit": limit
        }
    });

    Ok(serde_json::to_string_pretty(&response).unwrap())
}

async fn get_contact_details(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let contact_id = args
        .get("contact_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("contact_id is required".into()))?;

    let include_timeline = args
        .get("include_timeline")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let timeline_limit = args
        .get("timeline_limit")
        .and_then(|v| v.as_u64())
        .unwrap_or(10);

    // Get contact
    let contact: Option<Value> = db
        .select(("contact", contact_id))
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let contact = contact.ok_or_else(|| McpError::InvalidParams("Contact not found".into()))?;

    let mut response = json!({ "contact": contact });

    // Get timeline if requested
    if include_timeline {
        let sql = format!(
            "SELECT * FROM timeline_entry WHERE contact = contact:{} ORDER BY timestamp DESC LIMIT {}",
            contact_id, timeline_limit
        );
        let mut result = db
            .query(&sql)
            .await
            .map_err(|e| McpError::Database(e.to_string()))?;
        let timeline: Vec<Value> = result.take(0).map_err(|e| McpError::Database(e.to_string()))?;
        response["timeline"] = json!(timeline);
    }

    Ok(serde_json::to_string_pretty(&response).unwrap())
}

async fn create_contact(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let first_name = args
        .get("first_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("first_name is required".into()))?;
    let last_name = args
        .get("last_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("last_name is required".into()))?;

    let contact = json!({
        "first_name": first_name,
        "last_name": last_name,
        "email": args.get("email"),
        "phone": args.get("phone"),
        "company": args.get("company"),
        "linkedin_url": args.get("linkedin_url"),
        "status": args.get("status").and_then(|v| v.as_str()).unwrap_or("lead"),
        "tags": args.get("tags").unwrap_or(&json!([])),
        "engagement_score": 0.0,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let created: Vec<Value> = db
        .create("contact")
        .content(contact)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let created = created.first().cloned().unwrap_or(json!(null));

    // Log initial note if provided
    if let Some(notes) = args.get("notes").and_then(|v| v.as_str()) {
        if !notes.is_empty() {
            if let Some(id) = created.get("id") {
                let timeline_entry = json!({
                    "contact": id,
                    "type": "note",
                    "content": notes,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });
                let _: Vec<Value> = db
                    .create("timeline_entry")
                    .content(timeline_entry)
                    .await
                    .map_err(|e| McpError::Database(e.to_string()))?;
            }
        }
    }

    Ok(serde_json::to_string_pretty(&json!({
        "success": true,
        "contact": created,
        "message": format!("Created contact: {} {}", first_name, last_name)
    }))
    .unwrap())
}

async fn update_contact(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let contact_id = args
        .get("contact_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("contact_id is required".into()))?;

    // Build update object
    let mut updates = json!({
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    for field in [
        "first_name",
        "last_name",
        "email",
        "phone",
        "company",
        "linkedin_url",
        "status",
    ] {
        if let Some(value) = args.get(field) {
            updates[field] = value.clone();
        }
    }

    // Handle tag operations
    if let Some(tags) = args.get("tags") {
        updates["tags"] = tags.clone();
    }
    // TODO: Handle add_tags and remove_tags with MERGE operations

    let updated: Option<Value> = db
        .update(("contact", contact_id))
        .merge(updates)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    Ok(serde_json::to_string_pretty(&json!({
        "success": true,
        "contact": updated,
        "message": "Contact updated successfully"
    }))
    .unwrap())
}

async fn log_interaction(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let contact_id = args
        .get("contact_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("contact_id is required".into()))?;
    let interaction_type = args
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("type is required".into()))?;
    let content = args
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("content is required".into()))?;

    let entry = json!({
        "contact": format!("contact:{}", contact_id),
        "type": interaction_type,
        "content": content,
        "metadata": args.get("metadata").unwrap_or(&json!({})),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let created: Vec<Value> = db
        .create("timeline_entry")
        .content(entry)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    // Update contact's engagement score (simple increment)
    let _: Option<Value> = db
        .query("UPDATE contact SET engagement_score += 1, updated_at = $now WHERE id = $id")
        .bind(("id", format!("contact:{}", contact_id)))
        .bind(("now", chrono::Utc::now().to_rfc3339()))
        .await
        .map_err(|e| McpError::Database(e.to_string()))?
        .take::<Option<Value>>(0)
        .ok()
        .flatten();

    Ok(serde_json::to_string_pretty(&json!({
        "success": true,
        "timeline_entry": created.first(),
        "message": format!("Logged {} interaction for contact", interaction_type)
    }))
    .unwrap())
}

async fn suggest_campaign_contacts(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let objective = args
        .get("objective")
        .and_then(|v| v.as_str())
        .unwrap_or("lead_gen");
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(50);

    // Map objective to status/criteria
    let (status_filter, engagement_threshold) = match objective {
        "investor" => (Some("investor"), 0.0),
        "early_adopters" => (Some("lead"), 50.0),
        "lead_gen" => (Some("lead"), 0.0),
        "event" => (None, 30.0),
        _ => (None, 0.0),
    };

    let mut conditions = vec!["engagement_score >= $threshold".to_string()];
    if let Some(status) = status_filter {
        conditions.push(format!("status = '{}'", status));
    }

    let sql = format!(
        "SELECT id, first_name, last_name, email, status, tags, engagement_score FROM contact WHERE {} ORDER BY engagement_score DESC LIMIT {}",
        conditions.join(" AND "),
        limit
    );

    let mut result = db
        .query(&sql)
        .bind(("threshold", engagement_threshold))
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let contacts: Vec<Value> = result.take(0).map_err(|e| McpError::Database(e.to_string()))?;

    Ok(serde_json::to_string_pretty(&json!({
        "objective": objective,
        "suggested_contacts": contacts,
        "count": contacts.len(),
        "criteria_applied": {
            "status": status_filter,
            "min_engagement": engagement_threshold
        }
    }))
    .unwrap())
}

async fn draft_campaign_content(args: Value) -> Result<String, McpError> {
    let content_type = args
        .get("content_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("content_type is required".into()))?;
    let context = args
        .get("context")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("context is required".into()))?;
    let tone = args
        .get("tone")
        .and_then(|v| v.as_str())
        .unwrap_or("professional");
    let audience = args
        .get("target_audience")
        .and_then(|v| v.as_str())
        .unwrap_or("general audience");
    let cta = args
        .get("call_to_action")
        .and_then(|v| v.as_str())
        .unwrap_or("learn more");

    // This is a stub - in production, this would call an AI service
    let draft = match content_type {
        "email" => format!(
            "Subject: {}\n\nHi [Name],\n\n{}\n\nWould you like to {}?\n\nBest regards",
            context.split_whitespace().take(5).collect::<Vec<_>>().join(" "),
            context,
            cta
        ),
        "social_post" => format!(
            "🚀 {} \n\n{} \n\n#startup #founder",
            context.split_whitespace().take(10).collect::<Vec<_>>().join(" "),
            cta
        ),
        "landing_page" => format!(
            "# {}\n\n{}\n\n## For {}\n\n[{}]",
            context.split_whitespace().take(5).collect::<Vec<_>>().join(" "),
            context,
            audience,
            cta
        ),
        "event_invite" => format!(
            "You're invited!\n\n{}\n\nJoin us to {}\n\n[RSVP]",
            context, cta
        ),
        _ => format!("{}", context),
    };

    Ok(serde_json::to_string_pretty(&json!({
        "content_type": content_type,
        "draft": draft,
        "parameters_used": {
            "tone": tone,
            "target_audience": audience,
            "call_to_action": cta
        },
        "note": "This is a draft. Review and customize before sending."
    }))
    .unwrap())
}

async fn get_pipeline_summary(db: &Surreal<Client>, _args: Value) -> Result<String, McpError> {
    let sql = r#"
        SELECT status, count() as count
        FROM contact
        GROUP BY status
    "#;

    let mut result = db
        .query(sql)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let counts: Vec<Value> = result.take(0).map_err(|e| McpError::Database(e.to_string()))?;

    // Get total count
    let sql_total = "SELECT count() as total FROM contact";
    let mut total_result = db
        .query(sql_total)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;
    let total: Vec<Value> = total_result
        .take(0)
        .map_err(|e| McpError::Database(e.to_string()))?;

    Ok(serde_json::to_string_pretty(&json!({
        "pipeline": {
            "by_status": counts,
            "total": total.first().and_then(|v| v.get("total")).unwrap_or(&json!(0))
        },
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
    .unwrap())
}

async fn get_engagement_insights(db: &Surreal<Client>, args: Value) -> Result<String, McpError> {
    let insight_type = args
        .get("insight_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| McpError::InvalidParams("insight_type is required".into()))?;
    let days = args
        .get("days_threshold")
        .and_then(|v| v.as_u64())
        .unwrap_or(30);
    let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);

    let sql = match insight_type {
        "hot_prospects" => format!(
            "SELECT * FROM contact WHERE engagement_score >= 70 ORDER BY engagement_score DESC LIMIT {}",
            limit
        ),
        "stale_leads" => format!(
            "SELECT * FROM contact WHERE status = 'lead' AND updated_at < time::now() - {}d ORDER BY updated_at ASC LIMIT {}",
            days, limit
        ),
        "needs_followup" => format!(
            "SELECT * FROM contact WHERE updated_at < time::now() - 7d AND engagement_score > 30 ORDER BY engagement_score DESC LIMIT {}",
            limit
        ),
        "recent_activity" => format!(
            "SELECT * FROM contact ORDER BY updated_at DESC LIMIT {}",
            limit
        ),
        "at_risk" => format!(
            "SELECT * FROM contact WHERE status = 'customer' AND updated_at < time::now() - {}d ORDER BY updated_at ASC LIMIT {}",
            days, limit
        ),
        _ => {
            return Err(McpError::InvalidParams(format!(
                "Unknown insight_type: {}",
                insight_type
            )))
        }
    };

    let mut result = db
        .query(&sql)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let contacts: Vec<Value> = result.take(0).map_err(|e| McpError::Database(e.to_string()))?;

    Ok(serde_json::to_string_pretty(&json!({
        "insight_type": insight_type,
        "contacts": contacts,
        "count": contacts.len(),
        "parameters": {
            "days_threshold": days,
            "limit": limit
        }
    }))
    .unwrap())
}

async fn get_recent_contacts(db: &Surreal<Client>) -> Result<String, McpError> {
    let sql = "SELECT * FROM contact WHERE created_at > time::now() - 7d ORDER BY created_at DESC LIMIT 50";

    let mut result = db
        .query(sql)
        .await
        .map_err(|e| McpError::Database(e.to_string()))?;

    let contacts: Vec<Value> = result.take(0).map_err(|e| McpError::Database(e.to_string()))?;

    Ok(serde_json::to_string_pretty(&json!({
        "recent_contacts": contacts,
        "count": contacts.len(),
        "period": "7 days"
    }))
    .unwrap())
}
