# Attached meeting analysis and reviewed task promotion

Continue the approved summary/task plan. First concrete summary option is explicit cloud TEXT processing through Core's existing model integration, clearly labeled before clicking. No audio endpoint, automatic upload, silent cloud fallback, biometric attribution, or automatic task creation. A common validated result contract also permits a future local summarizer.

Shared protocol: bounded source-labelled transcript segments and user-supplied name aliases; cited overview/decisions/questions plus task candidates with exact evidence quotes, ownership uncertainty and optional explicit deadline. Never treat microphone as identity. Reject unknown citations, non-verbatim task quotes, invalid deadlines, oversized input/output, and duplicate evidence-action keys. Model has no tools. The UI always requires user review before canonical task creation.

Local persistence: transcript fingerprint guards stale results independently of written notes; preserve previous summary on failure, cache unchanged requests, retain per-action saved/dismissed decisions across unchanged evidence. Store an approved task's exact outgoing request before sending it; stable request IDs and persisted bodies support retry after ambiguous responses. Only a confirmed result marks a task saved. A deleted server task stays dismissed (410). No automatic background outbox sync in this slice; pending approvals have explicit retry.

Delivery: shared DTO/validation tests and dependency; Core authenticated stateless text endpoint, usage accounting, timeout and no-content telemetry; desktop SQLite v3 attachments/review receipts, native authenticated commands; summary/review UI with evidence and explicit cloud consent; fixtures/tests/fresh review, publish all changed repositories in dependency order. Local LLM execution, hierarchical long-transcript processing and packaged-device/provider-live verification remain separate. Reject large transcripts visibly rather than truncate them.

Ruling: summary sections are generated/read-only; existing editable Your notes remain separate and are never overwritten or sent as part of this request. Task creation stores generated task text plus opaque source links only after review. Deleting a meeting removes local analysis/outbox data but keeps already-added Timeline tasks, as stated in the confirmation.

## Completion evidence

Shared validation and schema contract published as e09c160, followed by title validation regression fix 9b608ce. Core endpoint, usage scope, per-account budget, timeout and OpenAPI added; native owner-bound commands and SQLite v3 attached analysis/review receipts implemented. UI includes explicit cloud-text disclosure, cited sections, uncertain ownership, reviewed promotion and manual retry.

Fresh whole-slice review found three P2s. Fixed all: orphan pending approvals remain independently visible after regeneration (browser RED -> GREEN); approved null deadlines stay null despite a newer suggestion (browser RED -> GREEN); title control characters rejected before attachment (protocol RED -> GREEN). Store suite 22/22 and controller suite 17/17 pass. Five meeting browser suites pass; production frontend build and native macOS check pass; strict meeting-store Clippy passes. Existing unrelated desktop Clippy warnings remain outside scope. Core final pre-push suite is the publish gate.

Ruling: generated sections stay read-only while Your notes stays editable and excluded from model input. Cost: users edit their own notes rather than changing generated sections in this slice. Local LLM summary execution and hierarchical long-transcript processing remain future work; live provider and Windows device verification are not claimed.
