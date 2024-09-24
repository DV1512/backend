DEFINE TABLE IF NOT EXISTS likes TYPE RELATION IN user OUT comment | post SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD IF NOT EXISTS in ON likes TYPE record<user> ASSERT !fn::likes_item($value, $this.out);
DEFINE FIELD IF NOT EXISTS out ON likes TYPE record<comment | post> ASSERT !fn::likes_item($this.in, $value);
