DEFINE TABLE session_of SCHEMAFULL TYPE RELATION FROM user TO session;

DEFINE FIELD time_created ON session_of TYPE datetime DEFAULT time::now();
DEFINE INDEX session_of_index ON session_of FIELDS out, in;