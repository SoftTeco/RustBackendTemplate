-- Migration to add a unique constraint and modify foreign key constraints in user_roles table

-- Step 1: Drop existing foreign key constraints
ALTER TABLE user_roles
DROP CONSTRAINT user_roles_user_id_fkey,
DROP CONSTRAINT user_roles_role_id_fkey;

-- Step 2: Add the unique constraint
ALTER TABLE user_roles
ADD CONSTRAINT unique_user_role UNIQUE (user_id, role_id);

-- Step 3: Add foreign key constraints with ON DELETE CASCADE
ALTER TABLE user_roles
ADD CONSTRAINT user_roles_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
ADD CONSTRAINT user_roles_role_id_fkey FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE;