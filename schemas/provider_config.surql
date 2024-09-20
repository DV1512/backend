DEFINE TABLE provider_config SCHEMAFULL;

DEFINE FIELD user_info_url ON provider_config TYPE option<string> ASSERT string::is::url($value);
DEFINE FIELD auth_url ON provider_config TYPE option<string> ASSERT string::is::url($value);
DEFINE FIELD token_url ON provider_config TYPE option<string> ASSERT string::is::url($value);
DEFINE FIELD scopes ON provider_config TYPE option<array<string>>;
DEFINE FIELD additional_config ON provider_config FLEXIBLE TYPE object DEFAULT {};
DEFINE FIELD redirect_endpoint ON provider_config TYPE option<string>;
