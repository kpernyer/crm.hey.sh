//! Engagement Scoring - Calculate how engaged a contact is
//!
//! This is PURE BUSINESS LOGIC. Given a set of interactions,
//! calculate an engagement score.
//!
//! The algorithm is:
//! 1. Each interaction type has a base score
//! 2. Recent interactions are weighted more heavily (time decay)
//! 3. Frequency matters - consistent engagement beats one-time spikes
//! 4. Score is normalized to 0-100 range

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Types of interactions that affect engagement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionType {
    EmailSent,
    EmailOpen,
    EmailClick,
    LandingPageVisit,
    FormSubmission,
    EventRegistration,
    EventAttendance,
    MeetingScheduled,
    MeetingAttended,
    CallCompleted,
    NoteAdded,
    SocialInteraction,
}

impl InteractionType {
    /// Base score for this interaction type
    ///
    /// Higher scores = more valuable interactions
    pub fn base_score(&self) -> f64 {
        match self {
            // Passive (we initiated)
            InteractionType::EmailSent => 1.0,
            InteractionType::NoteAdded => 0.5,

            // Responsive (they responded to us)
            InteractionType::EmailOpen => 3.0,
            InteractionType::EmailClick => 5.0,
            InteractionType::LandingPageVisit => 4.0,
            InteractionType::SocialInteraction => 3.0,

            // Active (they took action)
            InteractionType::FormSubmission => 10.0,
            InteractionType::EventRegistration => 8.0,
            InteractionType::MeetingScheduled => 12.0,

            // High-value (they invested significant time)
            InteractionType::EventAttendance => 15.0,
            InteractionType::MeetingAttended => 20.0,
            InteractionType::CallCompleted => 15.0,
        }
    }

    /// Whether this is an inbound interaction (they initiated)
    pub fn is_inbound(&self) -> bool {
        matches!(
            self,
            InteractionType::EmailOpen
                | InteractionType::EmailClick
                | InteractionType::LandingPageVisit
                | InteractionType::FormSubmission
                | InteractionType::EventRegistration
                | InteractionType::EventAttendance
                | InteractionType::MeetingScheduled
                | InteractionType::MeetingAttended
                | InteractionType::SocialInteraction
        )
    }
}

/// A single interaction event
#[derive(Debug, Clone)]
pub struct Interaction {
    pub interaction_type: InteractionType,
    pub occurred_at: DateTime<Utc>,
}

impl Interaction {
    pub fn new(interaction_type: InteractionType, occurred_at: DateTime<Utc>) -> Self {
        Self {
            interaction_type,
            occurred_at,
        }
    }
}

/// Configuration for the engagement scoring algorithm
#[derive(Debug, Clone)]
pub struct EngagementConfig {
    /// How quickly old interactions lose value (days)
    /// After this many days, an interaction is worth 50% of its base score
    pub half_life_days: f64,

    /// Bonus multiplier for having interactions in consecutive weeks
    pub consistency_bonus: f64,

    /// Maximum raw score before normalization
    pub max_raw_score: f64,

    /// Minimum interactions needed for a reliable score
    pub min_interactions: usize,
}

impl Default for EngagementConfig {
    fn default() -> Self {
        Self {
            half_life_days: 30.0,
            consistency_bonus: 1.5,
            max_raw_score: 200.0,
            min_interactions: 3,
        }
    }
}

