DEFINE TABLE authored SCHEMAFULL TYPE RELATION FROM user TO post;

DEFINE FIELD time_created ON authored TYPE datetime DEFAULT time::now();
DEFINE INDEX authored_index ON authored FIELDS out, in;