DEFINE TABLE IF NOT EXISTS comment SCHEMALESS;

DEFINE FIELD IF NOT EXISTS content ON TABLE comment TYPE string;
DEFINE FIELD IF NOT EXISTS created ON TABLE comment TYPE datetime DEFAULT time::now();

-- Create an index for the content field in comment for faster lookup
DEFINE INDEX IF NOT EXISTS comment_content_index ON comment FIELDS content SEARCH ANALYZER post_analyzer BM25 HIGHLIGHTS;

-- Create a foreign key relation between comment and post using post_id
DEFINE FIELD IF NOT EXISTS post_id ON comment TYPE record<post>;
DEFINE FIELD IF NOT EXISTS user_id ON comment TYPE record<user>;

DEFINE INDEX IF NOT EXISTS comment_post_id ON comment FIELDS post_id;
DEFINE INDEX IF NOT EXISTS comment_user_id ON comment FIELDS user_id;