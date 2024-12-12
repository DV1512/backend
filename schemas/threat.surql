DEFINE TABLE IF NOT EXISTS threat SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS mitre_id ON threat TYPE string;
DEFINE FIELD IF NOT EXISTS mitre_name ON threat TYPE string;
DEFINE FIELD IF NOT EXISTS mitre_description ON threat TYPE string;
DEFINE FIELD IF NOT EXISTS embedding ON threat TYPE array<float>;
DEFINE FIELD IF NOT EXISTS mitre_url ON threat TYPE string ASSERT string::is::url($value);
DEFINE FIELD IF NOT EXISTS created_at ON user TYPE datetime DEFAULT time::now();

DEFINE INDEX OVERWRITE threat_hsnw_index ON threat FIELDS embedding HNSW DIMENSION 384 DIST COSINE TYPE F32;
