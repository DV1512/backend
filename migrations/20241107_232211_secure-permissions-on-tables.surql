ALTER TABLE user PERMISSIONS FOR select, update WHERE id = $auth.id, FOR create NONE, FOR delete NONE;
ALTER TABLE session PERMISSIONS FOR select WHERE user_id = $auth.id, FOR create NONE, FOR update NONE, FOR delete NONE;
ALTER TABLE session_of PERMISSIONS FOR select WHERE in.id = $auth.id, FOR create NONE, FOR update NONE, FOR delete NONE;
ALTER TABLE provider PERMISSIONS FOR select FULL, FOR create NONE, FOR update NONE, FOR delete NONE;
ALTER TABLE provider_config PERMISSIONS FOR select FULL, FOR create NONE, FOR update NONE, FOR delete NONE;


