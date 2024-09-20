DEFINE FUNCTION IF NOT EXISTS fn::likes_item($user: record<user>, $item: record<post | comment>) {
	LET $liked_items = (
            SELECT * FROM likes
            WHERE in = $user
            AND out = $item
        );

	RETURN array::len($liked_items) != 0;
} PERMISSIONS FULL COMMENT "Returns True if the user has already liked this item or returns False if the user has not";