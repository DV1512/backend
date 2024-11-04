DEFINE TABLE IF NOT EXISTS session SCHEMALESS;

-- Define fields for the session table
DEFINE FIELD IF NOT EXISTS email ON session TYPE string ASSERT string::is::email($value);
DEFINE FIELD IF NOT EXISTS access_token ON session TYPE string;
DEFINE FIELD IF NOT EXISTS refresh_token ON session TYPE option<string>;
DEFINE FIELD IF NOT EXISTS created_at ON session TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS expires_at ON session TYPE datetime DEFAULT time::now() + 1h;

DEFINE INDEX IF NOT EXISTS unique_session_refresh_token_index ON session FIELDS refresh_token UNIQUE;
DEFINE INDEX IF NOT EXISTS unique_session_access_token_index ON session FIELDS access_token UNIQUE;

-- Create a foreign key relation between session and user using user_id
DEFINE FIELD IF NOT EXISTS user_id ON session TYPE record<user>;