DEFINE EVENT IF NOT EXISTS relate_comment_to_post ON TABLE comment
    WHEN $before == NONE
    THEN {
        LET $from = (SELECT * FROM comment WHERE id == $after.id)[0];
        LET $to = (SELECT * FROM post WHERE id == $after.post_id)[0];

        RELATE $from->comment_of->$to;
    };