/// Calculate engagement score from a list of interactions
///
/// # Algorithm
///
/// 1. For each interaction:
///    - Start with base_score for the interaction type
///    - Apply time decay: score * 0.5^(days_ago / half_life)
///
/// 2. Sum all decayed scores
///
/// 3. Apply consistency bonus if interactions span multiple weeks
///
/// 4. Normalize to 0-100 range
///
/// # Example
/// ```
/// use crm_backend::domain::engagement::*;
/// use chrono::Utc;
///
/// let interactions = vec![
///     Interaction::new(InteractionType::EmailOpen, Utc::now()),
///     Interaction::new(InteractionType::EmailClick, Utc::now()),
/// ];
///
/// let score = calculate_engagement_score(&interactions, &EngagementConfig::default());
/// assert!(score >= 0.0 && score <= 100.0);
/// ```
pub fn calculate_engagement_score(
    interactions: &[Interaction],
    config: &EngagementConfig,
) -> f64 {
    if interactions.is_empty() {
        return 0.0;
    }

    let now = Utc::now();
    let half_life_seconds = config.half_life_days * 24.0 * 60.0 * 60.0;

    // Calculate time-decayed score for each interaction
    let mut raw_score = 0.0;

    for interaction in interactions {
        let base = interaction.interaction_type.base_score();

        // Calculate time decay
        let seconds_ago = (now - interaction.occurred_at).num_seconds().max(0) as f64;
        let decay_factor = 0.5_f64.powf(seconds_ago / half_life_seconds);

        raw_score += base * decay_factor;
    }

    // Apply consistency bonus
    let consistency = calculate_consistency_factor(interactions, config);
    raw_score *= consistency;

    // Normalize to 0-100
    let normalized = (raw_score / config.max_raw_score) * 100.0;
    normalized.clamp(0.0, 100.0)
}

/// Calculate a consistency factor based on interaction distribution
///
/// Contacts who engage regularly get a bonus vs those with sporadic activity
fn calculate_consistency_factor(
    interactions: &[Interaction],
    config: &EngagementConfig,
) -> f64 {
    if interactions.len() < config.min_interactions {
        return 1.0; // Not enough data for consistency bonus
    }

    // Count unique weeks with interactions in the last 90 days
    let now = Utc::now();
    let ninety_days_ago = now - Duration::days(90);

    let mut weeks_with_activity = std::collections::HashSet::new();

    for interaction in interactions {
        if interaction.occurred_at >= ninety_days_ago {
            // Calculate week number (0-12)
            let days_ago = (now - interaction.occurred_at).num_days();
            let week = days_ago / 7;
            weeks_with_activity.insert(week);
        }
    }

    // More weeks with activity = higher bonus
    let active_weeks = weeks_with_activity.len() as f64;
    let max_weeks = 13.0; // 90 days ≈ 13 weeks

    // Scale between 1.0 (no bonus) and consistency_bonus (max)
    let factor = 1.0 + (config.consistency_bonus - 1.0) * (active_weeks / max_weeks);
    factor
}

// ============================================================================
// Engagement Insights - Derived metrics from the score
// ============================================================================

/// Engagement level categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngagementLevel {
    Cold,      // 0-20
    Warming,   // 21-40
    Engaged,   // 41-60
    Hot,       // 61-80
    Champion,  // 81-100
}

impl EngagementLevel {
    pub fn from_score(score: f64) -> Self {
        match score as u32 {
            0..=20 => EngagementLevel::Cold,
            21..=40 => EngagementLevel::Warming,
            41..=60 => EngagementLevel::Engaged,
            61..=80 => EngagementLevel::Hot,
            _ => EngagementLevel::Champion,
        }
    }

    /// Get recommended next action for this engagement level
    pub fn recommended_action(&self) -> &'static str {
        match self {
            EngagementLevel::Cold => "Re-engagement campaign or remove from active lists",
            EngagementLevel::Warming => "Nurture with valuable content",
            EngagementLevel::Engaged => "Invite to events, offer demos",
            EngagementLevel::Hot => "Direct sales outreach, schedule call",
            EngagementLevel::Champion => "Referral request, case study opportunity",
        }
    }
}

/// Trend direction for engagement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngagementTrend {
    Declining,
    Stable,
    Improving,
}

/// Calculate the trend by comparing recent vs older engagement
pub fn calculate_engagement_trend(
    interactions: &[Interaction],
    config: &EngagementConfig,
) -> EngagementTrend {
    let now = Utc::now();
    let thirty_days_ago = now - Duration::days(30);
    let sixty_days_ago = now - Duration::days(60);

    // Split into recent (last 30 days) and older (30-60 days ago)
    let recent: Vec<_> = interactions
        .iter()
        .filter(|i| i.occurred_at >= thirty_days_ago)
        .cloned()
        .collect();

    let older: Vec<_> = interactions
        .iter()
        .filter(|i| i.occurred_at >= sixty_days_ago && i.occurred_at < thirty_days_ago)
        .cloned()
        .collect();

    // Calculate scores for each period
    let recent_score = calculate_engagement_score(&recent, config);
    let older_score = calculate_engagement_score(&older, config);

    // Determine trend
    let diff = recent_score - older_score;

    if diff > 10.0 {
        EngagementTrend::Improving
    } else if diff < -10.0 {
        EngagementTrend::Declining
    } else {
        EngagementTrend::Stable
    }
}

