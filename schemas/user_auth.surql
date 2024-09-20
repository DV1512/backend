DEFINE TABLE user_auth SCHEMAFULL;

DEFINE FIELD providers ON user_auth TYPE array<record<provider>> ASSERT array::len($value) > 0;
DEFINE FIELD password ON user_auth TYPE option<string>;
DEFINE FIELD created_at ON user_auth TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user_auth TYPE datetime DEFAULT time::now();