DEFINE TABLE commented SCHEMAFULL TYPE RELATION FROM user TO comment;

DEFINE FIELD time_created ON commented TYPE datetime DEFAULT time::now();
DEFINE INDEX commented_index ON commented FIELDS out, in;