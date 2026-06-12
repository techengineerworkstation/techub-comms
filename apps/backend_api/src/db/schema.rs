pub const CREATE_USERS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_login TIMESTAMPTZ
);
"#;

pub const CREATE_SESSIONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS auth_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(512) UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
"#;

pub const CREATE_CALLS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS calls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    vonage_call_id VARCHAR(255),
    phone_number VARCHAR(50) NOT NULL,
    call_type VARCHAR(20) NOT NULL DEFAULT 'simple',
    status VARCHAR(50) NOT NULL DEFAULT 'initiated',
    duration_seconds INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);
"#;

pub const CREATE_MESSAGES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    vonage_message_id VARCHAR(255),
    recipient VARCHAR(50) NOT NULL,
    channel VARCHAR(20) NOT NULL DEFAULT 'sms',
    content TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT NOW()
);
"#;

pub const CREATE_RECORDINGS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS recordings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    vonage_archive_id VARCHAR(255),
    room_name VARCHAR(255) NOT NULL,
    session_id VARCHAR(500),
    status VARCHAR(50) NOT NULL DEFAULT 'started',
    duration_seconds INTEGER DEFAULT 0,
    file_url VARCHAR(1000),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);
"#;
