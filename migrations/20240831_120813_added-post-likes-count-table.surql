DEFINE TABLE post_likes_count SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD post_id ON post_likes_count TYPE record<post> PERMISSIONS FULL;
DEFINE FIELD likes_count ON post_likes_count TYPE int DEFAULT 0 PERMISSIONS FULL;
DEFINE INDEX post_likes_count_index ON post_likes_count FIELDS post_id UNIQUE;

DEFINE TABLE comment_likes_count SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD comment_id ON comment_likes_count TYPE record<comment> PERMISSIONS FULL;
DEFINE FIELD likes_count ON comment_likes_count TYPE int DEFAULT 0 PERMISSIONS FULL;
DEFINE INDEX comment_likes_count_index ON comment_likes_count FIELDS comment_id UNIQUE;

DEFINE EVENT increment_like_count ON likes
WHEN $before == NONE AND $after != NONE THEN {
    LET $item = $after.out;

    IF $item == NONE {
        THROW "Invalid item";
    };

    LET $table_name = meta::table($item);
    LET $likes_table = string::join('_', $table_name, 'likes_count');
    LET $id_field = string::join('_', $table_name, 'id');
    LET $likes_record = (SELECT * FROM type::table($likes_table) WHERE type::field($id_field) == $item)[0];
    LET $current_count = $likes_record.likes_count ?? 0;

    IF $current_count == 0 {
        IF $table_name == 'post' {
            INSERT INTO post_likes_count (post_id, likes_count) VALUES ($item, 1) ON DUPLICATE KEY UPDATE likes_count += 1;
        } ELSE IF $table_name == 'comment' {
            INSERT INTO comment_likes_count (comment_id, likes_count) VALUES ($item, 1)  ON DUPLICATE KEY UPDATE likes_count += 1;
        }
    } ELSE {
        IF $table_name == 'post' {
            UPDATE post_likes_count SET likes_count = $current_count + 1 WHERE post_id == $item;
        } ELSE IF $table_name == 'comment' {
            UPDATE comment_likes_count SET likes_count = $current_count + 1 WHERE comment_id == $item;
        }
    }
};

DEFINE EVENT decrement_like_count ON likes
WHEN $before != NONE AND $after == NONE THEN {
    LET $item = $before.out;

    IF $item == NONE {
        THROW "Invalid item";
    };

    LET $table_name = meta::table($item);
    LET $likes_table = string::join('_', $table_name, 'likes_count');
    LET $id_field = string::join('_', $table_name, 'id');
    LET $current_count = (SELECT likes_count FROM type::table($likes_table) WHERE type::field($id_field) == $item)[0].likes_count ?? 0;

    IF $current_count > 1 {
        IF $table_name == 'post' {
            UPDATE post_likes_count SET likes_count = $current_count - 1 WHERE post_id == $item;
        } ELSE IF $table_name == 'comment' {
            UPDATE comment_likes_count SET likes_count = $current_count - 1 WHERE comment_id == $item;
        }
    } ELSE IF $current_count == 1 {
        IF $table_name == 'post' {
            UPDATE post_likes_count SET likes_count = 0 WHERE post_id == $item;
        } ELSE IF $table_name == 'comment' {
            UPDATE comment_likes_count SET likes_count = 0 WHERE comment_id == $item;
        }
    }
};