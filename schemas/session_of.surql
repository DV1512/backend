DEFINE TABLE IF NOT EXISTS session_of SCHEMAFULL TYPE RELATION FROM user TO session;

DEFINE FIELD IF NOT EXISTS time_created ON session_of TYPE datetime DEFAULT time::now();
DEFINE INDEX IF NOT EXISTS session_of_index ON session_of FIELDS out, in;