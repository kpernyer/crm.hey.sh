//! MCP Tool definitions for CRM
//!
//! This module defines all available tools that LLMs can use to interact with the CRM.

use serde_json::{json, Value};

use crate::protocol::ToolDefinition;

/// Get all available tool definitions
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        // Contact tools
        search_contacts_tool(),
        get_contact_details_tool(),
        create_contact_tool(),
        update_contact_tool(),
        log_interaction_tool(),
        // Campaign tools
        suggest_campaign_contacts_tool(),
        draft_campaign_content_tool(),
        // Analytics tools
        get_pipeline_summary_tool(),
        get_engagement_insights_tool(),
    ]
}

fn search_contacts_tool() -> ToolDefinition {
    ToolDefinition {
        name: "search_contacts".into(),
        description: "Search CRM contacts by name, company, status, tags, or engagement level. \
            Use this to find people matching specific criteria. Returns contact summaries with IDs \
            for further operations.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Free-text search across name, email, company"
                },
                "status": {
                    "type": "string",
                    "enum": ["lead", "customer", "partner", "investor"],
                    "description": "Filter by pipeline status"
                },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Filter by tags (e.g., ['techcrunch-2024', 'founder'])"
                },
                "min_engagement": {
                    "type": "number",
                    "description": "Minimum engagement score (0-100)"
                },
                "limit": {
                    "type": "integer",
                    "default": 20,
                    "description": "Maximum results to return"
                }
            }
        }),
    }
}

fn get_contact_details_tool() -> ToolDefinition {
    ToolDefinition {
        name: "get_contact_details".into(),
        description: "Get full details and recent interaction history for a specific contact. \
            Use after search_contacts to dive deeper into a contact's profile and relationship history.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "contact_id": {
                    "type": "string",
                    "description": "Contact ID from search results"
                },
                "include_timeline": {
                    "type": "boolean",
                    "default": true,
                    "description": "Include recent interactions"
                },
                "timeline_limit": {
                    "type": "integer",
                    "default": 10,
                    "description": "Number of timeline entries to include"
                }
            },
            "required": ["contact_id"]
        }),
    }
}

fn create_contact_tool() -> ToolDefinition {
    ToolDefinition {
        name: "create_contact".into(),
        description: "Add a new contact to the CRM. Use when you learn about a new person \
            the user wants to track. At minimum requires first and last name.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "first_name": {
                    "type": "string",
                    "description": "Contact's first name"
                },
                "last_name": {
                    "type": "string",
                    "description": "Contact's last name"
                },
                "email": {
                    "type": "string",
                    "description": "Email address"
                },
                "phone": {
                    "type": "string",
                    "description": "Phone number"
                },
                "company": {
                    "type": "string",
                    "description": "Company name"
                },
                "linkedin_url": {
                    "type": "string",
                    "description": "LinkedIn profile URL"
                },
                "status": {
                    "type": "string",
                    "enum": ["lead", "customer", "partner", "investor"],
                    "default": "lead",
                    "description": "Initial pipeline status"
                },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Tags to categorize the contact"
                },
                "notes": {
                    "type": "string",
                    "description": "Initial notes about the contact"
                }
            },
            "required": ["first_name", "last_name"]
        }),
    }
}

fn update_contact_tool() -> ToolDefinition {
    ToolDefinition {
        name: "update_contact".into(),
        description: "Update a contact's information or status. Use to move contacts through \
            the pipeline, update their details, or add/modify tags.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "contact_id": {
                    "type": "string",
                    "description": "Contact ID to update"
                },
                "first_name": { "type": "string" },
                "last_name": { "type": "string" },
                "email": { "type": "string" },
                "phone": { "type": "string" },
                "company": { "type": "string" },
                "linkedin_url": { "type": "string" },
                "status": {
                    "type": "string",
                    "enum": ["lead", "customer", "partner", "investor", "other"],
                    "description": "New pipeline status"
                },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Replace all existing tags"
                },
                "add_tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Add to existing tags (without removing)"
                },
                "remove_tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Remove specific tags"
                }
            },
            "required": ["contact_id"]
        }),
    }
}

