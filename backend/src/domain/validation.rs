//! Validation - Pure functions for validating data
//!
//! IMPORTANT: These functions have NO side effects.
//! They don't check the database. They don't call APIs.
//! They just validate the FORMAT and STRUCTURE of data.
//!
//! "Is this email syntactically valid?" → Yes, validation
//! "Is this email already in use?" → No, that's a repository concern

use super::errors::{DomainError, DomainResult};
use once_cell::sync::Lazy;
use regex::Regex;

// Compile regex patterns once, reuse forever
static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Accepts: +1234567890, (123) 456-7890, 123-456-7890, etc.
    Regex::new(r"^[\d\s\-\(\)\+\.]{7,20}$").unwrap()
});

static LINKEDIN_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^https?://(www\.)?linkedin\.com/in/[\w\-]+/?$").unwrap());

/// Validate an email address format
///
/// # Rules:
/// - Must contain exactly one @
/// - Must have characters before @
/// - Must have a valid domain after @
/// - Domain must have at least one dot
/// - TLD must be at least 2 characters
///
/// # Examples
/// ```
/// use crm_backend::domain::validation::validate_email;
///
/// assert!(validate_email("user@example.com").is_ok());
/// assert!(validate_email("invalid").is_err());
/// assert!(validate_email("").is_err());
/// ```
pub fn validate_email(email: &str) -> DomainResult<()> {
    if email.is_empty() {
        return Err(DomainError::RequiredFieldMissing {
            field: "email".to_string(),
        });
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err(DomainError::InvalidField {
            field: "email".to_string(),
            reason: "Invalid email format".to_string(),
        });
    }

    // Additional check: no consecutive dots
    if email.contains("..") {
        return Err(DomainError::InvalidField {
            field: "email".to_string(),
            reason: "Email cannot contain consecutive dots".to_string(),
        });
    }

    Ok(())
}

/// Validate a phone number format
///
/// # Rules:
/// - Optional (None is valid)
/// - If provided, must be 7-20 characters
/// - Can contain digits, spaces, dashes, parentheses, plus, dots
pub fn validate_phone(phone: Option<&str>) -> DomainResult<()> {
    match phone {
        None => Ok(()),
        Some(p) if p.is_empty() => Ok(()), // Treat empty as None
        Some(p) => {
            if !PHONE_REGEX.is_match(p) {
                return Err(DomainError::InvalidField {
                    field: "phone".to_string(),
                    reason: "Invalid phone format".to_string(),
                });
            }
            Ok(())
        }
    }
}

/// Validate a LinkedIn URL
///
/// # Rules:
/// - Optional (None is valid)
/// - If provided, must be a valid LinkedIn profile URL
pub fn validate_linkedin_url(url: Option<&str>) -> DomainResult<()> {
    match url {
        None => Ok(()),
        Some(u) if u.is_empty() => Ok(()),
        Some(u) => {
            if !LINKEDIN_REGEX.is_match(u) {
                return Err(DomainError::InvalidField {
                    field: "linkedin_url".to_string(),
                    reason:
                        "Must be a valid LinkedIn profile URL (https://linkedin.com/in/username)"
                            .to_string(),
                });
            }
            Ok(())
        }
    }
}

/// Validate a name (first or last)
///
/// # Rules:
/// - Required (cannot be empty)
/// - Must be 1-100 characters
/// - Can contain letters, spaces, hyphens, apostrophes
pub fn validate_name(name: &str, field_name: &str) -> DomainResult<()> {
    if name.trim().is_empty() {
        return Err(DomainError::RequiredFieldMissing {
            field: field_name.to_string(),
        });
    }

    if name.len() > 100 {
        return Err(DomainError::InvalidField {
            field: field_name.to_string(),
            reason: "Name cannot exceed 100 characters".to_string(),
        });
    }

    // Check for invalid characters (basic check)
    if name.chars().any(|c| c.is_control()) {
        return Err(DomainError::InvalidField {
            field: field_name.to_string(),
            reason: "Name cannot contain control characters".to_string(),
        });
    }

    Ok(())
}

