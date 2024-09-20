DEFINE FUNCTION IF NOT EXISTS fn::number_of_likes($item: record<post | comment>) {
    RETURN ((SELECT count() FROM likes WHERE out == $item GROUP ALL)[0].count ?? 0);
} PERMISSIONS FULL;
