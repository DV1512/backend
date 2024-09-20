DEFINE TABLE IF NOT EXISTS provider SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS name ON TABLE provider VALUE record::id($this.id);
DEFINE FIELD IF NOT EXISTS config ON provider TYPE option<record<provider_config>>;