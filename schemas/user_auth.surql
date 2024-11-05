DEFINE TABLE IF NOT EXISTS user_auth SCHEMAFULL;

DEFINE FUNCTION OVERWRITE fn::hash_string($password: option<string>) -> option<string> {
	IF $password IS NOT None {
	    RETURN crypto::argon2::generate($password)
	} ELSE {
	    RETURN None
	}
} PERMISSIONS FULL;

DEFINE FIELD IF NOT EXISTS providers ON user_auth TYPE array<record<provider>> ASSERT array::len($value) > 0;
DEFINE FIELD IF NOT EXISTS password ON user_auth TYPE option<string> VALUE fn::hash_string($value);
DEFINE FIELD IF NOT EXISTS created_at ON user_auth TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD IF NOT EXISTS updated_at ON user_auth TYPE datetime DEFAULT time::now();