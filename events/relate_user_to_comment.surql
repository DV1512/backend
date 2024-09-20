DEFINE EVENT relate_user_to_comment ON TABLE comment
    WHEN $before == NONE
    THEN {
        LET $from = (SELECT * FROM user WHERE id == $after.user_id)[0];
        LET $to = (SELECT * FROM comment WHERE id == $after.id)[0];

        RELATE $from->commented->$to;
    };