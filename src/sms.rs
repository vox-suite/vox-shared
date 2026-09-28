/**
* Shared SMS-classification taxonomy and result shape, used by both vox-core's
* cloud (Gemini) extractor and vox-desktop's on-device (local Gemma) extractor
* so the category list and prompt schema can't silently drift between them.
*/
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

/// The category prompt clause, ready to splice into either extractor's
/// classification prompt: `"payment"|"delivery"|...`.
pub const SMS_CATEGORIES_PROMPT: &str =
    "\"payment\"|\"delivery\"|\"appointment\"|\"travel\"|\"otp\"|\"other\"";

/// Categories every user starts with. Extractors may invent more; the ones a
/// user already has are fed back into the prompt so the set grows with them.
pub const SMS_SEED_CATEGORIES: &[&str] = &[
    "payment",
    "income",
    "bill_due",
    "emi_due",
    "credit_card_bill",
    "subscription_renewal",
    "insurance_premium",
    "refund",
    "delivery",
    "appointment",
    "travel",
    "other",
];

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExtractedSmsEvent {
    pub relevant: bool,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, deserialize_with = "lenient_f64")]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    /// `debit`, `credit`, `due` or `info`.
    #[serde(default)]
    pub direction: Option<String>,
    /// `pending`, `upcoming`, `overdue`, `paid` or `info`.
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub merchant: Option<String>,
    /// Last digits of the card or account the message refers to.
    #[serde(default)]
    pub account_hint: Option<String>,
    /// Transaction, bill or order reference printed in the message.
    #[serde(default)]
    pub reference: Option<String>,
    /// When something is due or scheduled: `YYYY-MM-DD` or ISO 8601.
    #[serde(default)]
    pub due_at: Option<String>,
    /// When the described event happened, if not when the SMS arrived.
    #[serde(default)]
    pub event_at: Option<String>,
    /// Anything else worth keeping, as free-form key/value pairs.
    #[serde(default)]
    pub attributes: Option<Map<String, Value>>,
}

/// Lowercase snake_case, at most 40 characters; empty input becomes `other`.
pub fn normalize_category(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') && !out.is_empty() {
            out.push('_');
        }
    }
    let out = out.trim_matches('_');
    if out.is_empty() {
        "other".to_string()
    } else {
        out.chars().take(40).collect()
    }
}

fn lenient_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    Ok(match Option::<Value>::deserialize(deserializer)? {
        Some(Value::Number(n)) => n.as_f64(),
        Some(Value::String(s)) => {
            let cleaned: String = s
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                .collect();
            cleaned.parse().ok()
        }
        _ => None,
    })
}
