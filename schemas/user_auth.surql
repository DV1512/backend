DEFINE TABLE IF NOT EXISTS user_auth SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS providers ON user_auth TYPE array<record<provider>> ASSERT array::len($value) > 0;
DEFINE FIELD IF NOT EXISTS password ON user_auth TYPE option<string>;
DEFINE FIELD IF NOT EXISTS created_at ON user_auth TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON user_auth TYPE datetime DEFAULT time::now();