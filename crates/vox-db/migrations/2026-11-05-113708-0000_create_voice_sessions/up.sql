CREATE TABLE voice_sessions (
    id uuid PRIMARY KEY,
    channel text NOT NULL CHECK (channel IN ('phone', 'desktop', 'android')),
    status text NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'active', 'ended', 'failed')),
    twilio_account_sid text,
    twilio_call_sid text,
    started_at timestamptz,
    ended_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),

    UNIQUE (twilio_account_sid, twilio_call_sid),
    CHECK (
        (twilio_account_sid IS NULL) = (twilio_call_sid IS NULL)
    )
);
