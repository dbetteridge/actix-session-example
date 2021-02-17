INSERT INTO testing.users(email, name, username, password)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;