DEFINE EVENT post_updated ON TABLE post
    WHEN $before.title != $after.title OR $before.content != $after.content AND $after != NONE AND $before != NONE AND $event = "UPDATE"
    THEN {
        UPDATE $after MERGE {
            updated: time::now()
        };
    };