/// Validate a tag
///
/// # Rules:
/// - Must be 1-50 characters
/// - Can only contain alphanumeric, hyphens, underscores
/// - Will be lowercased for consistency
pub fn validate_tag(tag: &str) -> DomainResult<String> {
    let normalized = tag.trim().to_lowercase();

    if normalized.is_empty() {
        return Err(DomainError::InvalidField {
            field: "tag".to_string(),
            reason: "Tag cannot be empty".to_string(),
        });
    }

    if normalized.len() > 50 {
        return Err(DomainError::InvalidField {
            field: "tag".to_string(),
            reason: "Tag cannot exceed 50 characters".to_string(),
        });
    }

    // Only allow alphanumeric, hyphens, underscores
    if !normalized
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return Err(DomainError::InvalidField {
            field: "tag".to_string(),
            reason: "Tag can only contain letters, numbers, hyphens, and underscores".to_string(),
        });
    }

    Ok(normalized)
}

/// Validate a list of tags
pub fn validate_tags(tags: &[String]) -> DomainResult<Vec<String>> {
    let mut validated = Vec::with_capacity(tags.len());

    for tag in tags {
        validated.push(validate_tag(tag)?);
    }

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    validated.retain(|t| seen.insert(t.clone()));

    Ok(validated)
}

/// Validate engagement score
///
/// # Rules:
/// - Must be between 0.0 and 100.0 (inclusive)
/// - NaN and Infinity are not allowed
///
pub fn validate_engagement_score(score: f64) -> DomainResult<()> {
    // cargo test test_engagement_score_validation

    if score.is_nan() || score.is_infinite() || score < 0.0 || score > 100.0 {
        return Err(DomainError::InvalidField {
            field: "engagement_score".to_string(),
            reason: "Engagement score must be between 0.0 and 100.0 (inclusive)".to_string(),
        });
    }

    Ok(())
}

/// Validate a company domain
///
/// # Rules:
/// - Optional (None is valid)
/// - If provided, must be a valid domain format (e.g., "example.com")
/// - No protocol prefix (http://, https://)
/// - Must have at least one dot
/// - TLD must be 2-10 characters
///
pub fn validate_company_domain(domain: Option<&str>) -> DomainResult<()> {
    if let Some(d) = domain {
        if let Some(value) = validate_company_domain_protocol(d) {
            return value;
        }

        let no_dot = !d.contains('.');
        let tld_too_short = d.split('.').last().unwrap().len() < 2;
        let tld_too_long = d.split('.').last().unwrap().len() > 10;

        if no_dot || tld_too_short || tld_too_long {
            return Err(DomainError::InvalidField {
                field: "company_domain".to_string(),
                reason: "Company domain must have at least one dot and a valid TLD".to_string(),
            });
        }
    }

    Ok(())
}

