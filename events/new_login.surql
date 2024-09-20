DEFINE EVENT IF NOT EXISTS new_login ON TABLE session
    WHEN $before == NONE
    THEN {
        LET $user = (SELECT * FROM user WHERE id == $after.user_id);

        DELETE session WHERE expires_at < time::now();

        UPDATE $user MERGE {
            last_login: time::now()
        };
    };