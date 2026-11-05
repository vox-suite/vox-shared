// @generated automatically by Diesel CLI.

diesel::table! {
    voice_sessions (id) {
        id -> Uuid,
        channel -> Text,
        status -> Text,
        twilio_account_sid -> Nullable<Text>,
        twilio_call_sid -> Nullable<Text>,
        started_at -> Nullable<Timestamptz>,
        ended_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}
