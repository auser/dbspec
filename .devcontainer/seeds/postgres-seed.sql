-- Create the database
-- Note: Creation of the database cannot be included in a transactional SQL script.
-- You should run this command separately in your PostgreSQL management tool.
-- CREATE DATABASE production;

-- Connect to the database
-- \c CompanyDB;

-- Creating the Users table
CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Creating the Groups table
CREATE TABLE IF NOT EXISTS Groups (
    id SERIAL PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL,
    description TEXT
);

-- Creating the Teams table
CREATE TABLE IF NOT EXISTS Teams (
    id SERIAL PRIMARY KEY,
    team_name VARCHAR(255) NOT NULL,
    description TEXT
);

-- Creating the Transactions table
CREATE TABLE IF NOT EXISTS Transactions (
    id SERIAL PRIMARY KEY,
    user_id INT,
    amount NUMERIC(10, 2),
    transaction_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

-- Creating a Users_Groups junction table for many-to-many relationship between Users and Groups
CREATE TABLE IF NOT EXISTS Users_Groups (
    user_id INT,
    group_id INT,
    PRIMARY KEY (user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES Groups(id) ON DELETE CASCADE
);

-- Creating a Users_Teams junction table for many-to-many relationship between Users and Teams
CREATE TABLE IF NOT EXISTS Users_Teams (
    user_id INT,
    team_id INT,
    PRIMARY KEY (user_id, team_id),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES Teams(id) ON DELETE CASCADE
);

-- Seed data for Users
INSERT INTO Users (username, email) VALUES
('johndoe', 'john.doe@example.com'),
('janedoe', 'jane.doe@example.com')
ON CONFLICT (email) DO NOTHING;

-- Seed data for Groups
INSERT INTO Groups (group_name, description) VALUES
('Admins', 'System administrators group.'),
('Support', 'Customer support group.')
ON CONFLICT (group_name) DO NOTHING;

-- Seed data for Teams
INSERT INTO Teams (team_name, description) VALUES
('Development', 'Software development team.'),
('Sales', 'Sales and Marketing team.')
ON CONFLICT (team_name) DO NOTHING;

-- Seed data for Transactions
INSERT INTO Transactions (user_id, amount) VALUES
(1, 100.50),
(2, 200.75);

-- Seed data for Users_Groups
INSERT INTO Users_Groups (user_id, group_id) VALUES
(1, 1),
(2, 2)
ON CONFLICT DO NOTHING;

-- Seed data for Users_Teams
INSERT INTO Users_Teams (user_id, team_id) VALUES
(1, 1),
(2, 2)
ON CONFLICT DO NOTHING;
