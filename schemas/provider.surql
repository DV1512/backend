DEFINE TABLE provider SCHEMAFULL;

DEFINE FIELD name ON TABLE provider VALUE meta::id($this.id);
DEFINE FIELD config ON provider TYPE option<record<provider_config>>;