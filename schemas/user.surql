DEFINE TABLE user SCHEMALESS;

DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD url_safe_username ON user TYPE string;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD first_name ON user TYPE string;
DEFINE FIELD last_name ON user TYPE string;
DEFINE FIELD name ON TABLE user VALUE $this.first_name + ' ' + $this.last_name;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD last_login ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD picture ON user TYPE string;
DEFINE FIELD role ON user TYPE string ASSERT $value IN ["User", "Admin", "Owner"] DEFAULT "User";

DEFINE ANALYZER user_analyzer TOKENIZERS blank,class,camel,punct FILTERS lowercase, edgengram(2,10);

-- Create unique index for the email field
DEFINE INDEX unique_user_email_index ON user FIELDS email UNIQUE;

-- Create unique index for the url_safe_username and username fields
DEFINE INDEX unique_user_url_safe_username_index ON user FIELDS url_safe_username UNIQUE;
DEFINE INDEX unique_user_username_index ON user FIELDS username UNIQUE;
DEFINE INDEX user_username_index ON user FIELDS username SEARCH ANALYZER user_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX user_name_index ON user FIELDS name SEARCH ANALYZER user_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX user_email_index ON user FIELDS email SEARCH ANALYZER user_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX user_first_name_index ON user FIELDS first_name SEARCH ANALYZER user_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX user_last_name_index ON user FIELDS last_name SEARCH ANALYZER user_analyzer BM25 HIGHLIGHTS;