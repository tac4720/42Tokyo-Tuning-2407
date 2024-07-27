-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX index_username(username);
ALTER TABLE users ADD INDEX index_id(id);
ALTER TABLE users ADD INDEX index_profile_image(profile_image);
ALTER TABLE sessions ADD INDEX index_session_token(session_token);
ALTER TABLE sessions ADD INDEX index_session_user_id(user_id);
ALTER TABLE sessions ADD INDEX index_session_is_valid(is_valid);
ALTER TABLE locations ADD INDEX index_location_id(id);
ALTER TABLE nodes ADD INDEX index_name(name);
ALTER TABLE nodes ADD INDEX area_id(area_id);
ALTER TABLE tow_trucks ADD INDEX index_user_id(user_id);
ALTER TABLE tow_trucks ADD INDEX index_status(status);
ALTER TABLE edges ADD INDEX index_weight(weight);
