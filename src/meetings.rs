//! Text-only meeting analysis contract shared by Core and the local desktop store.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioSource {
    Mic,
    System,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranscriptSegment {
    pub id: String,
    pub source: AudioSource,
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisRequest {
    pub schema_version: u32,
    pub meeting_id: String,
    pub transcript_hash: String,
    pub started_at: String,
    pub timezone: String,
    pub my_names: Vec<String>,
    pub segments: Vec<TranscriptSegment>,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceText {
    pub text: String,
    pub evidence_ids: Vec<String>,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionOwner {
    Me,
    Other,
    Unknown,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionCandidate {
    pub title: String,
    pub notes: String,
    pub owner: ActionOwner,
    pub due_at: Option<String>,
    pub evidence_ids: Vec<String>,
    pub evidence_quote: String,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeetingAnalysis {
    pub overview: Vec<EvidenceText>,
    pub decisions: Vec<EvidenceText>,
    pub open_questions: Vec<EvidenceText>,
    pub actions: Vec<ActionCandidate>,
}
#[cfg_attr(feature = "schema", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisResponse {
    pub analysis: MeetingAnalysis,
    pub model: String,
    pub prompt_revision: u32,
}
fn hash(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes.as_ref()))
}
pub fn transcript_hash(segments: &[TranscriptSegment]) -> String {
    hash(serde_json::to_vec(segments).expect("Plain meeting fields serialize"))
}
pub fn request_hash(request: &AnalysisRequest) -> String {
    hash(serde_json::to_vec(request).expect("Plain meeting fields serialize"))
}
fn check(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn text_ok(text: &str, max: usize) -> bool {
    !text.trim().is_empty()
        && text.len() <= max
        && !text
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\t' | '\r'))
}
pub fn validate_request(input: &AnalysisRequest) -> Result<(), String> {
    check(
        input.schema_version == 1,
        "Unsupported meeting analysis version",
    )?;
    check(
        uuid::Uuid::parse_str(&input.meeting_id).is_ok(),
        "Invalid meeting ID",
    )?;
    check(
        chrono::DateTime::parse_from_rfc3339(&input.started_at).is_ok(),
        "Invalid meeting start time",
    )?;
    check(
        text_ok(&input.timezone, 100)
            && input.my_names.len() <= 8
            && input
                .my_names
                .iter()
                .all(|n| text_ok(n, 100) && n.trim().chars().count() >= 2),
        "Invalid meeting name aliases or timezone",
    )?;
    check(
        !input.segments.is_empty() && input.segments.len() <= 5000,
        "No transcript, or too many transcript segments",
    )?;
    let mut ids = HashSet::new();
    let mut bytes = 0;
    for segment in &input.segments {
        check(
            text_ok(&segment.id, 256)
                && ids.insert(segment.id.as_str())
                && text_ok(&segment.text, 20000)
                && segment.end_ms >= segment.start_ms
                && segment.end_ms <= 7_200_000,
            "Invalid or duplicate transcript segment",
        )?;
        bytes += segment.text.len() + segment.id.len();
    }
    check(
        bytes <= 100_000,
        "Transcript exceeds the current summary limit; export it for processing in smaller parts",
    )?;
    check(
        input.transcript_hash == transcript_hash(&input.segments),
        "Transcript fingerprint mismatch",
    )
}
fn references(ids: &[String], segments: &HashMap<&str, &TranscriptSegment>) -> Result<(), String> {
    check(
        !ids.is_empty()
            && ids.len() <= 8
            && ids.iter().all(|id| segments.contains_key(id.as_str()))
            && ids.iter().collect::<HashSet<_>>().len() == ids.len(),
        "Generated text cites missing or duplicate transcript evidence",
    )
}
pub fn action_key(input: &AnalysisRequest, action: &ActionCandidate) -> Result<String, String> {
    let primary = action
        .evidence_ids
        .first()
        .and_then(|id| input.segments.iter().find(|s| &s.id == id))
        .ok_or("Missing task evidence")?;
    let quote = action
        .evidence_quote
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    Ok(hash(
        serde_json::to_vec(&(&input.meeting_id, &primary.source, primary.start_ms, quote))
            .expect("Plain evidence fields serialize"),
    ))
}
fn names_in_quote(names: &[String], quote: &str) -> bool {
    let lower = quote.to_lowercase();
    names.iter().any(|name| {
        let name = name.trim().to_lowercase();
        lower.match_indices(&name).any(|(i, _)| {
            lower[..i]
                .chars()
                .next_back()
                .is_none_or(|c| !c.is_alphanumeric())
                && lower[i + name.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| !c.is_alphanumeric())
        })
    })
}
pub fn validate_analysis(
    input: &AnalysisRequest,
    output: &mut MeetingAnalysis,
) -> Result<(), String> {
    validate_request(input)?;
    check(
        !output.overview.is_empty()
            || !output.actions.is_empty()
            || !output.decisions.is_empty()
            || !output.open_questions.is_empty(),
        "The model returned an empty analysis",
    )?;
    check(
        output.overview.len() <= 8
            && output.decisions.len() <= 20
            && output.open_questions.len() <= 20
            && output.actions.len() <= 30,
        "Generated analysis is too large",
    )?;
    let segments: HashMap<_, _> = input.segments.iter().map(|s| (s.id.as_str(), s)).collect();
    for item in output
        .overview
        .iter()
        .chain(&output.decisions)
        .chain(&output.open_questions)
    {
        check(text_ok(&item.text, 4000), "Invalid generated summary text")?;
        references(&item.evidence_ids, &segments)?;
    }
    let mut keys = HashSet::new();
    for action in &mut output.actions {
        check(
            text_ok(&action.title, 800)
                && action.title.chars().count() <= 200
                && action.notes.len() <= 4000
                && text_ok(&action.evidence_quote, 2000),
            "Invalid generated task text",
        )?;
        check(
            !action
                .notes
                .chars()
                .any(|c| c.is_control() && !matches!(c, '\n' | '\t' | '\r')),
            "Invalid task note characters",
        )?;
        references(&action.evidence_ids, &segments)?;
        check(
            segments[action.evidence_ids[0].as_str()]
                .text
                .contains(action.evidence_quote.trim()),
            "Task evidence quote is not present in the cited transcript",
        )?;
        if let Some(date) = &action.due_at {
            check(
                chrono::DateTime::parse_from_rfc3339(date).is_ok(),
                "Generated deadline needs an explicit timezone",
            )?;
        }
        if action.owner == ActionOwner::Me
            && !names_in_quote(&input.my_names, &action.evidence_quote)
        {
            action.owner = ActionOwner::Unknown;
        }
        check(
            keys.insert(action_key(input, action)?),
            "Duplicate actions cite the same evidence; retry analysis",
        )?;
    }
    check(
        serde_json::to_vec(output).map_err(|e| e.to_string())?.len() <= 128_000,
        "Generated analysis is too large",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> AnalysisRequest {
        serde_json::from_value(serde_json::json!({"schema_version":1,"meeting_id":"00000000-0000-4000-8000-000000000001","transcript_hash":"","started_at":"2026-10-28T12:00:00Z","timezone":"Asia/Kolkata","my_names":["Rahul"],"segments":[{"id":"mic:1","source":"mic","start_ms":0,"end_ms":5000,"text":"Rahul will send the proposal tomorrow."}]})).unwrap()
    }
    #[test]
    fn citations_and_verbatim_action_evidence_are_required() {
        let mut request = request();
        request.transcript_hash = transcript_hash(&request.segments);
        validate_request(&request).unwrap();
        let mut analysis:MeetingAnalysis=serde_json::from_value(serde_json::json!({"overview":[{"text":"Proposal follow-up agreed.","evidence_ids":["mic:1"]}],"decisions":[],"open_questions":[],"actions":[{"title":"Send the proposal","notes":"","owner":"me","due_at":null,"evidence_ids":["mic:1"],"evidence_quote":"Rahul will send the proposal tomorrow."}]})).unwrap();
        validate_analysis(&request, &mut analysis).unwrap();
        let key = action_key(&request, &analysis.actions[0]).unwrap();
        assert_eq!(key.len(), 64);
        analysis.actions[0].evidence_quote = "Invented words".into();
        assert!(validate_analysis(&request, &mut analysis).is_err());
    }
    #[test]
    fn microphone_is_not_identity_and_unknown_citations_fail() {
        let mut request = request();
        request.my_names.clear();
        request.transcript_hash = transcript_hash(&request.segments);
        let mut analysis:MeetingAnalysis=serde_json::from_value(serde_json::json!({"overview":[],"decisions":[],"open_questions":[],"actions":[{"title":"Send it","notes":"","owner":"me","due_at":null,"evidence_ids":["mic:1"],"evidence_quote":"Rahul will send the proposal tomorrow."}]})).unwrap();
        validate_analysis(&request, &mut analysis).unwrap();
        assert_eq!(analysis.actions[0].owner, ActionOwner::Unknown);
        analysis.actions[0].evidence_ids = vec!["missing".into()];
        assert!(validate_analysis(&request, &mut analysis).is_err());
    }
}

#[cfg(test)]
mod limits_tests {
    use super::*;
    #[test]
    fn parser_rejects_audio_fields_and_deadlines_without_timezone() {
        let request = serde_json::from_value::<AnalysisRequest>(
            serde_json::json!({"schema_version":1,"meeting_id":"id","transcript_hash":"hash","started_at":"date","timezone":"UTC","my_names":[],"segments":[],"audio":"not allowed"}),
        );
        assert!(request.is_err());
        assert!(chrono::DateTime::parse_from_rfc3339("2026-10-30T10:00:00").is_err());
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    #[test]
    fn repeated_words_at_different_times_do_not_share_a_review_receipt() {
        let request:AnalysisRequest=serde_json::from_value(serde_json::json!({"schema_version":1,"meeting_id":"00000000-0000-4000-8000-000000000001","transcript_hash":"","started_at":"2026-10-28T12:00:00Z","timezone":"UTC","my_names":[],"segments":[{"id":"a","source":"mic","start_ms":0,"end_ms":1000,"text":"Send it."},{"id":"b","source":"mic","start_ms":2000,"end_ms":3000,"text":"Send it."}]})).unwrap();
        let mut action:ActionCandidate=serde_json::from_value(serde_json::json!({"title":"Send it","notes":"","owner":"unknown","due_at":null,"evidence_ids":["a"],"evidence_quote":"Send it."})).unwrap();
        let first = action_key(&request, &action).unwrap();
        action.evidence_ids = vec!["b".into()];
        assert_ne!(first, action_key(&request, &action).unwrap());
    }
}
