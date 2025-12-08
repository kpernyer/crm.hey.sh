use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPost {
    pub platform: SocialPlatform,
    pub content: String,
    pub hashtags: Vec<String>,
    pub suggested_image_prompt: String,
    pub character_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialPlatform {
    Twitter,
    LinkedIn,
    Facebook,
    Instagram,
}

/// Generate social media posts from a prompt
/// This is a stub that returns template-based mock data
/// In production, this would call an AI service
pub async fn generate_social_posts(prompt: &str) -> Vec<GeneratedPost> {
    let base_content = if prompt.len() > 50 {
        &prompt[..50]
    } else {
        prompt
    };

    vec![
        GeneratedPost {
            platform: SocialPlatform::Twitter,
            content: format!(
                "Exciting news! {} - We're building the future of founder-focused CRM. Stay tuned for more updates!",
                base_content
            ),
            hashtags: vec![
                "#startup".to_string(),
                "#CRM".to_string(),
                "#founders".to_string(),
                "#growth".to_string(),
            ],
            suggested_image_prompt: "Modern tech dashboard with growth charts, blue gradient background".to_string(),
            character_count: 140,
        },
        GeneratedPost {
            platform: SocialPlatform::LinkedIn,
            content: format!(
                "🚀 {} \n\nAt hey.sh, we're reimagining how founders manage relationships and drive growth. Our new CRM platform is designed specifically for the unique needs of startup founders.\n\nKey features:\n✅ Contact & company management\n✅ AI-powered campaign builder\n✅ Event tracking & RSVPs\n✅ Real-time analytics\n\nInterested in early access? Drop a comment below!",
                base_content
            ),
            hashtags: vec![
                "#Entrepreneurship".to_string(),
                "#StartupLife".to_string(),
                "#SaaS".to_string(),
                "#B2B".to_string(),
                "#ProductLaunch".to_string(),
            ],
            suggested_image_prompt: "Professional product screenshot showing CRM dashboard with modern UI".to_string(),
            character_count: 500,
        },
        GeneratedPost {
            platform: SocialPlatform::Facebook,
            content: format!(
                "Big things are happening! 🎉\n\n{}\n\nWe've been hard at work building something special for founders and growth teams. Our new CRM platform combines the simplicity you need with the power you want.\n\nFollow our page to be the first to know when we launch!",
                base_content
            ),
            hashtags: vec![
                "#startup".to_string(),
                "#business".to_string(),
                "#technology".to_string(),
            ],
            suggested_image_prompt: "Team collaboration image with modern office aesthetic".to_string(),
            character_count: 300,
        },
        GeneratedPost {
            platform: SocialPlatform::Instagram,
            content: format!(
                "Building the future, one relationship at a time. 💫\n\n{}\n\nLink in bio for early access!",
                base_content
            ),
            hashtags: vec![
                "#startuplife".to_string(),
                "#entrepreneurship".to_string(),
                "#techstartup".to_string(),
                "#saas".to_string(),
                "#crm".to_string(),
                "#growthhacking".to_string(),
                "#b2b".to_string(),
                "#founder".to_string(),
            ],
            suggested_image_prompt: "Minimalist product mockup on gradient background with geometric shapes".to_string(),
            character_count: 150,
        },
    ]
}
