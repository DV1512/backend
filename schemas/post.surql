DEFINE TABLE IF NOT EXISTS post SCHEMALESS;

DEFINE FIELD IF NOT EXISTS title ON TABLE post TYPE string;
DEFINE FIELD IF NOT EXISTS content ON TABLE post TYPE string;
DEFINE FIELD IF NOT EXISTS created ON TABLE post TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated ON TABLE post TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS user_id ON post TYPE record<user>;

DEFINE ANALYZER IF NOT EXISTS post_analyzer TOKENIZERS blank,class,camel,punct FILTERS lowercase, edgengram(1,10), snowball(english);

-- Define index for posts
DEFINE INDEX IF NOT EXISTS post_title_index ON post FIELDS title SEARCH ANALYZER post_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX IF NOT EXISTS post_content_index ON post FIELDS content SEARCH ANALYZER post_analyzer BM25 HIGHLIGHTS;