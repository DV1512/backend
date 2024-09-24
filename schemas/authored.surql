DEFINE TABLE IF NOT EXISTS authored SCHEMAFULL TYPE RELATION FROM user TO post;

DEFINE FIELD IF NOT EXISTS time_created ON authored TYPE datetime DEFAULT time::now();
DEFINE INDEX IF NOT EXISTS authored_index ON authored FIELDS out, in;