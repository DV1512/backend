DEFINE TABLE comment_of SCHEMAFULL TYPE RELATION FROM comment TO post;

DEFINE FIELD time_created ON comment_of TYPE datetime DEFAULT time::now();
DEFINE INDEX comment_of_index ON comment_of FIELDS out, in;