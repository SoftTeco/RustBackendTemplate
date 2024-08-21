-- Migration to remove unique constraint and modify foreign key constraints in user_roles table

-- Step 1: Drop existing foreign key constraints
ALTER TABLE user_roles
DROP CONSTRAINT user_roles_user_id_fkey,
DROP CONSTRAINT user_roles_role_id_fkey;

-- Step 2: Drop the unique constraint
ALTER TABLE user_roles
DROP CONSTRAINT unique_user_role;

-- Step 3: Add foreign key constraints without ON DELETE CASCADE
ALTER TABLE user_roles
ADD CONSTRAINT user_roles_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id),
ADD CONSTRAINT user_roles_role_id_fkey FOREIGN KEY (role_id) REFERENCES roles(id);