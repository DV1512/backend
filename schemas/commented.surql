DEFINE TABLE commented SCHEMAFULL TYPE RELATION FROM user TO comment;

DEFINE FIELD IF NOT EXISTS time_created ON commented TYPE datetime DEFAULT time::now();
DEFINE INDEX IF NOT EXISTS commented_index ON commented FIELDS out, in;