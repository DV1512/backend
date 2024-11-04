DEFINE EVENT IF NOT EXISTS user_deleted ON TABLE user
    WHEN $before != NONE AND $after == NONE
    THEN {
        LET $user_id = $before.id;

        -- Delete sessions associated with the user
        DELETE session WHERE user_id == $user_id;

        -- Remove all authored, commented, and session_of relations
        DELETE $user_id->session_of WHERE out != NONE;
        DELETE $user_id->auth_for WHERE out != NONE;
    };