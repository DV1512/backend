DEFINE EVENT user_deleted ON TABLE user
    WHEN $before != NONE AND $after == NONE
    THEN {
        LET $user_id = $before.id;

        -- Delete posts authored by the user
        DELETE post WHERE user_id == $user_id;

        -- Delete comments made by the user
        DELETE comment WHERE user_id == $user_id;

        -- Delete sessions associated with the user
        DELETE session WHERE user_id == $user_id;

        -- Remove all authored, commented, and session_of relations
        DELETE $user_id->authored WHERE out != NONE;
        DELETE $user_id->commented WHERE out != NONE;
        DELETE $user_id->session_of WHERE out != NONE;
        DELETE $user_id->liked WHERE out != NONE;
        DELETE $user_id->auth_for WHERE out != NONE;
    };