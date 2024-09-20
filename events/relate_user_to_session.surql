DEFINE EVENT IF NOT EXISTS relate_user_to_session ON TABLE session
    WHEN $before == NONE
    THEN {
        LET $from = (SELECT * FROM user WHERE id == $after.user_id)[0];
        LET $to = (SELECT * FROM session WHERE id == $after.id)[0];

        RELATE $from->session_of->$to;
    };