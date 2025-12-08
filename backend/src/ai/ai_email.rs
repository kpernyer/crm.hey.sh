use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedEmail {
    pub subject: String,
    pub preview_text: String,
    pub body_html: String,
    pub body_text: String,
    pub cta_text: String,
    pub cta_url: String,
}

/// Generate an email from a prompt
/// This is a stub that returns template-based mock data
/// In production, this would call an AI service (e.g., Claude API)
pub async fn generate_email(prompt: &str) -> GeneratedEmail {
    // Extract key themes from prompt for personalization
    let is_product_launch = prompt.to_lowercase().contains("launch")
        || prompt.to_lowercase().contains("product");
    let is_event = prompt.to_lowercase().contains("event")
        || prompt.to_lowercase().contains("webinar");
    let is_newsletter = prompt.to_lowercase().contains("newsletter")
        || prompt.to_lowercase().contains("update");

    if is_event {
        GeneratedEmail {
            subject: "You're Invited: Exclusive Event Just for You".to_string(),
            preview_text: "Join us for an exciting event that you won't want to miss".to_string(),
            body_html: format!(
                r##"<html>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
    <h1 style="color: #1a1a1a;">You're Invited!</h1>
    <p style="color: #4a4a4a; line-height: 1.6;">
        Based on your prompt: "{}"
    </p>
    <p style="color: #4a4a4a; line-height: 1.6;">
        We're hosting an exclusive event and would love for you to join us. This is your chance to connect with industry leaders, learn from experts, and be part of something special.
    </p>
    <div style="margin: 30px 0;">
        <a href="#" style="background-color: #0066ff; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; font-weight: 600;">
            Reserve Your Spot
        </a>
    </div>
    <p style="color: #666; font-size: 14px;">
        Space is limited, so don't wait!
    </p>
</body>
</html>"##,
                prompt
            ),
            body_text: format!(
                "You're Invited!\n\nBased on your prompt: \"{}\"\n\nWe're hosting an exclusive event and would love for you to join us.\n\nReserve your spot now!",
                prompt
            ),
            cta_text: "Reserve Your Spot".to_string(),
            cta_url: "https://crm.hey.sh/events/register".to_string(),
        }
    } else if is_product_launch {
        GeneratedEmail {
            subject: "Introducing Something New".to_string(),
            preview_text: "Be the first to experience our latest innovation".to_string(),
            body_html: format!(
                r##"<html>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
    <h1 style="color: #1a1a1a;">Something Big is Here</h1>
    <p style="color: #4a4a4a; line-height: 1.6;">
        Based on your prompt: "{}"
    </p>
    <p style="color: #4a4a4a; line-height: 1.6;">
        We've been working hard to bring you something amazing, and today we're thrilled to share it with you. This is more than just an update - it's a leap forward.
    </p>
    <div style="margin: 30px 0;">
        <a href="#" style="background-color: #0066ff; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; font-weight: 600;">
            Learn More
        </a>
    </div>
</body>
</html>"##,
                prompt
            ),
            body_text: format!(
                "Something Big is Here\n\nBased on your prompt: \"{}\"\n\nWe've been working hard to bring you something amazing.\n\nLearn more now!",
                prompt
            ),
            cta_text: "Learn More".to_string(),
            cta_url: "https://crm.hey.sh/product".to_string(),
        }
    } else if is_newsletter {
        GeneratedEmail {
            subject: "Your Weekly Update".to_string(),
            preview_text: "Here's what you need to know this week".to_string(),
            body_html: format!(
                r##"<html>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
    <h1 style="color: #1a1a1a;">This Week's Highlights</h1>
    <p style="color: #4a4a4a; line-height: 1.6;">
        Based on your prompt: "{}"
    </p>
    <p style="color: #4a4a4a; line-height: 1.6;">
        Here's a quick roundup of everything that happened this week and what's coming up next.
    </p>
    <div style="margin: 30px 0;">
        <a href="#" style="background-color: #0066ff; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; font-weight: 600;">
            Read Full Update
        </a>
    </div>
</body>
</html>"##,
                prompt
            ),
            body_text: format!(
                "This Week's Highlights\n\nBased on your prompt: \"{}\"\n\nHere's a quick roundup of everything that happened this week.",
                prompt
            ),
            cta_text: "Read Full Update".to_string(),
            cta_url: "https://crm.hey.sh/blog".to_string(),
        }
    } else {
        GeneratedEmail {
            subject: "A Quick Note for You".to_string(),
            preview_text: "We have something to share".to_string(),
            body_html: format!(
                r##"<html>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
    <h1 style="color: #1a1a1a;">Hello!</h1>
    <p style="color: #4a4a4a; line-height: 1.6;">
        Based on your prompt: "{}"
    </p>
    <p style="color: #4a4a4a; line-height: 1.6;">
        We wanted to reach out and share something with you. Your engagement means a lot to us, and we're always looking for ways to provide value.
    </p>
    <div style="margin: 30px 0;">
        <a href="#" style="background-color: #0066ff; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; font-weight: 600;">
            Learn More
        </a>
    </div>
</body>
</html>"##,
                prompt
            ),
            body_text: format!(
                "Hello!\n\nBased on your prompt: \"{}\"\n\nWe wanted to reach out and share something with you.",
                prompt
            ),
            cta_text: "Learn More".to_string(),
            cta_url: "https://crm.hey.sh".to_string(),
        }
    }
}
