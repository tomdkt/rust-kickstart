-- Add created_at column to users table
ALTER TABLE users 
ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Create index for efficient pagination
CREATE INDEX idx_users_created_at_id ON users (created_at, id);