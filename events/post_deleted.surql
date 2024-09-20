DEFINE EVENT post_deleted ON TABLE post
    WHEN $before != NONE AND $after == NONE
    THEN {
        LET $post_id = $before.id;
        IF $post_id != NONE {
            DELETE comment WHERE post_id == $post_id;
            DELETE $before.user_id->authored WHERE out = $post_id;
            DELETE likes WHERE out = $post_id;
            DELETE post_likes_count WHERE post_id == $post_id;
        }
    };