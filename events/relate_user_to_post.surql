DEFINE EVENT relate_user_to_post ON TABLE post
    WHEN $before == NONE
    THEN {
        LET $from = (SELECT * FROM user WHERE id == $after.user_id)[0];

        RELATE $from->authored->$after;
    };