// ============================================================================
// YOUR TURN: Implement these functions
// ============================================================================

/// Calculate engagement velocity - how fast is engagement changing?
///
/// Returns a value indicating the rate of change:
/// - Positive: engagement is accelerating
/// - Zero: engagement is stable
/// - Negative: engagement is decelerating
///
/// # Algorithm Hint:
/// Compare the score change in the last 15 days vs the prior 15 days.
/// Velocity = recent_change - older_change
///
/// YOUR IMPLEMENTATION:
pub fn calculate_engagement_velocity(
    _interactions: &[Interaction],
    _config: &EngagementConfig,
) -> f64 {
    // TODO: Implement this function
    //
    // Steps:
    // 1. Split interactions into 3 periods: last 15 days, 15-30 days, 30-45 days
    // 2. Calculate score for each period
    // 3. recent_change = score_0_15 - score_15_30
    // 4. older_change = score_15_30 - score_30_45
    // 5. velocity = recent_change - older_change
    //
    // Test by running: cargo test test_engagement_velocity

    todo!("Implement calculate_engagement_velocity")
}

/// Identify the most impactful interaction types for a contact
///
/// Returns interaction types sorted by their contribution to the total score.
/// Useful for understanding what's driving engagement.
///
/// YOUR IMPLEMENTATION:
pub fn identify_top_interaction_types(
    _interactions: &[Interaction],
    _config: &EngagementConfig,
    _top_n: usize,
) -> Vec<(InteractionType, f64)> {
    // TODO: Implement this function
    //
    // Steps:
    // 1. Group interactions by type
    // 2. Calculate the decayed score contribution for each type
    // 3. Sort by contribution descending
    // 4. Return top N
    //
    // Test by running: cargo test test_top_interaction_types

    todo!("Implement identify_top_interaction_types")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn make_interaction(
        interaction_type: InteractionType,
        days_ago: i64,
    ) -> Interaction {
        Interaction::new(
            interaction_type,
            Utc::now() - Duration::days(days_ago),
        )
    }

    // ---- Basic Scoring Tests ----

    #[test]
    fn test_empty_interactions_returns_zero() {
        let score = calculate_engagement_score(&[], &EngagementConfig::default());
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_recent_interaction_scores_higher_than_old() {
        let config = EngagementConfig::default();

        // Same interaction type, different times
        let recent = vec![make_interaction(InteractionType::EmailOpen, 1)];
        let old = vec![make_interaction(InteractionType::EmailOpen, 60)];

        let recent_score = calculate_engagement_score(&recent, &config);
        let old_score = calculate_engagement_score(&old, &config);

        assert!(
            recent_score > old_score,
            "Recent score {} should be > old score {}",
            recent_score,
            old_score
        );
    }

    #[test]
    fn test_high_value_interactions_score_higher() {
        let config = EngagementConfig::default();

        // Meeting attended vs email sent (same day)
        let meeting = vec![make_interaction(InteractionType::MeetingAttended, 0)];
        let email = vec![make_interaction(InteractionType::EmailSent, 0)];

        let meeting_score = calculate_engagement_score(&meeting, &config);
        let email_score = calculate_engagement_score(&email, &config);

        assert!(
            meeting_score > email_score,
            "Meeting score {} should be > email score {}",
            meeting_score,
            email_score
        );
    }

    #[test]
    fn test_score_is_clamped_to_100() {
        let config = EngagementConfig::default();

        // Lots of high-value interactions should still cap at 100
        let interactions: Vec<_> = (0..50)
            .map(|i| make_interaction(InteractionType::MeetingAttended, i))
            .collect();

        let score = calculate_engagement_score(&interactions, &config);
        assert!(score <= 100.0, "Score {} should be <= 100", score);
    }

    // ---- Engagement Level Tests ----

    #[test]
    fn test_engagement_levels() {
        assert_eq!(EngagementLevel::from_score(0.0), EngagementLevel::Cold);
        assert_eq!(EngagementLevel::from_score(15.0), EngagementLevel::Cold);
        assert_eq!(EngagementLevel::from_score(25.0), EngagementLevel::Warming);
        assert_eq!(EngagementLevel::from_score(50.0), EngagementLevel::Engaged);
        assert_eq!(EngagementLevel::from_score(75.0), EngagementLevel::Hot);
        assert_eq!(EngagementLevel::from_score(90.0), EngagementLevel::Champion);
    }

    // ---- Trend Tests ----

    #[test]
    fn test_improving_trend() {
        let config = EngagementConfig::default();

        // Lots of recent activity, little old activity
        let mut interactions = vec![];

        // Recent: many interactions in last 30 days
        for i in 0..10 {
            interactions.push(make_interaction(InteractionType::EmailOpen, i));
        }

        // Old: few interactions 30-60 days ago
        interactions.push(make_interaction(InteractionType::EmailOpen, 45));

        let trend = calculate_engagement_trend(&interactions, &config);
        assert_eq!(trend, EngagementTrend::Improving);
    }

    #[test]
    fn test_declining_trend() {
        let config = EngagementConfig::default();

        // Little recent activity, lots of old activity
        let mut interactions = vec![];

        // Recent: one interaction
        interactions.push(make_interaction(InteractionType::EmailOpen, 5));

        // Old: many interactions 30-60 days ago
        for i in 35..50 {
            interactions.push(make_interaction(InteractionType::EmailClick, i));
        }

        let trend = calculate_engagement_trend(&interactions, &config);
        assert_eq!(trend, EngagementTrend::Declining);
    }

    // ---- Interaction Type Tests ----

    #[test]
    fn test_inbound_vs_outbound() {
        assert!(!InteractionType::EmailSent.is_inbound());
        assert!(InteractionType::EmailOpen.is_inbound());
        assert!(InteractionType::FormSubmission.is_inbound());
        assert!(!InteractionType::NoteAdded.is_inbound());
    }

    // ---- YOUR TESTS ----

    #[test]
    #[ignore] // Remove after implementing calculate_engagement_velocity
    fn test_engagement_velocity() {
        let config = EngagementConfig::default();

        // Accelerating engagement: more recent activity
        let mut accelerating = vec![];
        for i in 0..5 {
            accelerating.push(make_interaction(InteractionType::EmailClick, i));
        }
        for i in 20..22 {
            accelerating.push(make_interaction(InteractionType::EmailClick, i));
        }
        for i in 35..36 {
            accelerating.push(make_interaction(InteractionType::EmailClick, i));
        }

        let velocity = calculate_engagement_velocity(&accelerating, &config);
        assert!(velocity > 0.0, "Velocity should be positive for accelerating engagement");

        // Decelerating engagement: less recent activity
        let mut decelerating = vec![];
        for i in 0..2 {
            decelerating.push(make_interaction(InteractionType::EmailClick, i));
        }
        for i in 20..25 {
            decelerating.push(make_interaction(InteractionType::EmailClick, i));
        }
        for i in 35..40 {
            decelerating.push(make_interaction(InteractionType::EmailClick, i));
        }

        let velocity = calculate_engagement_velocity(&decelerating, &config);
        assert!(velocity < 0.0, "Velocity should be negative for decelerating engagement");
    }

    #[test]
    #[ignore] // Remove after implementing identify_top_interaction_types
    fn test_top_interaction_types() {
        let config = EngagementConfig::default();

        let interactions = vec![
            make_interaction(InteractionType::EmailSent, 0),
            make_interaction(InteractionType::EmailSent, 1),
            make_interaction(InteractionType::EmailSent, 2),
            make_interaction(InteractionType::MeetingAttended, 0),
            make_interaction(InteractionType::EmailClick, 0),
        ];

        let top = identify_top_interaction_types(&interactions, &config, 2);

        // MeetingAttended has highest base score (20), should be first
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, InteractionType::MeetingAttended);
    }
}
