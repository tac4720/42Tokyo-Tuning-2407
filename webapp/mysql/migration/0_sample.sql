-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users ADD INDEX index_username(username);

ALTER TABLE orders ADD INDEX index_status(status);
ALTER TABLE orders ADD INDEX index_node_id(node_id);
ALTER TABLE orders ADD INDEX index_order_time(order_time);
ALTER TABLE orders ADD INDEX index_client_id(client_id);