fn validate_company_domain_protocol(d: &str) -> Option<Result<(), DomainError>> {
    if d.starts_with("http://") || d.starts_with("https://") {
        return Some(Err(DomainError::InvalidField {
            field: "company_domain".to_string(),
            reason: "Company domain cannot have protocol prefix".to_string(),
        }));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Email Validation Tests ----

    #[test]
    fn test_valid_emails() {
        let valid_emails = [
            "user@example.com",
            "user.name@example.com",
            "user+tag@example.com",
            "user@subdomain.example.com",
            "user@example.co.uk",
        ];

        for email in valid_emails {
            assert!(
                validate_email(email).is_ok(),
                "Expected '{}' to be valid",
                email
            );
        }
    }

    #[test]
    fn test_invalid_emails() {
        let invalid_emails = [
            ("", "empty"),
            ("invalid", "no @"),
            ("@example.com", "no local part"),
            ("user@", "no domain"),
            ("user@.com", "no domain name"),
            ("user@example", "no TLD"),
            ("user..name@example.com", "consecutive dots"),
        ];

        for (email, reason) in invalid_emails {
            assert!(
                validate_email(email).is_err(),
                "Expected '{}' to be invalid ({})",
                email,
                reason
            );
        }
    }

    // ---- Phone Validation Tests ----

    #[test]
    fn test_valid_phones() {
        let valid_phones = [
            "+1 234 567 8900",
            "(123) 456-7890",
            "123-456-7890",
            "+44 20 7946 0958",
            "1234567890",
        ];

        for phone in valid_phones {
            assert!(
                validate_phone(Some(phone)).is_ok(),
                "Expected '{}' to be valid",
                phone
            );
        }
    }

    #[test]
    fn test_phone_optional() {
        assert!(validate_phone(None).is_ok());
        assert!(validate_phone(Some("")).is_ok());
    }

    // ---- Name Validation Tests ----

    #[test]
    fn test_valid_names() {
        let valid_names = ["John", "Mary Jane", "O'Connor", "García", "Jean-Pierre"];

        for name in valid_names {
            assert!(
                validate_name(name, "first_name").is_ok(),
                "Expected '{}' to be valid",
                name
            );
        }
    }

    #[test]
    fn test_invalid_names() {
        assert!(validate_name("", "first_name").is_err());
        assert!(validate_name("   ", "first_name").is_err());
        assert!(validate_name(&"a".repeat(101), "first_name").is_err());
    }

    // ---- Tag Validation Tests ----

    #[test]
    fn test_tag_normalization() {
        assert_eq!(validate_tag("  VIP  ").unwrap(), "vip");
        assert_eq!(validate_tag("Early-Adopter").unwrap(), "early-adopter");
        assert_eq!(validate_tag("priority_1").unwrap(), "priority_1");
    }

    #[test]
    fn test_invalid_tags() {
        assert!(validate_tag("").is_err());
        assert!(validate_tag("tag with spaces").is_err());
        assert!(validate_tag("tag@special").is_err());
    }

    #[test]
    fn test_tags_deduplication() {
        let tags = vec![
            "vip".to_string(),
            "VIP".to_string(),
            "early-adopter".to_string(),
        ];
        let result = validate_tags(&tags).unwrap();
        assert_eq!(result, vec!["vip", "early-adopter"]);
    }

    // ---- LinkedIn URL Tests ----

    #[test]
    fn test_valid_linkedin_urls() {
        let valid_urls = [
            "https://linkedin.com/in/johndoe",
            "https://www.linkedin.com/in/johndoe",
            "http://linkedin.com/in/john-doe-123",
            "https://linkedin.com/in/johndoe/",
        ];

        for url in valid_urls {
            assert!(
                validate_linkedin_url(Some(url)).is_ok(),
                "Expected '{}' to be valid",
                url
            );
        }
    }

    #[test]
    fn test_invalid_linkedin_urls() {
        let invalid_urls = [
            "https://linkedin.com/company/acme", // company, not person
            "https://twitter.com/in/johndoe",    // wrong domain
            "linkedin.com/in/johndoe",           // no protocol
        ];

        for url in invalid_urls {
            assert!(
                validate_linkedin_url(Some(url)).is_err(),
                "Expected '{}' to be invalid",
                url
            );
        }
    }

    #[test]
    fn test_engagement_score_validation() {
        // Valid scores
        assert!(validate_engagement_score(0.0).is_ok());
        assert!(validate_engagement_score(50.0).is_ok());
        assert!(validate_engagement_score(100.0).is_ok());

        // Invalid scores
        assert!(validate_engagement_score(-1.0).is_err());
        assert!(validate_engagement_score(100.1).is_err());
        assert!(validate_engagement_score(f64::NAN).is_err());
        assert!(validate_engagement_score(f64::INFINITY).is_err());
    }

    #[test]
    fn test_company_domain_validation() {
        // Valid domains
        assert!(validate_company_domain(Some("example.com")).is_ok());
        assert!(validate_company_domain(Some("sub.example.co.uk")).is_ok());
        assert!(validate_company_domain(None).is_ok());

        // Invalid domains
        assert!(validate_company_domain(Some("https://example.com")).is_err());
        assert!(validate_company_domain(Some("example")).is_err());
        assert!(validate_company_domain(Some("example.")).is_err());
    }
}
