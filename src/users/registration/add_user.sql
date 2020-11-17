INSERT INTO testing.users(username, email_id, password)
VALUES ($1, $2, $3)
RETURNING $table_fields;
