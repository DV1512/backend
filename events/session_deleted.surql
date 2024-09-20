DEFINE EVENT session_deleted ON TABLE session
    WHEN $before != NONE AND $after == NONE
    THEN {
        LET $session_id = $before.id;
        LET $user_id = $before.user_id;

        -- Remove session_of relation between user and session
        DELETE $user_id->session_of WHERE out = $session_id;
    };