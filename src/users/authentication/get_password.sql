SELECT password FROM users WHERE username = (username)
VALUES ($1)
RETURNING $table_fields;
