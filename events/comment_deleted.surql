DEFINE EVENT comment_deleted ON TABLE comment
    WHEN $before != NONE AND $after == NONE
    THEN {
        LET $comment_id = $before.id;
        LET $post_id = $before.post_id;
        IF $post_id != NONE AND $comment_id != NONE {
            DELETE $before.user_id->commented WHERE out = $comment_id;
            DELETE $comment_id->comment_of WHERE out = $post_id;
            DELETE likes WHERE out = $comment_id;
            DELETE comment_likes_count WHERE comment_id == $comment_id;
        }
    };