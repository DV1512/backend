DEFINE TABLE session SCHEMALESS;

-- Define fields for the session table
DEFINE FIELD email ON session TYPE string ASSERT string::is::email($value);
DEFINE FIELD access_token ON session TYPE string;
DEFINE FIELD refresh_token ON session TYPE option<string>;
DEFINE FIELD created_at ON session TYPE datetime DEFAULT time::now();
DEFINE FIELD expires_at ON session TYPE datetime DEFAULT time::now() + 1h;

DEFINE INDEX unique_session_email_index ON session FIELDS email UNIQUE;
DEFINE INDEX unique_session_user_id_index ON session FIELDS user_id UNIQUE;
DEFINE INDEX unique_session_access_token_index ON session FIELDS access_token UNIQUE;

-- Create a foreign key relation between session and user using user_id
DEFINE FIELD  user_id ON session TYPE record(user);