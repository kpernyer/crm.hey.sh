use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedLandingPage {
    pub title: String,
    pub subtitle: String,
    pub hero_section: HeroSection,
    pub features: Vec<FeatureSection>,
    pub cta_section: CtaSection,
    pub testimonials: Vec<Testimonial>,
    pub faq: Vec<FaqItem>,
    pub footer: FooterSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSection {
    pub headline: String,
    pub subheadline: String,
    pub cta_text: String,
    pub cta_url: String,
    pub image_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSection {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtaSection {
    pub headline: String,
    pub description: String,
    pub button_text: String,
    pub button_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonial {
    pub quote: String,
    pub author: String,
    pub role: String,
    pub company: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterSection {
    pub company_name: String,
    pub tagline: String,
    pub links: Vec<FooterLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterLink {
    pub text: String,
    pub url: String,
}

/// Generate a landing page from a prompt
/// This is a stub that returns template-based mock data
/// In production, this would call an AI service
pub async fn generate_landing_page(prompt: &str) -> GeneratedLandingPage {
    let is_product = prompt.to_lowercase().contains("product");
    let is_event = prompt.to_lowercase().contains("event");
    let is_waitlist = prompt.to_lowercase().contains("waitlist") || prompt.to_lowercase().contains("early access");

    let headline = if is_event {
        "Join Us for an Exclusive Event".to_string()
    } else if is_waitlist {
        "Be First in Line".to_string()
    } else if is_product {
        "The CRM Built for Founders".to_string()
    } else {
        "Transform How You Connect".to_string()
    };

    GeneratedLandingPage {
        title: format!("{} | hey.sh", headline),
        subtitle: format!("Generated from: {}", prompt),
        hero_section: HeroSection {
            headline,
            subheadline: "A modern CRM designed specifically for startup founders. Manage relationships, run campaigns, and grow your business - all in one place.".to_string(),
            cta_text: if is_waitlist { "Join the Waitlist".to_string() } else { "Get Started Free".to_string() },
            cta_url: "/signup".to_string(),
            image_prompt: "Modern SaaS dashboard with clean UI, showing CRM features, light mode".to_string(),
        },
        features: vec![
            FeatureSection {
                title: "Contact Management".to_string(),
                description: "Keep track of every relationship with smart contact profiles, company associations, and engagement scoring.".to_string(),
                icon: "users".to_string(),
            },
            FeatureSection {
                title: "Campaign Builder".to_string(),
                description: "Create multi-channel campaigns with AI-generated content. Email, social, landing pages - all from one prompt.".to_string(),
                icon: "rocket".to_string(),
            },
            FeatureSection {
                title: "Event Management".to_string(),
                description: "Host webinars, meetups, and demos. Track RSVPs and attendance automatically.".to_string(),
                icon: "calendar".to_string(),
            },
            FeatureSection {
                title: "Real-time Analytics".to_string(),
                description: "Understand your funnel with detailed analytics. Track engagement, conversions, and ROI.".to_string(),
                icon: "chart".to_string(),
            },
        ],
        cta_section: CtaSection {
            headline: "Ready to Transform Your Outreach?".to_string(),
            description: "Join thousands of founders who are building better relationships with hey.sh CRM.".to_string(),
            button_text: "Start Free Trial".to_string(),
            button_url: "/signup".to_string(),
        },
        testimonials: vec![
            Testimonial {
                quote: "Finally, a CRM that understands what founders actually need. Simple, powerful, and just works.".to_string(),
                author: "Sarah Chen".to_string(),
                role: "CEO".to_string(),
                company: "TechStartup Inc".to_string(),
            },
            Testimonial {
                quote: "The AI campaign builder saved us hours of work. Our email open rates increased by 40%.".to_string(),
                author: "Mike Johnson".to_string(),
                role: "Head of Growth".to_string(),
                company: "ScaleUp Labs".to_string(),
            },
        ],
        faq: vec![
            FaqItem {
                question: "How is this different from other CRMs?".to_string(),
                answer: "hey.sh CRM is built specifically for founders and small teams. We focus on simplicity and AI-powered automation instead of bloated features you'll never use.".to_string(),
            },
            FaqItem {
                question: "What's included in the free trial?".to_string(),
                answer: "Full access to all features for 14 days. No credit card required.".to_string(),
            },
            FaqItem {
                question: "Can I import my existing contacts?".to_string(),
                answer: "Yes! We support CSV import and direct integrations with popular tools.".to_string(),
            },
        ],
        footer: FooterSection {
            company_name: "hey.sh".to_string(),
            tagline: "The founder-focused CRM".to_string(),
            links: vec![
                FooterLink { text: "About".to_string(), url: "/about".to_string() },
                FooterLink { text: "Blog".to_string(), url: "/blog".to_string() },
                FooterLink { text: "Privacy".to_string(), url: "/privacy".to_string() },
                FooterLink { text: "Terms".to_string(), url: "/terms".to_string() },
            ],
        },
    }
}
