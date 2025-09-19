-- Find user by ID
SELECT id, name, age, created_at FROM users WHERE id = $1