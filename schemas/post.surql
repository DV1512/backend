DEFINE TABLE post SCHEMALESS;

DEFINE FIELD title ON TABLE post TYPE string;
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD created ON TABLE post TYPE datetime DEFAULT time::now();
DEFINE FIELD updated ON TABLE post TYPE datetime DEFAULT time::now();
DEFINE FIELD user_id ON post TYPE record(user);

DEFINE ANALYZER post_analyzer TOKENIZERS blank,class,camel,punct FILTERS lowercase, edgengram(1,10), snowball(english);

-- Define index for posts
DEFINE INDEX post_title_index ON post FIELDS title SEARCH ANALYZER post_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX post_content_index ON post FIELDS content SEARCH ANALYZER post_analyzer BM25 HIGHLIGHTS;