use crate::models::TimelineEntry;

/// Summarize a contact's timeline entries
/// This is a stub that returns template-based mock data
/// In production, this would call an AI service
pub async fn summarize_timeline(entries: &[TimelineEntry]) -> String {
    if entries.is_empty() {
        return "No interactions recorded yet.".to_string();
    }

    let total = entries.len();

    // Count by type
    let mut emails_sent = 0;
    let mut emails_opened = 0;
    let mut notes = 0;
    let mut events = 0;
    let mut calls = 0;
    let mut landing_pages = 0;

    for entry in entries {
        match entry.entry_type {
            crate::models::TimelineEntryType::EmailSent => emails_sent += 1,
            crate::models::TimelineEntryType::EmailOpen => emails_opened += 1,
            crate::models::TimelineEntryType::EmailClick => emails_opened += 1,
            crate::models::TimelineEntryType::Note => notes += 1,
            crate::models::TimelineEntryType::EventInvite | crate::models::TimelineEntryType::EventAttend => events += 1,
            crate::models::TimelineEntryType::Call => calls += 1,
            crate::models::TimelineEntryType::LandingPageVisit => landing_pages += 1,
            _ => {}
        }
    }

    let mut summary_parts = vec![
        format!("This contact has {} total interactions", total),
    ];

    if emails_sent > 0 {
        let open_rate = if emails_sent > 0 {
            (emails_opened as f64 / emails_sent as f64 * 100.0).round()
        } else {
            0.0
        };
        summary_parts.push(format!("{} emails sent ({}% engagement)", emails_sent, open_rate));
    }

    if calls > 0 {
        summary_parts.push(format!("{} calls logged", calls));
    }

    if events > 0 {
        summary_parts.push(format!("{} event interactions", events));
    }

    if notes > 0 {
        summary_parts.push(format!("{} notes recorded", notes));
    }

    if landing_pages > 0 {
        summary_parts.push(format!("{} landing page visits", landing_pages));
    }

    // Get most recent activity
    if let Some(latest) = entries.first() {
        summary_parts.push(format!(
            "Most recent activity: {} on {}",
            latest.content,
            latest.timestamp.format("%B %d, %Y")
        ));
    }

    summary_parts.join(". ") + "."
}

/// Generate engagement insights for a contact
pub async fn generate_engagement_insights(entries: &[TimelineEntry], engagement_score: f64) -> EngagementInsights {
    let trend = if entries.len() < 5 {
        EngagementTrend::New
    } else {
        // Simple heuristic based on recent vs older activity
        let recent_count = entries.iter().take(entries.len() / 2).count();
        let older_count = entries.len() - recent_count;

        if recent_count > older_count {
            EngagementTrend::Increasing
        } else if recent_count < older_count {
            EngagementTrend::Decreasing
        } else {
            EngagementTrend::Stable
        }
    };

    let recommendation = match (engagement_score as i32, &trend) {
        (0..=30, _) => "Consider reaching out with a personalized message to re-engage this contact.",
        (31..=60, EngagementTrend::Decreasing) => "Engagement is declining. Schedule a check-in call or send relevant content.",
        (31..=60, _) => "Moderate engagement. Continue nurturing with valuable content.",
        (61..=80, _) => "Good engagement level. Consider inviting to exclusive events or early access programs.",
        (81..=100, _) => "Highly engaged! This contact may be ready for a sales conversation or partnership discussion.",
        _ => "Continue building the relationship with consistent, valuable touchpoints.",
    };

    EngagementInsights {
        score: engagement_score,
        trend,
        recommendation: recommendation.to_string(),
        next_best_action: determine_next_action(entries, engagement_score),
    }
}

fn determine_next_action(entries: &[TimelineEntry], score: f64) -> String {
    if entries.is_empty() {
        return "Send an introductory email".to_string();
    }

    let last_entry = entries.first().unwrap();
    let days_since = (chrono::Utc::now() - last_entry.timestamp).num_days();

    if days_since > 30 {
        "Re-engage with a check-in message".to_string()
    } else if score > 70.0 {
        "Schedule a call or meeting".to_string()
    } else {
        "Share relevant content or invite to upcoming event".to_string()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EngagementInsights {
    pub score: f64,
    pub trend: EngagementTrend,
    pub recommendation: String,
    pub next_best_action: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EngagementTrend {
    Increasing,
    Stable,
    Decreasing,
    New,
}
