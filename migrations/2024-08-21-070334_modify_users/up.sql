ALTER TABLE users
ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL,
ADD COLUMN user_type VARCHAR(24) NOT NULL DEFAULT 'regular';

SELECT diesel_manage_updated_at('users');