fn log_interaction_tool() -> ToolDefinition {
    ToolDefinition {
        name: "log_interaction".into(),
        description: "Record an interaction with a contact (meeting, call, email, note). \
            Always log interactions to maintain relationship context and history. \
            This helps track engagement and provides context for future conversations.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "contact_id": {
                    "type": "string",
                    "description": "Contact ID"
                },
                "type": {
                    "type": "string",
                    "enum": ["email_sent", "email_received", "call", "meeting", "note", "social_touch", "event"],
                    "description": "Type of interaction"
                },
                "content": {
                    "type": "string",
                    "description": "Summary or content of the interaction"
                },
                "metadata": {
                    "type": "object",
                    "description": "Additional structured data (e.g., meeting duration, topics discussed, location)",
                    "properties": {
                        "duration_minutes": { "type": "integer" },
                        "location": { "type": "string" },
                        "topics": {
                            "type": "array",
                            "items": { "type": "string" }
                        },
                        "sentiment": {
                            "type": "string",
                            "enum": ["positive", "neutral", "negative"]
                        },
                        "follow_up_needed": { "type": "boolean" }
                    }
                }
            },
            "required": ["contact_id", "type", "content"]
        }),
    }
}

fn suggest_campaign_contacts_tool() -> ToolDefinition {
    ToolDefinition {
        name: "suggest_campaign_contacts".into(),
        description: "Get AI-suggested contacts for a campaign based on objective and criteria. \
            Use before creating outreach campaigns to identify the best targets.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "objective": {
                    "type": "string",
                    "enum": ["awareness", "lead_gen", "event", "investor", "early_adopters"],
                    "description": "Campaign goal"
                },
                "criteria": {
                    "type": "string",
                    "description": "Natural language description of ideal contacts (e.g., 'founders at seed-stage startups in fintech')"
                },
                "exclude_tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Tags to exclude from results"
                },
                "min_engagement": {
                    "type": "number",
                    "description": "Minimum engagement score"
                },
                "limit": {
                    "type": "integer",
                    "default": 50,
                    "description": "Maximum contacts to suggest"
                }
            },
            "required": ["objective"]
        }),
    }
}

fn draft_campaign_content_tool() -> ToolDefinition {
    ToolDefinition {
        name: "draft_campaign_content".into(),
        description: "Generate draft content for a campaign (email, social post, landing page). \
            Returns editable drafts that can be reviewed and customized before sending.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "content_type": {
                    "type": "string",
                    "enum": ["email", "social_post", "landing_page", "event_invite"],
                    "description": "Type of content to generate"
                },
                "context": {
                    "type": "string",
                    "description": "What the campaign is about, key messages to convey"
                },
                "tone": {
                    "type": "string",
                    "enum": ["professional", "casual", "urgent", "friendly", "formal"],
                    "default": "professional",
                    "description": "Desired tone of the content"
                },
                "target_audience": {
                    "type": "string",
                    "description": "Who this content is for (e.g., 'early-stage founders', 'enterprise CTOs')"
                },
                "call_to_action": {
                    "type": "string",
                    "description": "Desired action (e.g., 'schedule a demo', 'register for event')"
                },
                "length": {
                    "type": "string",
                    "enum": ["short", "medium", "long"],
                    "default": "medium",
                    "description": "Desired length of content"
                }
            },
            "required": ["content_type", "context"]
        }),
    }
}

fn get_pipeline_summary_tool() -> ToolDefinition {
    ToolDefinition {
        name: "get_pipeline_summary".into(),
        description: "Get current pipeline status - how many contacts in each stage, \
            conversion rates, and engagement trends. Useful for understanding overall CRM health.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "time_range": {
                    "type": "string",
                    "enum": ["7d", "30d", "90d", "all"],
                    "default": "30d",
                    "description": "Time range for trend data"
                },
                "include_trends": {
                    "type": "boolean",
                    "default": true,
                    "description": "Include week-over-week trends"
                }
            }
        }),
    }
}

fn get_engagement_insights_tool() -> ToolDefinition {
    ToolDefinition {
        name: "get_engagement_insights".into(),
        description: "Identify contacts needing attention - stale leads, highly engaged prospects, \
            recent converts, or contacts needing follow-up. Helps prioritize outreach.".into(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "insight_type": {
                    "type": "string",
                    "enum": ["stale_leads", "hot_prospects", "recent_activity", "needs_followup", "at_risk"],
                    "description": "Type of insight to retrieve"
                },
                "days_threshold": {
                    "type": "integer",
                    "default": 30,
                    "description": "Days threshold for stale/recent calculations"
                },
                "limit": {
                    "type": "integer",
                    "default": 10,
                    "description": "Maximum contacts to return"
                }
            },
            "required": ["insight_type"]
        }),
    }
}
