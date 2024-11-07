DEFINE FUNCTION OVERWRITE fn::create_user($email: string, $username: string, $url_safe_username: string, $first_name: string, $last_name: string, $password: option<string>, $providers: array<record<provider>>) -> any {
	LET $USER = (
	    CREATE user CONTENT {
	        email: $email,
	        username: $username,
	        url_safe_username: $url_safe_username,
	        first_name: $first_name,
	        last_name: $last_name,
	    }
	);

	LET $USER_AUTH = (
	    CREATE user_auth SET
	    providers = $providers,
	    password = $password
	);

	RELATE ($USER_AUTH) -> auth_for -> ($USER);

	RETURN $USER

} PERMISSIONS FULL COMMENT "Registers a user";

DEFINE ACCESS OVERWRITE user ON DATABASE TYPE RECORD SIGNUP (RETURN fn::create_user($email, $username, $url_safe_username, $first_name, $last_name, $password, [
	provider:Email
])) SIGNIN (return (select out.* as user from auth_for where out.email == $email and crypto::argon2::compare(in.password, $password)).user) DURATION FOR TOKEN 5m, FOR SESSION 1h;
