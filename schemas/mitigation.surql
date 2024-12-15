DEFINE TABLE IF NOT EXISTS mitigation SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS mitre_id ON mitigation TYPE string;
DEFINE FIELD IF NOT EXISTS mitre_name ON mitigation TYPE string;
DEFINE FIELD IF NOT EXISTS mitre_description ON mitigation TYPE string;
DEFINE FIELD IF NOT EXISTS embedding ON mitigation TYPE array<float>;
DEFINE FIELD IF NOT EXISTS mitre_url ON mitigation TYPE string ASSERT string::is::url($value);
DEFINE FIELD IF NOT EXISTS created_at ON user TYPE datetime DEFAULT time::now();

DEFINE INDEX OVERWRITE mitigation_hsnw_index ON mitigation FIELDS embedding HNSW DIMENSION 384 DIST COSINE TYPE F32;
