DEFINE TABLE IF NOT EXISTS provider_config SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS user_info_url ON provider_config TYPE option<string> ASSERT string::is::url($value);
DEFINE FIELD IF NOT EXISTS auth_url ON provider_config TYPE option<string> ASSERT string::is::url($value);
DEFINE FIELD IF NOT EXISTS token_url ON provider_config TYPE option<string> ASSERT string::is::url($value);
DEFINE FIELD IF NOT EXISTS scopes ON provider_config TYPE option<array<string>>;
DEFINE FIELD IF NOT EXISTS additional_config ON provider_config FLEXIBLE TYPE object DEFAULT {};
DEFINE FIELD IF NOT EXISTS redirect_endpoint ON provider_config TYPE option<string>;
